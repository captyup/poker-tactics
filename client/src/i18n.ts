import { createI18n } from 'vue-i18n'
import en from './locales/en.json'
import zhTW from './locales/zh-TW.json'
import zhCN from './locales/zh-CN.json'

const i18n = createI18n({
  legacy: false, // Use Composition API mode
  locale: navigator.language.includes('TW') ? 'zh-TW' : (navigator.language.includes('CN') ? 'zh-CN' : 'en'), 
  fallbackLocale: 'en',
  messages: {
    en,
    'zh-TW': zhTW,
    'zh-CN': zhCN
  }
})

export default i18n
