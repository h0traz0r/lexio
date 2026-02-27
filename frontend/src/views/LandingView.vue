<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Upload, BookOpen, Languages, Library } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { uploadPdf } from '@/lib/api.js'
import { useBook } from '@/composables/useBook.js'

const router = useRouter()
const { setBook } = useBook()

const uploading = ref(false)
const uploadError = ref(null)
const fileInput = ref(null)

async function onFileChange(e) {
  const file = e.target.files?.[0]
  if (!file) return
  uploading.value = true
  uploadError.value = null
  try {
    const title = file.name.replace(/\.[^.]+$/, '')
    setBook(await uploadPdf(file, title))
    router.push('/reader')
  } catch (err) {
    uploadError.value = err.message
  } finally {
    uploading.value = false
    e.target.value = ''
  }
}
</script>

<template>
  <main class="flex-1 flex flex-col">
    <!-- Hero -->
    <section class="flex-1 flex flex-col items-center justify-center gap-6 px-6 text-center py-20">
      <h1 class="text-4xl sm:text-5xl font-bold tracking-tight max-w-xl leading-tight">
        Read German. Learn naturally.
      </h1>
      <p class="text-muted-foreground max-w-md text-lg">
        Browse classic German literature or upload your own PDF. Click any word for an instant translation.
      </p>
      <div class="flex flex-wrap gap-3 justify-center mt-2">
        <Button size="lg" as-child>
          <RouterLink to="/library">
            <Library class="size-4 mr-2" />
            Browse Library
          </RouterLink>
        </Button>
        <Button size="lg" variant="outline" :disabled="uploading" @click="fileInput.click()">
          <Upload class="size-4 mr-2" />
          {{ uploading ? 'Uploading…' : 'Upload PDF' }}
        </Button>
      </div>
      <p v-if="uploadError" class="text-destructive text-sm max-w-sm">{{ uploadError }}</p>
      <input ref="fileInput" type="file" accept=".pdf" class="hidden" @change="onFileChange" />
    </section>

    <!-- Features -->
    <section class="border-t border-border bg-muted/30">
      <div class="max-w-4xl mx-auto px-6 py-16 grid sm:grid-cols-3 gap-8">
        <div class="flex flex-col items-center text-center gap-3">
          <div class="size-10 rounded-lg bg-primary/10 flex items-center justify-center">
            <Library class="size-5 text-primary" />
          </div>
          <h3 class="font-semibold">Gutenberg Library</h3>
          <p class="text-sm text-muted-foreground">Thousands of free German classics from Project Gutenberg, ready to read.</p>
        </div>
        <div class="flex flex-col items-center text-center gap-3">
          <div class="size-10 rounded-lg bg-primary/10 flex items-center justify-center">
            <BookOpen class="size-5 text-primary" />
          </div>
          <h3 class="font-semibold">Immersive Reader</h3>
          <p class="text-sm text-muted-foreground">Paginated text with clickable words. Read at your own pace.</p>
        </div>
        <div class="flex flex-col items-center text-center gap-3">
          <div class="size-10 rounded-lg bg-primary/10 flex items-center justify-center">
            <Languages class="size-5 text-primary" />
          </div>
          <h3 class="font-semibold">Instant Translation</h3>
          <p class="text-sm text-muted-foreground">Click any word for a DeepL-powered translation. No copy-pasting.</p>
        </div>
      </div>
    </section>
  </main>
</template>
