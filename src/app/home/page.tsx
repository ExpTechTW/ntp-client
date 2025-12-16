'use client'

import { useState, useEffect, useRef } from 'react'
import { RefreshCw, CheckCircle2, AlertCircle, Loader2, Timer, Power, Globe, Activity, Clock, Package, GitCompare, Sun, Moon } from 'lucide-react'
import { invoke } from '@tauri-apps/api/core'
import { getVersion } from '@tauri-apps/api/app'
import { open } from '@tauri-apps/plugin-shell'
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart'
import { useTranslation } from 'react-i18next'
import '@/i18n'
import LanguageSwitcher from '@/components/LanguageSwitcher'

interface NtpResult {
  success: boolean
  server: string
  server_ip: string
  t1: number
  t2: number
  t3: number
  t4: number
  offset: number
  delay: number
  leap: number
  version: number
  mode: number
  stratum: number
  poll: number
  precision: number
  root_delay: number
  root_dispersion: number
  ref_id: string
  ref_time: number
  pre_sync_offset: number
  post_sync_offset: number
}

const NTP_SERVERS = [
  { value: 'time.exptech.com.tw', label: 'ExpTech' },
  { value: 'time.apple.com', label: 'Apple' },
  { value: 'time.google.com', label: 'Google' },
  { value: 'time.cloudflare.com', label: 'Cloudflare' },
]

const WEEKDAYS = ['日', '一', '二', '三', '四', '五', '六']

const TABS = [
  { id: 'time', label: '時間戳', icon: Clock },
  { id: 'calc', label: '計算', icon: Activity },
  { id: 'packet', label: '封包', icon: Package },
  { id: 'compare', label: '對比', icon: GitCompare },
] as const

type TabId = typeof TABS[number]['id']

const fmtS = (ms: number) => `${(ms / 1000).toFixed(3)}s`

const fmtTs = (ms: number) =>
  new Date(ms).toLocaleString('zh-TW', {
    month: '2-digit', day: '2-digit',
    hour: '2-digit', minute: '2-digit', second: '2-digit',
    fractionalSecondDigits: 3,
  })

const getStatus = (ms: number) => {
  const abs = Math.abs(ms)
  if (abs < 10) return { color: 'text-emerald-500', label: '極佳' }
  if (abs < 50) return { color: 'text-green-500', label: '良好' }
  if (abs < 100) return { color: 'text-yellow-500', label: '正常' }
  if (abs < 500) return { color: 'text-orange-500', label: '偏差' }
  return { color: 'text-red-500', label: '異常' }
}

const getStratumDesc = (s: number) =>
  s === 0 ? '未指定' : s === 1 ? '主參考' : s <= 15 ? `第${s}層` : s === 16 ? '未同步' : '保留'

const Info = ({ label, value, sub, isDark }: { label: string; value: React.ReactNode; sub?: string; isDark?: boolean }) => (
  <div className={`rounded px-1.5 py-0.5 ${isDark ? 'bg-zinc-800/40' : 'bg-zinc-200/60'}`}>
    <p className={`text-[9px] leading-none ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`}>{label}</p>
    <p className={`text-[10px] font-mono leading-tight mt-0.5 ${isDark ? 'text-zinc-300' : 'text-zinc-700'}`}>{value}</p>
    {sub && <p className={`text-[8px] font-mono leading-none mt-0.5 ${isDark ? 'text-zinc-600' : 'text-zinc-400'}`}>{sub}</p>}
  </div>
)

