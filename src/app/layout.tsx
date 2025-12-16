import type { Metadata } from 'next'
import './globals.css'
import '@/i18n'
import { HistoryProvider } from '@/contexts/HistoryContext'

export const metadata: Metadata = {
  title: 'NTP Client - 網路時間同步',
  description: 'NTP 時間同步客戶端工具',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="zh-TW" suppressHydrationWarning>
      <body className="bg-zinc-950">
        <HistoryProvider>
          {children}
        </HistoryProvider>
      </body>
    </html>
  )
}
