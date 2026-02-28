import { createI18n } from 'vue-i18n'
import { targetLang } from './composables/useLanguage.js'

const modules = import.meta.glob('./locales/*.json', { eager: true })

const messages = Object.fromEntries(
  Object.entries(modules).map(([path, mod]) => {
    const lang = path.match(/\/(\w+)\.json$/)[1]
    return [lang, mod.default ?? mod]
  })
)

export const i18n = createI18n({
  legacy: false,
  locale: targetLang.value.toLowerCase(),
  fallbackLocale: 'en',
  messages,
})
