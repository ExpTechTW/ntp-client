'use client'

import { useState, useEffect, useRef } from 'react'
import { Clock, Server, RefreshCw, CheckCircle2, AlertCircle, Loader2, Timer, Power, Activity, Package, GitCompare } from 'lucide-react'
import { invoke } from '@tauri-apps/api/core'
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
}

interface NtpError {
  success: boolean
  error: string
  code: string
}

const NTP_SERVERS = [
  { value: 'time.exptech.com.tw', label: 'ExpTech (台灣)' },
  { value: 'time.apple.com', label: 'Apple' },
  { value: 'time.google.com', label: 'Google' },
  { value: 'time.cloudflare.com', label: 'Cloudflare' },
]
const WEEKDAYS = ['日', '一', '二', '三', '四', '五', '六']

const TABS = [
  { id: 'timestamps', label: '時間戳記', icon: Clock },
  { id: 'results', label: '計算結果', icon: Activity },
  { id: 'packet', label: '封包資訊', icon: Package },
  { id: 'compare', label: '時間對比', icon: GitCompare },
] as const

const getStratumDesc = (s: number) =>
  s === 0 ? '未指定' : s === 1 ? '主要參考 (GPS/原子鐘)' : s <= 15 ? `次級參考 (第 ${s} 層)` : s === 16 ? '未同步' : '保留'

const getLeapDesc = (l: number) => ['正常', '+1 秒', '-1 秒', '未同步'][l] ?? '未知'

const getOffsetColor = (ms: number) => {
  const abs = Math.abs(ms)
  return abs < 100 ? 'text-emerald-500' : abs < 1000 ? 'text-yellow-500' : 'text-red-500'
}

const formatOffset = (ms: number) => {
  const abs = Math.abs(ms)
  return abs < 1000 ? `${ms.toFixed(3)} ms` : `${(ms / 1000).toFixed(3)} s`
}

const formatTs = (ms: number) =>
  new Date(ms).toLocaleString('zh-TW', {
    year: 'numeric', month: '2-digit', day: '2-digit',
    hour: '2-digit', minute: '2-digit', second: '2-digit',
    fractionalSecondDigits: 3,
  })

const formatTime = (d: Date) => ({
  time: `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}:${String(d.getSeconds()).padStart(2, '0')}`,
  date: `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')} 星期${WEEKDAYS[d.getDay()]}`,
  ms: String(d.getMilliseconds()).padStart(3, '0'),
})

const InfoCard = ({ label, value, sub }: { label: string; value: React.ReactNode; sub?: string }) => (
  <div className="p-2.5 bg-muted/30 rounded-md">
    <p className="text-[11px] text-muted-foreground">{label}</p>
    <p className="text-sm font-mono text-foreground mt-0.5">{value}</p>
    {sub && <p className="text-[10px] text-muted-foreground font-mono mt-0.5">{sub}</p>}
  </div>
)

