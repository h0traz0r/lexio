import { ref, watch } from 'vue'

export const LANGUAGES = [
  { code: 'EN', flag: '🇬🇧', label: 'English' },
  { code: 'DE', flag: '🇩🇪', label: 'Deutsch' },
  { code: 'FR', flag: '🇫🇷', label: 'Français' },
  { code: 'ES', flag: '🇪🇸', label: 'Español' },
  { code: 'IT', flag: '🇮🇹', label: 'Italiano' },
  { code: 'RU', flag: '🇷🇺', label: 'Русский' },
  { code: 'JA', flag: '🇯🇵', label: '日本語' },
  { code: 'ZH', flag: '🇨🇳', label: '中文' },
  { code: 'PT', flag: '🇵🇹', label: 'Português' },
]

const targetLang = ref(localStorage.getItem('targetLang') || 'EN')

watch(targetLang, (v) => localStorage.setItem('targetLang', v))

export function useLanguage() {
  return { targetLang, LANGUAGES }
}
