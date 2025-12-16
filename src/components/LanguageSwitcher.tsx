'use client'

import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Languages } from 'lucide-react'
import { changeLanguage, getCurrentLanguage, getSupportedLanguages, languageConfig } from '@/i18n'

export default function LanguageSwitcher() {
  const { i18n } = useTranslation()
  const [isOpen, setIsOpen] = useState(false)
  
  const currentLanguage = getCurrentLanguage()
  const supportedLanguages = getSupportedLanguages()
  const currentLanguageInfo = languageConfig[currentLanguage as keyof typeof languageConfig]

  const handleLanguageChange = (language: string) => {
    changeLanguage(language)
    setIsOpen(false)
  }

  const code = currentLanguage === 'zh-Hant' ? 'ZH' : currentLanguage === 'ja-JP' ? 'JA' : currentLanguage === 'en-US' ? 'EN' : currentLanguage

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-1.5 text-[10px] text-zinc-500 hover:text-zinc-300 transition-colors"
      >
        <Languages className="w-3 h-3" />
        <span>{code}</span>
      </button>

      {isOpen && (
        <div className="absolute bottom-full right-0 mb-1 min-w-[100px] bg-zinc-900 border border-zinc-800 rounded shadow-lg z-50">
          {supportedLanguages.map((language) => {
            const languageInfo = languageConfig[language as keyof typeof languageConfig]
            const isSelected = language === currentLanguage

            return (
              <button
                key={language}
                onClick={() => handleLanguageChange(language)}
                className={`w-full flex items-center gap-1.5 px-2 py-1 text-left text-[9px] hover:bg-zinc-800 transition-colors first:rounded-t last:rounded-b ${
                  isSelected ? 'text-blue-400' : 'text-zinc-400'
                }`}
              >
                <span>{languageInfo?.name}</span>
                {isSelected && (
                  <svg className="w-2.5 h-2.5 ml-auto" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                  </svg>
                )}
              </button>
            )
          })}
        </div>
      )}
    </div>
  )
}
