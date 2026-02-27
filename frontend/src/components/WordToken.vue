<script setup>
defineProps({ word: String, active: Boolean })
defineEmits(['select'])
</script>

<template>
  <span
    class="word-token cursor-pointer rounded px-0.5 transition-colors hover:bg-muted"
    :class="active && 'bg-primary/15 text-primary'"
    @click="$emit('select', word, $event)"
  >{{ word }}</span>
</template>
