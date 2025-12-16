'use client'

import { useState, useEffect, useRef } from 'react'
import { RefreshCw, CheckCircle2, AlertCircle, Loader2, Timer, Globe, Activity, Clock, Package, GitCompare, Sun, Moon, AlertTriangle, TrendingUp } from 'lucide-react'
import { invoke } from '@tauri-apps/api/core'
import { getVersion } from '@tauri-apps/api/app'
import { open } from '@tauri-apps/plugin-shell'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useTranslation } from 'react-i18next'
import { useRouter } from 'next/navigation'
import '@/i18n'
import LanguageSwitcher from '@/components/LanguageSwitcher'
import { useHistory } from '@/contexts/HistoryContext'

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

const TABS = [
  { id: 'time', icon: Clock },
  { id: 'calc', icon: Activity },
  { id: 'packet', icon: Package },
  { id: 'compare', icon: GitCompare },
] as const

type TabId = typeof TABS[number]['id']

const fmtS = (ms: number) => `${(ms / 1000).toFixed(3)}s`

const fmtTs = (ms: number) =>
  new Date(ms).toLocaleString('zh-TW', {
    month: '2-digit', day: '2-digit',
    hour: '2-digit', minute: '2-digit', second: '2-digit',
    fractionalSecondDigits: 3,
  })

const getStatus = (ms: number, t: (key: string) => string) => {
  const abs = Math.abs(ms)
  if (abs < 10) return { color: 'text-emerald-500', label: t('home.status.excellent') }
  if (abs < 50) return { color: 'text-green-500', label: t('home.status.good') }
  if (abs < 100) return { color: 'text-yellow-500', label: t('home.status.normal') }
  if (abs < 500) return { color: 'text-orange-500', label: t('home.status.deviation') }
  return { color: 'text-red-500', label: t('home.status.abnormal') }
}

const getStratumDesc = (s: number, t: (key: string) => string) => {
  if (s === 0) return t('home.stratum.unspecified')
  if (s === 1) return t('home.stratum.primary')
  if (s <= 15) return t('home.stratum.layer').replace('{{n}}', String(s))
  if (s === 16) return t('home.stratum.unsynced')
  return t('home.stratum.reserved')
}

const Info = ({ label, value, sub, isDark }: { label: string; value: React.ReactNode; sub?: string; isDark?: boolean }) => (
  <div className={`rounded px-1.5 py-0.5 ${isDark ? 'bg-zinc-800/40' : 'bg-zinc-200/60'}`}>
    <p className={`text-[9px] leading-none ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`}>{label}</p>
    <p className={`text-[10px] font-mono leading-tight mt-0.5 ${isDark ? 'text-zinc-300' : 'text-zinc-700'}`}>{value}</p>
    {sub && <p className={`text-[8px] font-mono leading-none mt-0.5 ${isDark ? 'text-zinc-600' : 'text-zinc-400'}`}>{sub}</p>}
  </div>
)

