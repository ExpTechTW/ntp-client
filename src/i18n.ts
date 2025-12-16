'use client'

import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'

import zhHantTranslation from '../public/locales/zh-Hant/translation.json'
import jaJpTranslation from '../public/locales/ja-JP/translation.json'
import enUsTranslation from '../public/locales/en-US/translation.json'

const supportedLanguages = ['zh-Hant', 'ja-JP', 'en-US']
const defaultLanguage = 'zh-Hant'

if (!i18n.isInitialized) {
  i18n
    .use(initReactI18next)
    .init({
      lng: defaultLanguage,
      fallbackLng: defaultLanguage,
      supportedLngs: supportedLanguages,
      resources: {
        'zh-Hant': {
          translation: zhHantTranslation
        },
        'ja-JP': {
          translation: jaJpTranslation
        },
        'en-US': {
          translation: enUsTranslation
        },
      },
      defaultNS: 'translation',
      ns: ['translation'],
      interpolation: {
        escapeValue: false,
      },
      debug: false,
      react: {
        useSuspense: false,
      },
    })
}

if (typeof window !== 'undefined') {
  const stored = localStorage.getItem('ntp-client-language')
  if (stored && supportedLanguages.includes(stored) && i18n.language !== stored) {
    i18n.changeLanguage(stored)
  }
}

export const changeLanguage = (language: string) => {
  if (supportedLanguages.includes(language)) {
    i18n.changeLanguage(language)
    if (typeof window !== 'undefined') {
      localStorage.setItem('ntp-client-language', language)
    }
  }
}

export const getCurrentLanguage = () => i18n.language

export const getSupportedLanguages = () => supportedLanguages

export const languageConfig = {
  'zh-Hant': { name: '繁體中文', flag: '🇹🇼' },
  'ja-JP': { name: '日本語', flag: '🇯🇵' },
  'en-US': { name: 'English', flag: '🇺🇸' }
}

export default i18n