export default function HomePage() {
  const { t } = useTranslation()
  const [server, setServer] = useState('time.exptech.com.tw')
  const [isQuerying, setIsQuerying] = useState(false)
  const [result, setResult] = useState<NtpResult | null>(null)
  const [now, setNow] = useState(new Date())
  const [countdown, setCountdown] = useState(30)
  const [mouse, setMouse] = useState({ x: 0, y: 0 })
  const [autostart, setAutostart] = useState(false)
  const [activeTab, setActiveTab] = useState<typeof TABS[number]['id']>('timestamps')
  const refs = useRef<{ time?: NodeJS.Timeout; sync?: NodeJS.Timeout; cd?: NodeJS.Timeout; syncing?: boolean }>({})

  const query = async (srv: string, manual = false) => {
    if (!srv.trim() || isQuerying || refs.current.syncing) return
    refs.current.syncing = true
    setIsQuerying(true)
    setCountdown(30)

    try {
      // 使用 sync_ntp_time：查詢 NTP 並自動套用（有權限時）
      const res = JSON.parse(await invoke<string>('sync_ntp_time', { server: srv.trim() }))
      console.group(`🕐 NTP Sync ${manual ? '(Manual)' : ''}`)
      console.log(JSON.stringify(res, null, 2))
      console.groupEnd()

      if (res.success) {
        // sync_ntp_time 成功時回傳的格式轉換為 NtpResult
        setResult({
          success: true,
          server: res.server,
          server_ip: res.server_ip,
          t1: res.t1,
          t2: res.t2,
          t3: res.t3,
          t4: res.t4,
          offset: res.offset,
          delay: res.delay,
          leap: 0,
          version: 4,
          mode: 4,
          stratum: 1,
          poll: 0,
          precision: 0,
          root_delay: 0,
          root_dispersion: 0,
          ref_id: '',
          ref_time: 0,
        })
      } else {
        console.error('Sync error:', res.error)
        setResult(null)
      }
    } catch (e) {
      console.error('Sync failed:', e)
      setResult(null)
    } finally {
      setIsQuerying(false)
      refs.current.syncing = false
    }
  }

  const checkAutostart = async () => {
    try {
      setAutostart(await isEnabled())
    } catch (e) {
      console.error('Autostart check failed:', e)
    }
  }

  const toggleAutostart = async () => {
    try {
      if (autostart) {
        await disable()
      } else {
        await enable()
      }
      setAutostart(!autostart)
    } catch (e) {
      console.error('Autostart toggle failed:', e)
    }
  }

  useEffect(() => {
    setNow(new Date())
    refs.current.time = setInterval(() => setNow(new Date()), 100)
    checkAutostart()
    return () => clearInterval(refs.current.time)
  }, [])

  useEffect(() => {
    query(server)
    refs.current.sync = setInterval(() => query(server), 30000)
    refs.current.cd = setInterval(() => setCountdown(p => (p <= 1 ? 30 : p - 1)), 1000)
    return () => {
      clearInterval(refs.current.sync)
      clearInterval(refs.current.cd)
    }
  }, [server])

  useEffect(() => {
    let raf: number
    const onMove = (e: MouseEvent) => {
      cancelAnimationFrame(raf)
      raf = requestAnimationFrame(() => setMouse({ x: e.clientX, y: e.clientY }))
    }
    window.addEventListener('mousemove', onMove)
    return () => {
      window.removeEventListener('mousemove', onMove)
      cancelAnimationFrame(raf)
    }
  }, [])

  const { time, date, ms } = formatTime(now)

  return (
    <div className="h-screen bg-gradient-to-br from-background via-background to-muted/10 flex flex-col relative overflow-hidden p-3">
      <div className="absolute top-2 right-2 z-20"><LanguageSwitcher /></div>

      <div className="absolute inset-0 overflow-hidden pointer-events-none z-0">
        <div className="absolute top-1/4 left-1/4 w-32 h-32 bg-primary/5 rounded-full blur-3xl animate-pulse" />
        <div className="absolute bottom-1/4 right-1/4 w-24 h-24 bg-primary/5 rounded-full blur-2xl animate-pulse delay-1000" />
      </div>

      <div className="fixed pointer-events-none z-10" style={{ left: mouse.x - 150, top: mouse.y - 150, width: 300, height: 300 }}>
        <div className="w-full h-full bg-primary/8 rounded-full blur-3xl" />
      </div>

      <div className="relative z-10 flex-1 flex flex-col gap-2 max-w-3xl mx-auto w-full">
        {/* Header */}
        <div className="flex items-center gap-3 bg-card/60 backdrop-blur-md rounded-lg border border-border/50 p-3">
          <div className="relative group shrink-0">
            <div className="absolute inset-0 bg-primary/20 rounded-full blur-md" />
            <div className="relative w-12 h-12 bg-gradient-to-br from-primary/20 to-primary/10 rounded-full flex items-center justify-center border border-primary/20">
              <Clock className="w-6 h-6 text-primary" />
            </div>
          </div>
          <div className="flex-1">
            <h1 className="text-xl font-bold text-foreground">{t('home.title')}</h1>
            <p className="text-xs text-muted-foreground" suppressHydrationWarning>{date}</p>
          </div>
          <div className="text-right">
            <div className="flex items-baseline gap-0.5">
              <span className="text-5xl font-bold tabular-nums text-foreground" suppressHydrationWarning>{time}</span>
              <span className="text-xl font-mono text-muted-foreground" suppressHydrationWarning>.{ms}</span>
            </div>
            {result?.success && (
              <div className="flex items-center justify-end gap-1 text-xs text-muted-foreground mt-1">
                <Timer className="w-3 h-3" /><span>下次同步: {countdown}s</span>
              </div>
            )}
          </div>
        </div>

        {/* Server Selection */}
        <div className="bg-card/60 backdrop-blur-md rounded-lg border border-border/50 p-2">
          <div className="flex gap-2 items-center">
            <Server className="w-3.5 h-3.5 text-primary shrink-0" />
            <select
              value={server}
              onChange={e => setServer(e.target.value)}
              className="flex-1 px-2 py-1 bg-background/50 border border-border/50 rounded text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary/50 transition-all"
            >
              {NTP_SERVERS.map(srv => (
                <option key={srv.value} value={srv.value}>{srv.label} ({srv.value})</option>
              ))}
            </select>
            <button
              onClick={() => query(server, true)}
              disabled={isQuerying || !server.trim()}
              className="px-2.5 py-1 bg-gradient-to-br from-primary/30 to-primary/15 rounded border border-primary/60 hover:from-primary/40 hover:to-primary/20 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-1"
            >
              {isQuerying ? <Loader2 className="w-3 h-3 animate-spin text-primary" /> : <RefreshCw className="w-3 h-3 text-primary" />}
              <span className="text-xs font-medium text-foreground">{t(isQuerying ? 'home.querying' : 'home.query')}</span>
            </button>
          </div>
        </div>

        {/* Result Panel */}
        {result?.success && (
          <div className="flex-1 bg-card/60 backdrop-blur-md rounded-lg border border-border/50 flex flex-col animate-in fade-in duration-300 min-h-0">
            <div className="flex items-center gap-2 text-emerald-500 px-3 py-2 border-b border-border/50 shrink-0">
              <CheckCircle2 className="w-4 h-4" />
              <span className="text-xs font-semibold">同步成功</span>
              <span className="text-xs text-muted-foreground ml-auto">{result.server} ({result.server_ip})</span>
            </div>

            <div className="flex border-b border-border/50 shrink-0">
              {TABS.map(tab => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`flex-1 flex items-center justify-center gap-1.5 px-2 py-2 text-xs font-medium transition-all ${
                    activeTab === tab.id
                      ? 'text-primary border-b-2 border-primary bg-primary/5'
                      : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'
                  }`}
                >
                  <tab.icon className="w-3.5 h-3.5" />
                  {tab.label}
                </button>
              ))}
            </div>

            <div className="flex-1 p-3">
              {activeTab === 'timestamps' && (
                <div className="grid grid-cols-2 gap-2 h-full">
                  <InfoCard label="T1 - 客戶端發送" value={formatTs(result.t1)} sub={`${result.t1.toFixed(3)} ms`} />
                  <InfoCard label="T2 - 伺服器接收" value={formatTs(result.t2)} sub={`${result.t2.toFixed(3)} ms`} />
                  <InfoCard label="T3 - 伺服器發送" value={formatTs(result.t3)} sub={`${result.t3.toFixed(3)} ms`} />
                  <InfoCard label="T4 - 客戶端接收" value={formatTs(result.t4)} sub={`${result.t4.toFixed(3)} ms`} />
                </div>
              )}

              {activeTab === 'results' && (
                <div className="grid grid-cols-2 gap-2 h-full">
                  <InfoCard
                    label="時間偏差 (Offset)"
                    value={<span className={`text-base font-semibold ${getOffsetColor(result.offset)}`}>{result.offset >= 0 ? '+' : ''}{formatOffset(result.offset)}</span>}
                    sub="((T2-T1) + (T3-T4)) / 2"
                  />
                  <InfoCard label="網路延遲 (Delay)" value={`${result.delay.toFixed(3)} ms`} sub="(T4-T1) - (T3-T2)" />
                  <InfoCard label="往返時間 (RTT)" value={`${(result.t4 - result.t1).toFixed(3)} ms`} sub="T4 - T1" />
                  <InfoCard label="根離散度" value={`${result.root_dispersion.toFixed(3)} ms`} />
                </div>
              )}

              {activeTab === 'packet' && (
                <div className="grid grid-cols-2 gap-2 h-full">
                  <InfoCard
                    label="LI/VN/Mode"
                    value={`0x${((result.leap << 6) | (result.version << 3) | result.mode).toString(16)}`}
                    sub={`LI=${result.leap} VN=${result.version} Mode=${result.mode}`}
                  />
                  <InfoCard label="階層 (Stratum)" value={result.stratum} sub={getStratumDesc(result.stratum)} />
                  <InfoCard label="輪詢間隔" value={result.poll} sub={`2^${result.poll} = ${Math.pow(2, result.poll)} s`} />
                  <InfoCard label="精度" value={result.precision} sub={`${(Math.pow(2, result.precision) * 1000).toExponential(2)} ms`} />
                  <InfoCard label="根延遲" value={`${result.root_delay.toFixed(3)} ms`} />
                  <InfoCard label="根離散度" value={`${result.root_dispersion.toFixed(3)} ms`} />
                  <InfoCard label="參考 ID" value={result.ref_id || '-'} />
                  <InfoCard label="參考時間戳" value={result.ref_time ? formatTs(result.ref_time) : '-'} />
                </div>
              )}

              {activeTab === 'compare' && (
                <div className="grid grid-cols-2 gap-2 h-full">
                  <InfoCard label="本地時間 (T4)" value={formatTs(result.t4)} />
                  <InfoCard label="伺服器時間 (T3)" value={formatTs(result.t3)} />
                  <InfoCard label="校正後時間" value={formatTs(result.t4 + result.offset)} />
                  <InfoCard
                    label="時鐘偏移"
                    value={<span className={`font-semibold ${getOffsetColor(result.offset)}`}>{result.offset >= 0 ? '+' : ''}{result.offset.toFixed(3)} ms</span>}
                  />
                </div>
              )}
            </div>

            <div className="flex items-center justify-between px-3 py-2 border-t border-border/50 shrink-0">
              <div className="flex items-center gap-2">
                <Power className="w-4 h-4 text-primary" />
                <span className="text-xs font-medium text-foreground">開機自動啟動</span>
              </div>
              <button
                onClick={toggleAutostart}
                className={`relative w-10 h-5 rounded-full transition-colors duration-200 ${autostart ? 'bg-emerald-500' : 'bg-muted'}`}
              >
                <span className={`absolute top-0.5 w-4 h-4 bg-white rounded-full shadow transition-transform duration-200 ${autostart ? 'translate-x-5' : 'translate-x-0.5'}`} />
              </button>
            </div>
          </div>
        )}

        {result && !result.success && (
          <div className="bg-card/60 backdrop-blur-md rounded-lg border border-border/50 p-2 animate-in fade-in duration-300">
            <div className="flex items-center gap-2 text-red-500">
              <AlertCircle className="w-3.5 h-3.5" />
              <div>
                <p className="text-[10px] font-semibold">{t('home.queryFailed')}</p>
                <p className="text-[10px] text-muted-foreground">請檢查伺服器地址或網路連線</p>
              </div>
            </div>
          </div>
        )}

        {!result && (
          <div className="flex-1 bg-card/60 backdrop-blur-md rounded-lg border border-border/50 flex items-center justify-center">
            <div className="text-center text-muted-foreground">
              <Loader2 className="w-6 h-6 animate-spin mx-auto mb-2" />
              <p className="text-xs">正在同步...</p>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
