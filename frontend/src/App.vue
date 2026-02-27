<script setup>
import { BookOpen } from 'lucide-vue-next'
import { Badge } from '@/components/ui/badge'
import LanguageSelector from '@/components/LanguageSelector.vue'
</script>

<template>
  <div class="min-h-screen bg-background flex flex-col">
    <header class="border-b border-border shrink-0">
      <div class="max-w-6xl mx-auto px-6 py-4 flex items-center gap-3">
        <BookOpen class="size-5 text-primary" />
        <RouterLink to="/" class="text-lg font-semibold tracking-tight hover:opacity-80">Lexio</RouterLink>
        <Badge variant="secondary" class="ml-0">Alpha</Badge>
        <nav class="ml-auto flex items-center gap-4 text-sm">
          <LanguageSelector />
          <RouterLink to="/" class="text-muted-foreground hover:text-foreground transition-colors">Home</RouterLink>
          <RouterLink to="/library" class="text-muted-foreground hover:text-foreground transition-colors">Library</RouterLink>
        </nav>
      </div>
    </header>

    <RouterView />
  </div>
</template>
