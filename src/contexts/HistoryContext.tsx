'use client'

import { createContext, useContext, useState, useCallback, ReactNode } from 'react'

export interface HistoryEntry {
  time: Date
  offset: number
  delay: number
  server: string
}

interface HistoryContextType {
  history: HistoryEntry[]
  addEntry: (entry: Omit<HistoryEntry, 'time'>) => void
  clearHistory: () => void
}

const HistoryContext = createContext<HistoryContextType | null>(null)

const MAX_HISTORY = 3600

export function HistoryProvider({ children }: { children: ReactNode }) {
  const [history, setHistory] = useState<HistoryEntry[]>([])

  const addEntry = useCallback((entry: Omit<HistoryEntry, 'time'>) => {
    setHistory(prev => {
      const newEntry: HistoryEntry = {
        ...entry,
        time: new Date(),
      }
      return [...prev, newEntry].slice(-MAX_HISTORY)
    })
  }, [])

  const clearHistory = useCallback(() => {
    setHistory([])
  }, [])

  return (
    <HistoryContext.Provider value={{ history, addEntry, clearHistory }}>
      {children}
    </HistoryContext.Provider>
  )
}

export function useHistory() {
  const context = useContext(HistoryContext)
  if (!context) {
    throw new Error('useHistory must be used within a HistoryProvider')
  }
  return context
}
