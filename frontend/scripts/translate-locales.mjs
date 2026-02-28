import { readFileSync, writeFileSync } from 'fs'
import { resolve, dirname } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))

const DEEPL_API_KEY = process.env.DEEPL_API_KEY
if (!DEEPL_API_KEY) {
  console.error('Error: DEEPL_API_KEY environment variable not set')
  process.exit(1)
}

const TARGET_LANGS = ['DE', 'FR', 'ES', 'IT', 'RU', 'JA', 'ZH', 'PT']

const en = JSON.parse(readFileSync(resolve(__dirname, '../src/locales/en.json'), 'utf8'))
const keys = Object.keys(en)
const texts = Object.values(en)

async function translateBatch(texts, targetLang) {
  const res = await fetch('https://api-free.deepl.com/v2/translate', {
    method: 'POST',
    headers: {
      'Authorization': `DeepL-Auth-Key ${DEEPL_API_KEY}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ text: texts, source_lang: 'EN', target_lang: targetLang }),
  })
  if (!res.ok) throw new Error(`DeepL error ${res.status}: ${await res.text()}`)
  const data = await res.json()
  return data.translations.map(t => t.text)
}

for (const lang of TARGET_LANGS) {
  process.stdout.write(`Translating ${lang}… `)
  try {
    const translated = await translateBatch(texts, lang)
    const result = Object.fromEntries(keys.map((k, i) => [k, translated[i]]))
    const outPath = resolve(__dirname, `../src/locales/${lang.toLowerCase()}.json`)
    writeFileSync(outPath, JSON.stringify(result, null, 2) + '\n')
    console.log('done')
  } catch (err) {
    console.error(`failed: ${err.message}`)
  }
}