export default function HomePage() {
  const { t } = useTranslation()
  const router = useRouter()
  const { addEntry } = useHistory()
  const [server, setServer] = useState('time.exptech.com.tw')
  const [isQuerying, setIsQuerying] = useState(false)
  const [result, setResult] = useState<NtpResult | null>(null)
  const [now, setNow] = useState(new Date())
  const [countdown, setCountdown] = useState(0)
  const [tab, setTab] = useState<TabId>('time')
  const [isCompact, setIsCompact] = useState(false)
  const [version, setVersion] = useState('')
  const [isDark, setIsDark] = useState(true)
  const [permissionError, setPermissionError] = useState(false)
  const [sidecarNotInstalled, setSidecarNotInstalled] = useState(false)
  const [isInstallingSidecar, setIsInstallingSidecar] = useState(false)
  const refs = useRef<{ time?: NodeJS.Timeout; sync?: NodeJS.Timeout; cd?: NodeJS.Timeout; syncing?: boolean }>({})

  const toggleTheme = () => {
    const newTheme = !isDark
    setIsDark(newTheme)
    localStorage.setItem('theme', newTheme ? 'dark' : 'light')
  }

  const query = async (srv: string) => {
    if (!srv.trim() || refs.current.syncing) return
    refs.current.syncing = true
    setIsQuerying(true)
    setCountdown(0)
    setPermissionError(false)
    setSidecarNotInstalled(false)

    try {
      const res = JSON.parse(await invoke<string>('sync_ntp_time', { server: srv.trim() }))
      if (res.server) {
        const newResult = {
          success: res.success, server: res.server, server_ip: res.server_ip,
          t1: res.t1, t2: res.t2, t3: res.t3, t4: res.t4,
          offset: res.offset, delay: res.delay,
          leap: 0, version: 4, mode: 4, stratum: 1,
          poll: 0, precision: 0, root_delay: 0, root_dispersion: 0,
          ref_id: '', ref_time: 0,
          pre_sync_offset: res.pre_sync_offset, post_sync_offset: res.post_sync_offset,
        }
        setResult(newResult)
        localStorage.setItem('lastNtpResult', JSON.stringify(newResult))
        localStorage.setItem('lastNtpSyncTime', Date.now().toString())
        addEntry({
          offset: res.offset,
          delay: res.delay,
          server: srv,
        })
        invoke('db_insert_record', {
          offset: res.offset,
          delay: res.delay,
          server: srv,
          timestamp: Date.now()
        }).catch(err => console.error('[DB] Failed to insert record:', err))
        setPermissionError(res.code === 'PERMISSION_DENIED')
        setSidecarNotInstalled(res.code === 'SIDECAR_NOT_INSTALLED' || res.code === 'SIDECAR_NOT_RUNNING')
      } else {
        setResult(null)
      }
    } catch {
      setResult(null)
    } finally {
      setIsQuerying(false)
      refs.current.syncing = false
      setCountdown(60)
    }
  }

  const installSidecar = async () => {
    if (isInstallingSidecar) return
    setIsInstallingSidecar(true)
    try {
      const res = JSON.parse(await invoke<string>('install_sidecar'))

      const window = getCurrentWindow()
      await window.show()
      await window.setFocus()

      if (res.success) {
        setSidecarNotInstalled(false)
        setTimeout(() => {
          query(server)
        }, 2000)
      } else {
        console.error(t('home.sidecar.installFailed'), res.message)
      }
    } catch (e) {
      console.error(t('home.sidecar.installFailed'), e)
      try {
        const window = getCurrentWindow()
        await window.show()
        await window.setFocus()
      } catch {}
    } finally {
      setIsInstallingSidecar(false)
    }
  }

  useEffect(() => {
    setNow(new Date())
    refs.current.time = setInterval(() => setNow(new Date()), 50)
    getVersion().then(v => setVersion(`v${v}`)).catch(() => {})
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
    // 檢查是否有最近的同步結果（60秒內），避免頁面切換時重複同步
    const lastSyncTime = localStorage.getItem('lastNtpSyncTime')
    const now = Date.now()
    const shouldSkipInitialSync = lastSyncTime && (now - parseInt(lastSyncTime, 10)) < 60000

    if (!shouldSkipInitialSync) {
      query(server)
    } else {
      // 從快取載入上次結果
      const cachedResult = localStorage.getItem('lastNtpResult')
      if (cachedResult) {
        try {
          const parsed = JSON.parse(cachedResult)
          setResult(parsed)
          const elapsed = Math.floor((now - parseInt(lastSyncTime!, 10)) / 1000)
          setCountdown(Math.max(0, 60 - elapsed))
        } catch { /* ignore */ }
      }
    }

    refs.current.cd = setInterval(() => {
      setCountdown(p => {
        if (p <= 1) {
          if (!refs.current.syncing) {
            query(server)
          }
          return 0
        }
        return p - 1
      })
    }, 1000)
    return () => {
      clearInterval(refs.current.cd)
    }
  }, [server])

  const WEEKDAYS = [
    t('home.weekdays.sun'),
    t('home.weekdays.mon'),
    t('home.weekdays.tue'),
    t('home.weekdays.wed'),
    t('home.weekdays.thu'),
    t('home.weekdays.fri'),
    t('home.weekdays.sat'),
  ]

  const correctedTime = result ? new Date(now.getTime() + result.offset) : now
  const hh = String(correctedTime.getHours()).padStart(2, '0')
  const mm = String(correctedTime.getMinutes()).padStart(2, '0')
  const ss = String(correctedTime.getSeconds()).padStart(2, '0')
  const ms = String(correctedTime.getMilliseconds()).padStart(3, '0')
  const dateStr = `${correctedTime.getFullYear()}/${String(correctedTime.getMonth() + 1).padStart(2, '0')}/${String(correctedTime.getDate()).padStart(2, '0')} ${t('home.week')}${WEEKDAYS[correctedTime.getDay()]}`
  const status = result ? getStatus(result.offset, t) : null

  const sysHh = String(now.getHours()).padStart(2, '0')
  const sysMm = String(now.getMinutes()).padStart(2, '0')
  const sysSs = String(now.getSeconds()).padStart(2, '0')
  const sysMs = String(now.getMilliseconds()).padStart(3, '0')

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
        {result && status && (
          <div className="flex items-center gap-2 mt-1">
            <span className={`text-xs ${status.color}`}>{status.label}</span>
            <span className={`text-sm font-mono ${status.color}`} style={{ fontVariantNumeric: 'tabular-nums' }}>
              {result.offset >= 0 ? '+' : ''}{fmtS(result.offset)}
            </span>
          </div>
        )}
        {permissionError && (
          <div className="flex flex-col items-center gap-1 mt-2">
            <div className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-yellow-500/20 border border-yellow-500/50">
              <AlertTriangle className="w-4 h-4 text-yellow-500" />
              <span className="text-xs text-yellow-500">{t('home.permissionError')}</span>
            </div>
            <div className="flex items-center gap-1 text-yellow-500/80">
              <span className="text-[10px]">{t('home.systemTime')}:</span>
              <span className="text-xs font-mono" style={{ fontVariantNumeric: 'tabular-nums' }}>
                {sysHh}:{sysMm}:{sysSs}.{sysMs}
              </span>
            </div>
          </div>
        )}
        {sidecarNotInstalled && (
          <div className="flex flex-col items-center gap-2 mt-2">
            <div className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-yellow-500/20 border border-yellow-500/50">
              <AlertTriangle className="w-4 h-4 text-yellow-500" />
              <span className="text-xs text-yellow-500">{t('home.sidecar.installRequired')}</span>
            </div>
            <button
              onClick={installSidecar}
              disabled={isInstallingSidecar}
              className={`flex items-center gap-1.5 px-3 py-1.5 rounded text-xs font-medium text-white transition-colors ${
                isInstallingSidecar
                  ? 'bg-zinc-600 cursor-not-allowed'
                  : 'bg-yellow-500 hover:bg-yellow-600'
              }`}
            >
              {isInstallingSidecar ? (
                <>
                  <Loader2 className="w-3.5 h-3.5 animate-spin" />
                  <span>{t('home.sidecar.installing')}</span>
                </>
              ) : (
                <>
                  <Package className="w-3.5 h-3.5" />
                  <span>{t('home.sidecar.installButton')}</span>
                </>
              )}
            </button>
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
            <Timer className="w-2.5 h-2.5" />{isQuerying ? '--' : `${countdown}s`}
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
          {result ? (
            <>
              <div className={`flex items-center border-b ${isDark ? 'border-zinc-800' : 'border-zinc-200'}`}>
                <div className={`flex items-center gap-1 px-1.5 py-1 border-r ${isDark ? 'border-zinc-800' : 'border-zinc-200'}`}>
                  {result.success ? (
                    <CheckCircle2 className="w-2.5 h-2.5 text-emerald-500" />
                  ) : (
                    <AlertTriangle className="w-2.5 h-2.5 text-yellow-500" />
                  )}
                  <span className={`text-[9px] ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`}>{result.server_ip}</span>
                </div>
                <div className="flex flex-1">
                  {TABS.map(tabItem => (
                    <button
                      key={tabItem.id}
                      onClick={() => setTab(tabItem.id)}
                      className={`flex-1 flex items-center justify-center gap-0.5 py-1 text-[9px] ${
                        tab === tabItem.id ? 'text-blue-400 bg-blue-500/10' : isDark ? 'text-zinc-600 hover:text-zinc-400' : 'text-zinc-400 hover:text-zinc-600'
                      }`}
                    >
                      <tabItem.icon className="w-2.5 h-2.5" />
                      <span className="hidden sm:inline">{t(`home.tabs.${tabItem.id}`)}</span>
                    </button>
                  ))}
                </div>
              </div>

              <div className="p-1.5">
                {tab === 'time' && (
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-1">
                    <Info label={t('home.timeLabels.t1Send')} value={fmtTs(result.t1)} isDark={isDark} />
                    <Info label={t('home.timeLabels.t2Receive')} value={fmtTs(result.t2)} isDark={isDark} />
                    <Info label={t('home.timeLabels.t3Response')} value={fmtTs(result.t3)} isDark={isDark} />
                    <Info label={t('home.timeLabels.t4Received')} value={fmtTs(result.t4)} isDark={isDark} />
                  </div>
                )}

                {tab === 'calc' && (
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-1">
                    <Info label={t('home.calcLabels.offset')} value={<span className={status?.color}>{result.offset >= 0 ? '+' : ''}{fmtS(result.offset)}</span>} sub="((T2-T1)+(T3-T4))/2" isDark={isDark} />
                    <Info label={t('home.calcLabels.delay')} value={fmtS(result.delay)} sub="(T4-T1)-(T3-T2)" isDark={isDark} />
                    <Info label={t('home.calcLabels.rtt')} value={fmtS(result.t4 - result.t1)} sub="T4-T1" isDark={isDark} />
                    <Info label={t('home.calcLabels.processingTime')} value={fmtS(result.t3 - result.t2)} sub="T3-T2" isDark={isDark} />
                  </div>
                )}

                {tab === 'packet' && (
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-1">
                    <Info label={t('home.packetLabels.liVnMode')} value={`0x${((result.leap << 6) | (result.version << 3) | result.mode).toString(16).padStart(2, '0')}`} sub={`LI=${result.leap} VN=${result.version} M=${result.mode}`} isDark={isDark} />
                    <Info label={t('home.packetLabels.stratum')} value={result.stratum} sub={getStratumDesc(result.stratum, t)} isDark={isDark} />
                    <Info label={t('home.packetLabels.poll')} value={`${result.poll}`} sub={`${Math.pow(2, result.poll)}s`} isDark={isDark} />
                    <Info label={t('home.packetLabels.precision')} value={`${result.precision}`} sub={`${Math.pow(2, result.precision).toExponential(1)}s`} isDark={isDark} />
                    <Info label={t('home.packetLabels.rootDelay')} value={fmtS(result.root_delay)} isDark={isDark} />
                    <Info label={t('home.packetLabels.rootDisp')} value={fmtS(result.root_dispersion)} isDark={isDark} />
                    <Info label={t('home.packetLabels.refId')} value={result.ref_id || '-'} isDark={isDark} />
                    <Info label={t('home.packetLabels.refTime')} value={result.ref_time ? fmtTs(result.ref_time) : '-'} isDark={isDark} />
                  </div>
                )}

                {tab === 'compare' && (
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-1">
                    <Info label={t('home.calcLabels.preSyncOffset')} value={<span className={getStatus(result.pre_sync_offset, t).color}>{result.pre_sync_offset >= 0 ? '+' : ''}{fmtS(result.pre_sync_offset)}</span>} sub={t('home.calcSubs.preSyncMeasure')} isDark={isDark} />
                    <Info label={t('home.calcLabels.postSyncOffset')} value={<span className={status?.color}>{result.post_sync_offset >= 0 ? '+' : ''}{fmtS(result.post_sync_offset)}</span>} sub={t('home.calcSubs.postSyncVerify')} isDark={isDark} />
                    <Info label={t('home.calcLabels.correctionAmount')} value={fmtS(result.pre_sync_offset - result.post_sync_offset)} sub={t('home.calcSubs.improvement')} isDark={isDark} />
                    <Info label={t('home.calcLabels.delay')} value={fmtS(result.delay)} sub={t('home.calcLabels.networkDelay')} isDark={isDark} />
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
              <span className="text-[10px]">{t('home.syncing')}</span>
            </div>
          )}
        </div>

        <div className="flex items-center justify-between px-1">
          <div className="flex items-center gap-3">
            <span className={`text-[10px] font-mono ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`}>{version}</span>
          </div>
          <div className="flex items-center gap-3">
            <button
              onClick={() => router.push('/history')}
              className={`flex items-center gap-1 text-[10px] transition-colors ${
                isDark ? 'text-zinc-600 hover:text-zinc-400' : 'text-zinc-400 hover:text-zinc-600'
              }`}
            >
              <TrendingUp className="w-3 h-3" />
              {t('home.historyLabels.viewDetails')}
            </button>
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
