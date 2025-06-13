import { createI18n } from 'vue-i18n'
import en from './en'
import ja from './ja'
import zh from './zh'

const i18n = createI18n({
  legacy: false,
  locale: 'zh',
  fallbackLocale: 'en',
  messages: {
    en,
    ja,
    zh
  }
})

export default i18n 