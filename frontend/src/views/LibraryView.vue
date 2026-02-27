<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Search, Loader2, BookOpen } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { gutenbergSearch, gutenbergImport } from '@/lib/api.js'
import { useBook } from '@/composables/useBook.js'

const router = useRouter()
const { setBook } = useBook()

const query = ref('')
const lang = ref('de')
const books = ref([])
const loading = ref(false)
const importing = ref(null) // book id being imported
const error = ref(null)

const SEARCH_LANGS = [
  { code: 'de', flag: '🇩🇪', label: 'German' },
  { code: 'en', flag: '🇬🇧', label: 'English' },
  { code: 'fr', flag: '🇫🇷', label: 'French' },
  { code: 'es', flag: '🇪🇸', label: 'Spanish' },
  { code: 'it', flag: '🇮🇹', label: 'Italian' },
  { code: 'ru', flag: '🇷🇺', label: 'Russian' },
  { code: 'pt', flag: '🇵🇹', label: 'Portuguese' },
]

let debounceTimer = null

async function search() {
  loading.value = true
  error.value = null
  try {
    books.value = await gutenbergSearch(query.value, lang.value)
  } catch (err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}

function onInput() {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => search(), 400)
}

function setLang(code) {
  lang.value = code
  search()
}
async function importBook(book) {
  importing.value = book.id
  error.value = null
  try {
    setBook(await gutenbergImport(book.id))
    router.push('/reader')
  } catch (err) {
    error.value = err.message
    importing.value = null
  }
}

onMounted(() => search())
</script>

<template>
  <main class="flex-1 flex flex-col">
    <div class="max-w-4xl mx-auto w-full px-6 py-8 flex flex-col gap-6">
      <!-- Language picker -->
      <div class="flex items-center gap-1">
        <button
          v-for="l in SEARCH_LANGS"
          :key="l.code"
          :title="l.label"
          class="text-lg leading-none px-1.5 py-0.5 rounded transition-colors"
          :class="lang === l.code ? 'bg-primary/15 ring-1 ring-primary/40' : 'opacity-50 hover:opacity-100'"
          @click="setLang(l.code)"
        >{{ l.flag }}</button>
      </div>

      <!-- Search -->
      <div class="relative">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground" />
        <input
          v-model="query"
          @input="onInput"
          type="text"
          :placeholder="`Search ${SEARCH_LANGS.find(l => l.code === lang)?.label} books…`"
          class="w-full pl-10 pr-4 py-2.5 rounded-lg border border-input bg-background text-sm focus:outline-none focus:ring-2 focus:ring-ring"
        />
      </div>

      <!-- Error -->
      <p v-if="error" class="text-destructive text-sm">{{ error }}</p>

      <!-- Loading -->
      <div v-if="loading" class="flex-1 flex items-center justify-center py-20">
        <Loader2 class="size-6 animate-spin text-muted-foreground" />
      </div>

      <!-- Results -->
      <div v-else-if="books.length" class="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">
        <button
          v-for="book in books"
          :key="book.id"
          :disabled="importing === book.id"
          class="text-left border border-border rounded-lg p-4 hover:border-primary/40 hover:bg-muted/50 transition-colors flex flex-col gap-2 disabled:opacity-60"
          @click="importBook(book)"
        >
          <div class="flex items-start gap-2">
            <BookOpen class="size-4 text-muted-foreground shrink-0 mt-0.5" />
            <span class="font-medium text-sm leading-snug line-clamp-2">{{ book.title }}</span>
          </div>
          <p class="text-xs text-muted-foreground line-clamp-1">
            {{ book.authors.join(', ') || 'Unknown author' }}
          </p>
          <p class="text-xs text-muted-foreground mt-auto">
            {{ importing === book.id ? 'Importing…' : `${book.download_count.toLocaleString()} downloads` }}
          </p>
        </button>
      </div>

      <!-- Empty -->
      <div v-else class="flex-1 flex flex-col items-center justify-center py-20 text-muted-foreground text-sm">
        <p>No books found. Try a different search term.</p>
      </div>
    </div>
  </main>
</template>
