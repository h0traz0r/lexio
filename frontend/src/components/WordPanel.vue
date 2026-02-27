<script setup>
defineProps({ word: String, translation: String, loading: Boolean })
defineEmits(['close'])
</script>

<template>
  <aside class="w-64 shrink-0 border-l border-border flex flex-col gap-4 p-6">
    <div class="flex items-start justify-between gap-2">
      <span class="text-xl font-semibold break-all">{{ word }}</span>
      <button
        class="text-muted-foreground hover:text-foreground mt-0.5 shrink-0"
        @click="$emit('close')"
      >✕</button>
    </div>

    <div class="text-sm text-muted-foreground uppercase tracking-wide">Translation</div>

    <div v-if="loading" class="text-muted-foreground animate-pulse">Translating…</div>
    <div v-else-if="translation" class="text-base">{{ translation }}</div>
    <div v-else class="text-muted-foreground text-sm italic">No translation found.</div>
  </aside>
</template>
