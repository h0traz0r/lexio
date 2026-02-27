<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ChevronLeft, ChevronRight } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import WordToken from '@/components/WordToken.vue'
import TranslationBubble from '@/components/TranslationBubble.vue'
import { translateWord } from '@/lib/api.js'
import { useBook } from '@/composables/useBook.js'
import { useLanguage } from '@/composables/useLanguage.js'

const router = useRouter()
const { book } = useBook()
const { targetLang } = useLanguage()

const currentPage = ref(0)
const selectedWord = ref(null)
const translation = ref(null)
const translating = ref(false)
const anchorRect = ref(null)

// Redirect if no book loaded
onMounted(() => {
  if (!book.value) router.replace('/library')
})

const pageText = computed(() => book.value?.pages[currentPage.value] ?? '')
const totalPages = computed(() => book.value?.pages.length ?? 0)

function tokenize(text) {
  return text.split(/([A-Za-zÄÖÜäöüß]+)/).filter(t => t !== '')
}
function isWord(token) {
  return /^[A-Za-zÄÖÜäöüß]+$/.test(token)
}

async function selectWord(word, event) {
  if (selectedWord.value === word) { closeBubble(); return }

  // Get position of the clicked word
  const el = event.target
  anchorRect.value = el.getBoundingClientRect()

  selectedWord.value = word
  translation.value = null
  translating.value = true
  try {
    translation.value = await translateWord(word, targetLang.value)
  } finally {
    translating.value = false
  }
}

function closeBubble() {
  selectedWord.value = null
  translation.value = null
  anchorRect.value = null
}

function prevPage() {
  if (currentPage.value > 0) { currentPage.value--; closeBubble() }
}
function nextPage() {
  if (currentPage.value < totalPages.value - 1) { currentPage.value++; closeBubble() }
}
</script>

<template>
  <div v-if="book" class="flex-1 flex flex-col overflow-hidden relative">

    <!-- Title bar -->
    <div class="border-b border-border px-8 py-3 text-center shrink-0">
      <span class="font-medium">{{ book.title }}</span>
    </div>

    <!-- Centered text area -->
    <div class="flex-1 overflow-y-auto px-6 py-10">
      <p class="max-w-2xl mx-auto text-base leading-loose">
        <template v-for="(token, i) in tokenize(pageText)" :key="i">
          <WordToken
            v-if="isWord(token)"
            :word="token"
            :active="token === selectedWord"
            @select="selectWord"
          />
          <span v-else>{{ token }}</span>
        </template>
      </p>
    </div>

    <!-- Bottom-center pagination -->
    <div class="shrink-0 border-t border-border py-3 flex items-center justify-center gap-4">
      <Button variant="ghost" size="icon-sm" :disabled="currentPage === 0" @click="prevPage">
        <ChevronLeft class="size-4" />
      </Button>
      <span class="text-sm text-muted-foreground tabular-nums">
        {{ currentPage + 1 }} / {{ totalPages }}
      </span>
      <Button variant="ghost" size="icon-sm" :disabled="currentPage === totalPages - 1" @click="nextPage">
        <ChevronRight class="size-4" />
      </Button>
    </div>

    <!-- Translation bubble -->
    <TranslationBubble
      v-if="selectedWord"
      :word="selectedWord"
      :translation="translation"
      :loading="translating"
      :anchor-rect="anchorRect"
      @close="closeBubble"
    />
  </div>
</template>