export default function HomePage() {
  const { t } = useTranslation()
  const [server, setServer] = useState('time.exptech.com.tw')
  const [isQuerying, setIsQuerying] = useState(false)
  const [result, setResult] = useState<NtpResult | null>(null)
  const [now, setNow] = useState(new Date())
  const [countdown, setCountdown] = useState(60)
  const [autostart, setAutostart] = useState(false)
  const [tab, setTab] = useState<TabId>('time')
  const [isCompact, setIsCompact] = useState(false)
  const [version, setVersion] = useState('')
  const [isDark, setIsDark] = useState(true)
  const refs = useRef<{ time?: NodeJS.Timeout; sync?: NodeJS.Timeout; cd?: NodeJS.Timeout; syncing?: boolean }>({})

  const toggleTheme = () => {
    const newTheme = !isDark
    setIsDark(newTheme)
    localStorage.setItem('theme', newTheme ? 'dark' : 'light')
  }

  const query = async (srv: string) => {
    if (!srv.trim() || isQuerying || refs.current.syncing) return
    refs.current.syncing = true
    setIsQuerying(true)
    setCountdown(60)

    try {
      const res = JSON.parse(await invoke<string>('sync_ntp_time', { server: srv.trim() }))
      if (res.success) {
        setResult({
          success: true, server: res.server, server_ip: res.server_ip,
          t1: res.t1, t2: res.t2, t3: res.t3, t4: res.t4,
          offset: res.offset, delay: res.delay,
          leap: 0, version: 4, mode: 4, stratum: 1,
          poll: 0, precision: 0, root_delay: 0, root_dispersion: 0,
          ref_id: '', ref_time: 0,
          pre_sync_offset: res.pre_sync_offset, post_sync_offset: res.post_sync_offset,
        })
      } else {
        setResult(null)
      }
    } catch {
      setResult(null)
    } finally {
      setIsQuerying(false)
      refs.current.syncing = false
    }
  }

  const toggleAutostart = async () => {
    try {
      autostart ? await disable() : await enable()
      setAutostart(!autostart)
    } catch {}
  }

  useEffect(() => {
    setNow(new Date())
    refs.current.time = setInterval(() => setNow(new Date()), 50)
    isEnabled().then(setAutostart).catch(() => {})
    getVersion().then(v => setVersion(`v${v}`)).catch(() => {})

    // 讀取主題設定
    const savedTheme = localStorage.getItem('theme')
    if (savedTheme) {
      setIsDark(savedTheme === 'dark')
    }

    const checkSize = () => setIsCompact(window.innerHeight < 300)
    checkSize()
    window.addEventListener('resize', checkSize)
    return () => {
      clearInterval(refs.current.time)
      window.removeEventListener('resize', checkSize)
    }
  }, [])

  useEffect(() => {
    query(server)
    refs.current.sync = setInterval(() => query(server), 60000)
    refs.current.cd = setInterval(() => setCountdown(p => (p <= 1 ? 60 : p - 1)), 1000)
    return () => {
      clearInterval(refs.current.sync)
      clearInterval(refs.current.cd)
    }
  }, [server])

  const hh = String(now.getHours()).padStart(2, '0')
  const mm = String(now.getMinutes()).padStart(2, '0')
  const ss = String(now.getSeconds()).padStart(2, '0')
  const ms = String(now.getMilliseconds()).padStart(3, '0')
  const dateStr = `${now.getFullYear()}/${String(now.getMonth() + 1).padStart(2, '0')}/${String(now.getDate()).padStart(2, '0')} 星期${WEEKDAYS[now.getDay()]}`
  const status = result ? getStatus(result.offset) : null

  const Digit = ({ children, className = '' }: { children: string; className?: string }) => (
    <span className={`inline-block text-center ${className}`} style={{ fontVariantNumeric: 'tabular-nums' }}>
      {children}
    </span>
  )

  if (isCompact) {
    return (
      <div className={`h-screen flex items-center justify-center select-none overflow-hidden ${isDark ? 'bg-zinc-950' : 'bg-zinc-100'}`}>
        <div className="text-center">
          <div className="flex items-baseline justify-center font-mono">
            <Digit className={`text-3xl font-bold w-[1.2ch] ${isDark ? 'text-white' : 'text-zinc-900'}`}>{hh[0]}</Digit>
            <Digit className={`text-3xl font-bold w-[1.2ch] ${isDark ? 'text-white' : 'text-zinc-900'}`}>{hh[1]}</Digit>
            <span className={`text-3xl font-bold ${isDark ? 'text-white' : 'text-zinc-900'}`}>:</span>
            <Digit className={`text-3xl font-bold w-[1.2ch] ${isDark ? 'text-white' : 'text-zinc-900'}`}>{mm[0]}</Digit>
            <Digit className={`text-3xl font-bold w-[1.2ch] ${isDark ? 'text-white' : 'text-zinc-900'}`}>{mm[1]}</Digit>
            <span className={`text-3xl font-bold ${isDark ? 'text-white' : 'text-zinc-900'}`}>:</span>
            <Digit className={`text-3xl font-bold w-[1.2ch] ${isDark ? 'text-white' : 'text-zinc-900'}`}>{ss[0]}</Digit>
            <Digit className={`text-3xl font-bold w-[1.2ch] ${isDark ? 'text-white' : 'text-zinc-900'}`}>{ss[1]}</Digit>
            <span className={`text-base ${isDark ? 'text-zinc-500' : 'text-zinc-400'}`}>.</span>
            <Digit className={`text-base w-[1ch] ${isDark ? 'text-zinc-500' : 'text-zinc-400'}`}>{ms[0]}</Digit>
            <Digit className={`text-base w-[1ch] ${isDark ? 'text-zinc-500' : 'text-zinc-400'}`}>{ms[1]}</Digit>
            <Digit className={`text-base w-[1ch] ${isDark ? 'text-zinc-500' : 'text-zinc-400'}`}>{ms[2]}</Digit>
          </div>
          <p className={`text-xs mt-1 ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`} suppressHydrationWarning>{dateStr}</p>
          {status && (
            <p className={`text-[9px] font-mono ${status.color}`}>
              {result!.offset >= 0 ? '+' : ''}{fmtS(result!.offset)}
            </p>
          )}
        </div>
      </div>
    )
  }

  return (
    <div className={`h-screen select-none overflow-hidden flex flex-col ${isDark ? 'bg-zinc-950 text-white' : 'bg-zinc-100 text-zinc-900'}`}>

      <div className="flex-1 flex flex-col items-center justify-center min-h-0 py-2">
        <div className="flex items-baseline justify-center font-mono">
          <Digit className={`text-6xl sm:text-7xl md:text-8xl font-bold w-[1.2ch] ${isDark ? '' : 'text-zinc-900'}`}>{hh[0]}</Digit>
          <Digit className={`text-6xl sm:text-7xl md:text-8xl font-bold w-[1.2ch] ${isDark ? '' : 'text-zinc-900'}`}>{hh[1]}</Digit>
          <span className={`text-6xl sm:text-7xl md:text-8xl font-bold ${isDark ? '' : 'text-zinc-900'}`}>:</span>
          <Digit className={`text-6xl sm:text-7xl md:text-8xl font-bold w-[1.2ch] ${isDark ? '' : 'text-zinc-900'}`}>{mm[0]}</Digit>
          <Digit className={`text-6xl sm:text-7xl md:text-8xl font-bold w-[1.2ch] ${isDark ? '' : 'text-zinc-900'}`}>{mm[1]}</Digit>
          <span className={`text-6xl sm:text-7xl md:text-8xl font-bold ${isDark ? '' : 'text-zinc-900'}`}>:</span>
          <Digit className={`text-6xl sm:text-7xl md:text-8xl font-bold w-[1.2ch] ${isDark ? '' : 'text-zinc-900'}`}>{ss[0]}</Digit>
          <Digit className={`text-6xl sm:text-7xl md:text-8xl font-bold w-[1.2ch] ${isDark ? '' : 'text-zinc-900'}`}>{ss[1]}</Digit>
          <span className={`text-2xl sm:text-3xl ${isDark ? 'text-zinc-500' : 'text-zinc-400'}`}>.</span>
          <Digit className={`text-2xl sm:text-3xl w-[1ch] ${isDark ? 'text-zinc-500' : 'text-zinc-400'}`}>{ms[0]}</Digit>
          <Digit className={`text-2xl sm:text-3xl w-[1ch] ${isDark ? 'text-zinc-500' : 'text-zinc-400'}`}>{ms[1]}</Digit>
          <Digit className={`text-2xl sm:text-3xl w-[1ch] ${isDark ? 'text-zinc-500' : 'text-zinc-400'}`}>{ms[2]}</Digit>
        </div>
        <p className={`text-sm mt-1 ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`} suppressHydrationWarning>{dateStr}</p>
        {result?.success && status && (
          <div className="flex items-center gap-2 mt-1">
            <span className={`text-xs ${status.color}`}>{status.label}</span>
            <span className={`text-sm font-mono ${status.color}`} style={{ fontVariantNumeric: 'tabular-nums' }}>
              {result.offset >= 0 ? '+' : ''}{fmtS(result.offset)}
            </span>
          </div>
        )}
      </div>

      <div className="shrink-0 px-2 pb-2 space-y-1.5">
        <div className={`flex items-center gap-1.5 rounded px-2 py-1 border ${isDark ? 'bg-zinc-900 border-zinc-800' : 'bg-white border-zinc-300'}`}>
          <Globe className={`w-3 h-3 ${isDark ? 'text-zinc-600' : 'text-zinc-400'}`} />
          <select
            value={server}
            onChange={e => setServer(e.target.value)}
            className={`flex-1 bg-transparent text-[10px] focus:outline-none cursor-pointer ${isDark ? 'text-zinc-400' : 'text-zinc-600'}`}
          >
            {NTP_SERVERS.map(s => (
              <option key={s.value} value={s.value} className={isDark ? 'bg-zinc-900' : 'bg-white'}>
                {s.label} - {s.value}
              </option>
            ))}
          </select>
          <span className={`text-[9px] tabular-nums flex items-center gap-0.5 ${isDark ? 'text-zinc-600' : 'text-zinc-400'}`}>
            <Timer className="w-2.5 h-2.5" />{countdown}s
          </span>
          <button
            onClick={() => query(server)}
            disabled={isQuerying}
            className={`flex items-center gap-0.5 px-1.5 py-0.5 rounded text-[9px] font-medium text-white ${isQuerying ? 'bg-zinc-700' : 'bg-blue-600 hover:bg-blue-500'}`}
          >
            {isQuerying ? <Loader2 className="w-2.5 h-2.5 animate-spin" /> : <RefreshCw className="w-2.5 h-2.5" />}
            <span>{t('home.query')}</span>
          </button>
        </div>

        <div className={`rounded border ${isDark ? 'bg-zinc-900 border-zinc-800' : 'bg-white border-zinc-300'}`}>
          {result?.success ? (
            <>
              <div className={`flex items-center border-b ${isDark ? 'border-zinc-800' : 'border-zinc-200'}`}>
                <div className={`flex items-center gap-1 px-1.5 py-1 border-r ${isDark ? 'border-zinc-800' : 'border-zinc-200'}`}>
                  <CheckCircle2 className="w-2.5 h-2.5 text-emerald-500" />
                  <span className={`text-[9px] ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`}>{result.server_ip}</span>
                </div>
                <div className="flex flex-1">
                  {TABS.map(t => (
                    <button
                      key={t.id}
                      onClick={() => setTab(t.id)}
                      className={`flex-1 flex items-center justify-center gap-0.5 py-1 text-[9px] ${
                        tab === t.id ? 'text-blue-400 bg-blue-500/10' : isDark ? 'text-zinc-600 hover:text-zinc-400' : 'text-zinc-400 hover:text-zinc-600'
                      }`}
                    >
                      <t.icon className="w-2.5 h-2.5" />
                      <span className="hidden sm:inline">{t.label}</span>
                    </button>
                  ))}
                </div>
              </div>

              <div className="p-1.5">
                {tab === 'time' && (
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-1">
                    <Info label="T1 發送" value={fmtTs(result.t1)} isDark={isDark} />
                    <Info label="T2 接收" value={fmtTs(result.t2)} isDark={isDark} />
                    <Info label="T3 回應" value={fmtTs(result.t3)} isDark={isDark} />
                    <Info label="T4 收到" value={fmtTs(result.t4)} isDark={isDark} />
                  </div>
                )}

                {tab === 'calc' && (
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-1">
                    <Info label="Offset" value={<span className={status?.color}>{result.offset >= 0 ? '+' : ''}{fmtS(result.offset)}</span>} sub="((T2-T1)+(T3-T4))/2" isDark={isDark} />
                    <Info label="Delay" value={fmtS(result.delay)} sub="(T4-T1)-(T3-T2)" isDark={isDark} />
                    <Info label="RTT" value={fmtS(result.t4 - result.t1)} sub="T4-T1" isDark={isDark} />
                    <Info label="處理時間" value={fmtS(result.t3 - result.t2)} sub="T3-T2" isDark={isDark} />
                  </div>
                )}

                {tab === 'packet' && (
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-1">
                    <Info label="LI/VN/Mode" value={`0x${((result.leap << 6) | (result.version << 3) | result.mode).toString(16).padStart(2, '0')}`} sub={`LI=${result.leap} VN=${result.version} M=${result.mode}`} isDark={isDark} />
                    <Info label="Stratum" value={result.stratum} sub={getStratumDesc(result.stratum)} isDark={isDark} />
                    <Info label="Poll" value={`${result.poll}`} sub={`${Math.pow(2, result.poll)}s`} isDark={isDark} />
                    <Info label="Precision" value={`${result.precision}`} sub={`${Math.pow(2, result.precision).toExponential(1)}s`} isDark={isDark} />
                    <Info label="Root Delay" value={fmtS(result.root_delay)} isDark={isDark} />
                    <Info label="Root Disp" value={fmtS(result.root_dispersion)} isDark={isDark} />
                    <Info label="Ref ID" value={result.ref_id || '-'} isDark={isDark} />
                    <Info label="Ref Time" value={result.ref_time ? fmtTs(result.ref_time) : '-'} isDark={isDark} />
                  </div>
                )}

                {tab === 'compare' && (
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-1">
                    <Info label="校正前誤差" value={<span className={getStatus(result.pre_sync_offset).color}>{result.pre_sync_offset >= 0 ? '+' : ''}{fmtS(result.pre_sync_offset)}</span>} sub="同步前測量" isDark={isDark} />
                    <Info label="校正後誤差" value={<span className={status?.color}>{result.post_sync_offset >= 0 ? '+' : ''}{fmtS(result.post_sync_offset)}</span>} sub="同步後驗證" isDark={isDark} />
                    <Info label="校正量" value={fmtS(result.pre_sync_offset - result.post_sync_offset)} sub="改善幅度" isDark={isDark} />
                    <Info label="Delay" value={fmtS(result.delay)} sub="網路延遲" isDark={isDark} />
                  </div>
                )}
              </div>
            </>
          ) : result === null && !isQuerying ? (
            <div className="flex items-center justify-center gap-1 py-2 text-red-400">
              <AlertCircle className="w-3 h-3" />
              <span className="text-[10px]">{t('home.queryFailed')}</span>
            </div>
          ) : (
            <div className={`flex items-center justify-center gap-1 py-2 ${isDark ? 'text-zinc-600' : 'text-zinc-400'}`}>
              <Loader2 className="w-3 h-3 animate-spin" />
              <span className="text-[10px]">同步中...</span>
            </div>
          )}
        </div>

        <div className="flex items-center justify-between px-1">
          <div className="flex items-center gap-3">
            <span className={`text-[10px] font-mono ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`}>{version}</span>
            <button onClick={toggleAutostart} className={`flex items-center gap-1.5 text-[10px] transition-colors ${isDark ? 'text-zinc-500 hover:text-zinc-300' : 'text-zinc-500 hover:text-zinc-700'}`}>
              <Power className={`w-3 h-3 ${autostart ? 'text-emerald-400' : ''}`} />
              <span>開機啟動</span>
              <span className={`w-1.5 h-1.5 rounded-full ${autostart ? 'bg-emerald-400' : isDark ? 'bg-zinc-600' : 'bg-zinc-400'}`} />
            </button>
          </div>
          <div className="flex items-center gap-3">
            <button
              onClick={() => open('https://github.com/ExpTechTW/ntp-client')}
              className={`transition-colors ${isDark ? 'text-zinc-500 hover:text-zinc-300' : 'text-zinc-500 hover:text-zinc-700'}`}
            >
              <svg className="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
            </button>
            <button
              onClick={toggleTheme}
              className={`transition-colors ${isDark ? 'text-zinc-500 hover:text-zinc-300' : 'text-zinc-500 hover:text-zinc-700'}`}
            >
              {isDark ? <Sun className="w-3.5 h-3.5" /> : <Moon className="w-3.5 h-3.5" />}
            </button>
            <LanguageSwitcher />
          </div>
        </div>
      </div>
    </div>
  )
}
