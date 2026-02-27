<script setup>
import { ref, onMounted, onBeforeUnmount, watch, nextTick } from 'vue'

const props = defineProps({
  word: String,
  translation: String,
  loading: Boolean,
  anchorRect: Object, // { top, left, width, height } from getBoundingClientRect
})
const emit = defineEmits(['close'])

const bubble = ref(null)
const pos = ref({ top: 0, left: 0 })

function reposition() {
  if (!props.anchorRect || !bubble.value) return
  const el = bubble.value
  const r = props.anchorRect
  const bw = el.offsetWidth
  const bh = el.offsetHeight

  // center horizontally above the word
  let left = r.left + r.width / 2 - bw / 2
  let top = r.top - bh - 10

  // keep within viewport
  if (left < 8) left = 8
  if (left + bw > window.innerWidth - 8) left = window.innerWidth - bw - 8
  if (top < 8) top = r.top + r.height + 10 // flip below if no room above

  pos.value = { top, left }
}

watch(() => props.anchorRect, async () => {
  await nextTick()
  reposition()
}, { immediate: true })

onMounted(() => {
  nextTick(reposition)
  window.addEventListener('resize', reposition)
})
onBeforeUnmount(() => {
  window.removeEventListener('resize', reposition)
})

function onClickOutside(e) {
  if (bubble.value && !bubble.value.contains(e.target) && !e.target.closest('.word-token')) {
    emit('close')
  }
}
onMounted(() => document.addEventListener('mousedown', onClickOutside))
onBeforeUnmount(() => document.removeEventListener('mousedown', onClickOutside))
</script>

<template>
  <div
    ref="bubble"
    class="fixed z-50 bg-popover border border-border rounded-xl shadow-lg px-4 py-3 min-w-[120px] max-w-[280px] transition-opacity duration-150"
    :style="{ top: pos.top + 'px', left: pos.left + 'px' }"
  >
    <!-- Arrow -->
    <div
      v-if="pos.top < (anchorRect?.top ?? 0)"
      class="absolute -bottom-[6px] left-1/2 -translate-x-1/2 w-3 h-3 rotate-45 bg-popover border-r border-b border-border"
    />
    <div
      v-else
      class="absolute -top-[6px] left-1/2 -translate-x-1/2 w-3 h-3 rotate-45 bg-popover border-l border-t border-border"
    />

    <div class="text-xs text-muted-foreground uppercase tracking-wide mb-1">{{ word }}</div>

    <div v-if="loading" class="text-sm text-muted-foreground animate-pulse">Translating…</div>
    <div v-else-if="translation" class="text-sm font-medium">{{ translation }}</div>
    <div v-else class="text-sm text-muted-foreground italic">No translation found.</div>
  </div>
</template>
