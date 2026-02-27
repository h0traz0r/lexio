const BASE = import.meta.env.VITE_API_BASE || 'http://localhost:3000'

export async function uploadPdf(file, title) {
  const form = new FormData()
  form.append('title', title)
  form.append('file', file)
  const res = await fetch(`${BASE}/api/upload`, { method: 'POST', body: form })
  if (!res.ok) throw new Error(await res.text())
  return res.json() // { title, pages: string[] }
}

export async function translateWord(word, targetLang = 'EN') {
  const res = await fetch(`${BASE}/api/translate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ word, target_lang: targetLang }),
  })
  if (!res.ok) throw new Error(await res.text())
  const data = await res.json()
  return data.translation
}

export async function gutenbergSearch(query, lang = 'de') {
  const res = await fetch(`${BASE}/api/gutenberg/search?q=${encodeURIComponent(query)}&lang=${lang}`)
  if (!res.ok) throw new Error(await res.text())
  return res.json() // BookSummary[]
}

export async function gutenbergImport(id) {
  const res = await fetch(`${BASE}/api/gutenberg/import`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ id }),
  })
  if (!res.ok) throw new Error(await res.text())
  return res.json() // { title, pages: string[] }
}
