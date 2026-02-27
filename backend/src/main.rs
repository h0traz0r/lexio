use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, Query},
    http::Method,
    routing::{get, post},
    Json,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

const WORDS_PER_PAGE: usize = 300;

// ── Upload ────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct UploadResponse {
    title: String,
    pages: Vec<String>,
}

async fn upload(mut multipart: Multipart) -> Result<Json<UploadResponse>, String> {
    let mut title = String::from("Untitled");
    let mut pages: Vec<String> = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "title" => {
                title = field.text().await.map_err(|e| e.to_string())?;
            }
            "file" => {
                let bytes = field.bytes().await.map_err(|e| e.to_string())?;
                let text = pdf_extract::extract_text_from_mem(&bytes)
                    .map_err(|e| e.to_string())?;
                pages = split_into_pages(&text);
            }
            _ => {}
        }
    }

    Ok(Json(UploadResponse { title, pages }))
}

fn split_into_pages(text: &str) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    words
        .chunks(WORDS_PER_PAGE)
        .map(|chunk| chunk.join(" "))
        .collect()
}

// ── Translate ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct TranslateRequest {
    word: String,
}

#[derive(Serialize)]
struct TranslateResponse {
    translation: String,
}

#[derive(Deserialize)]
struct DeepLResponse {
    translations: Vec<DeepLTranslation>,
}

#[derive(Deserialize)]
struct DeepLTranslation {
    text: String,
}

async fn translate(
    Json(body): Json<TranslateRequest>,
) -> Result<Json<TranslateResponse>, String> {
    let api_key = std::env::var("DEEPL_API_KEY")
        .map_err(|_| "DEEPL_API_KEY not set".to_string())?;

    let client = reqwest::Client::new();
    let res: DeepLResponse = client
        .post("https://api-free.deepl.com/v2/translate")
        .header("Authorization", format!("DeepL-Auth-Key {api_key}"))
        .json(&serde_json::json!({
            "text": [body.word],
            "source_lang": "DE",
            "target_lang": "EN"
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let translation = res
        .translations
        .into_iter()
        .next()
        .map(|t| t.text)
        .unwrap_or_default();

    Ok(Json(TranslateResponse { translation }))
}

// ── Gutenberg ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct GutenbergSearchQuery {
    q: String,
}

#[derive(Deserialize)]
struct GutendexResponse {
    results: Vec<GutendexBook>,
}

#[derive(Deserialize)]
struct GutendexBook {
    id: u64,
    title: String,
    authors: Vec<GutendexAuthor>,
    formats: std::collections::HashMap<String, String>,
    download_count: u64,
}

#[derive(Deserialize)]
struct GutendexAuthor {
    name: String,
}

#[derive(Serialize)]
struct BookSummary {
    id: u64,
    title: String,
    authors: Vec<String>,
    download_count: u64,
}

async fn gutenberg_search(
    Query(params): Query<GutenbergSearchQuery>,
) -> Result<Json<Vec<BookSummary>>, String> {
    let client = reqwest::Client::new();
    let res: GutendexResponse = client
        .get("https://gutendex.com/books/")
        .query(&[("languages", "de"), ("search", &params.q)])
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let books = res
        .results
        .into_iter()
        .map(|b| BookSummary {
            id: b.id,
            title: b.title,
            authors: b.authors.into_iter().map(|a| a.name).collect(),
            download_count: b.download_count,
        })
        .collect();

    Ok(Json(books))
}

#[derive(Deserialize)]
struct GutenbergImportRequest {
    id: u64,
}

async fn gutenberg_import(
    Json(body): Json<GutenbergImportRequest>,
) -> Result<Json<UploadResponse>, String> {
    let client = reqwest::Client::new();

    // Fetch book metadata from Gutendex
    let url = format!("https://gutendex.com/books/{}", body.id);
    let book: GutendexBook = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    // Find the plain text URL
    let text_url = book
        .formats
        .get("text/plain; charset=utf-8")
        .or_else(|| book.formats.get("text/plain"))
        .ok_or_else(|| "No plain text format available for this book".to_string())?
        .clone();

    // Fetch the text
    let text = client
        .get(&text_url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let pages = split_into_pages(&text);

    Ok(Json(UploadResponse {
        title: book.title,
        pages,
    }))
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/upload", post(upload))
        .route("/api/translate", post(translate))
        .route("/api/gutenberg/search", get(gutenberg_search))
        .route("/api/gutenberg/import", post(gutenberg_import))
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100 MB
        .layer(cors);

    let port = std::env::var("BACKEND_PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("0.0.0.0:{port}");
    println!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
