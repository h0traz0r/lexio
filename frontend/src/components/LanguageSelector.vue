<script setup>
import { useLanguage, LANGUAGES } from '@/composables/useLanguage.js'

const { targetLang } = useLanguage()
</script>

<template>
  <div class="flex items-center gap-1">
    <button
      v-for="lang in LANGUAGES"
      :key="lang.code"
      :title="lang.label"
      class="text-lg leading-none px-1 py-0.5 rounded transition-colors"
      :class="targetLang === lang.code ? 'bg-primary/15 ring-1 ring-primary/40' : 'opacity-50 hover:opacity-100'"
      @click="targetLang = lang.code"
    >
      {{ lang.flag }}
    </button>
  </div>
</template>
