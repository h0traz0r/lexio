<script setup>
import { BookOpen } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
} from '@/components/ui/card'
</script>

<template>
  <div class="min-h-screen bg-background">
    <!-- Header -->
    <header class="border-b border-border">
      <div class="max-w-5xl mx-auto px-6 py-4 flex items-center gap-3">
        <BookOpen class="size-6 text-primary" />
        <span class="text-xl font-semibold tracking-tight">Lexio</span>
        <Badge variant="secondary" class="ml-auto">Alpha</Badge>
      </div>
    </header>

    <!-- Hero -->
    <main class="max-w-5xl mx-auto px-6 py-20 text-center">
      <h1 class="text-4xl font-bold tracking-tight mb-4">
        Read German. Learn faster.
      </h1>
      <p class="text-muted-foreground text-lg mb-10 max-w-xl mx-auto">
        Upload any text or PDF and read it word by word — with translations, progress tracking, and more coming soon.
      </p>
      <div class="flex items-center justify-center gap-3">
        <Button size="lg">Open Library</Button>
        <Button size="lg" variant="outline">Upload a Book</Button>
      </div>
    </main>

    <!-- Features -->
    <section class="max-w-5xl mx-auto px-6 pb-20 grid sm:grid-cols-3 gap-4">
      <Card>
        <CardHeader>
          <CardTitle class="text-base">Upload Text & PDF</CardTitle>
          <CardDescription>Drop in any .txt or .pdf file and start reading instantly.</CardDescription>
        </CardHeader>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle class="text-base">Paginated Reader</CardTitle>
          <CardDescription>Books are split into readable pages so you can track your progress.</CardDescription>
        </CardHeader>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle class="text-base">Translations Soon</CardTitle>
          <CardDescription>Click any word to get an instant DeepL translation inline.</CardDescription>
        </CardHeader>
      </Card>
    </section>
  </div>
</template>
