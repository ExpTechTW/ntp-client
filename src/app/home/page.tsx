'use client'

import { useState, useEffect, useRef } from 'react'
import { Clock, Server, RefreshCw, CheckCircle2, AlertCircle, Loader2, Timer } from 'lucide-react'
import { invoke } from '@tauri-apps/api/core'
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

const POPULAR_SERVERS = ['pool.ntp.org', 'time.google.com', 'time.cloudflare.com', 'time.windows.com']
const WEEKDAYS = ['日', '一', '二', '三', '四', '五', '六']

const getStratumDesc = (s: number) =>
  s === 0 ? '未指定' : s === 1 ? '主要參考 (GPS/原子鐘)' : s <= 15 ? `次級參考 (第 ${s} 層)` : s === 16 ? '未同步' : '保留'

const getLeapDesc = (l: number) => ['正常', '+1 秒', '-1 秒', '未同步'][l] ?? '未知'

const getOffsetColor = (ms: number) => {
  const abs = Math.abs(ms)
  return abs < 100 ? 'text-emerald-500' : abs < 1000 ? 'text-yellow-500' : 'text-red-500'
}

const formatOffset = (ms: number) => {
  const abs = Math.abs(ms)
  return abs < 1 ? `${(ms * 1000).toFixed(3)} μs` : abs < 1000 ? `${ms.toFixed(3)} ms` : `${(ms / 1000).toFixed(3)} s`
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
  <div className="space-y-1 p-3 bg-muted/30 rounded-lg">
    <p className="text-xs text-muted-foreground">{label}</p>
    <p className="text-sm font-mono text-foreground">{value}</p>
    {sub && <p className="text-xs text-muted-foreground font-mono">{sub}</p>}
  </div>
)

const Section = ({ title, children }: { title: string; children: React.ReactNode }) => (
  <div className="space-y-4">
    <h3 className="text-sm font-semibold text-foreground border-b border-border/50 pb-2">{title}</h3>
    <div className="grid grid-cols-2 gap-4">{children}</div>
  </div>
)

export default function HomePage() {
  const { t } = useTranslation()
  const [server, setServer] = useState('pool.ntp.org')
  const [isQuerying, setIsQuerying] = useState(false)
  const [result, setResult] = useState<NtpResult | null>(null)
  const [now, setNow] = useState(new Date())
  const [countdown, setCountdown] = useState(30)
  const [mouse, setMouse] = useState({ x: 0, y: 0 })
  const refs = useRef<{ time?: NodeJS.Timeout; sync?: NodeJS.Timeout; cd?: NodeJS.Timeout }>({})

  const query = async (srv: string, manual = false) => {
    if (!srv.trim() || isQuerying) return
    setIsQuerying(true)
    setCountdown(30)

    try {
      const res = JSON.parse(await invoke<string>('query_ntp_udp', { server: srv.trim() })) as NtpResult | NtpError
      console.group(`🕐 NTP ${manual ? '(Manual)' : ''}`)
      console.log(JSON.stringify(res, null, 2))
      console.groupEnd()

      if (res.success) setResult(res as NtpResult)
      else {
        console.error('NTP error:', (res as NtpError).error)
        setResult(null)
      }
    } catch (e) {
      console.error('NTP failed:', e)
      setResult(null)
    } finally {
      setIsQuerying(false)
    }
  }

  useEffect(() => {
    setNow(new Date())
    refs.current.time = setInterval(() => setNow(new Date()), 100)
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
    <div className="min-h-screen bg-gradient-to-br from-background via-background to-muted/10 flex items-center justify-center relative overflow-hidden">
      <div className="absolute top-4 right-4 z-20"><LanguageSwitcher /></div>

      <div className="absolute inset-0 overflow-hidden pointer-events-none z-0">
        <div className="absolute top-1/4 left-1/4 w-32 h-32 bg-primary/5 rounded-full blur-3xl animate-pulse" />
        <div className="absolute bottom-1/4 right-1/4 w-24 h-24 bg-primary/5 rounded-full blur-2xl animate-pulse delay-1000" />
      </div>

      <div className="fixed pointer-events-none z-10" style={{ left: mouse.x - 150, top: mouse.y - 150, width: 300, height: 300 }}>
        <div className="w-full h-full bg-primary/8 rounded-full blur-3xl" />
      </div>

      <div className="relative z-10 w-full max-w-6xl px-4 space-y-6 pb-8">
        <div className="text-center space-y-4">
          <div className="flex justify-center">
            <div className="relative group">
              <div className="absolute inset-0 bg-primary/20 rounded-full blur-lg group-hover:bg-primary/30 transition-all duration-300" />
              <div className="relative w-20 h-20 bg-gradient-to-br from-primary/20 to-primary/10 rounded-full flex items-center justify-center border border-primary/20 group-hover:border-primary/30 transition-all duration-300">
                <Clock className="w-10 h-10 text-primary" />
              </div>
            </div>
          </div>
          <h1 className="text-4xl font-bold bg-gradient-to-r from-foreground to-foreground/80 bg-clip-text text-transparent">{t('home.title')}</h1>
          <p className="text-sm text-muted-foreground">{t('home.subtitle')}</p>
        </div>

        <div className="bg-card/60 backdrop-blur-md rounded-2xl border border-border/50 p-8 hover:bg-card/80 hover:border-primary/30 transition-all duration-300 hover:shadow-2xl group relative overflow-hidden">
          <div className="absolute inset-0 bg-gradient-to-br from-primary/5 to-transparent rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
          <div className="relative z-10 text-center space-y-2">
            <p className="text-sm text-muted-foreground mb-4" suppressHydrationWarning>{date}</p>
            <div className="flex items-baseline justify-center gap-2">
              <span className="text-7xl font-bold tabular-nums text-foreground" suppressHydrationWarning>{time}</span>
              <span className="text-2xl font-mono text-muted-foreground" suppressHydrationWarning>.{ms}</span>
            </div>
            <p className="text-xs text-muted-foreground mt-4" suppressHydrationWarning>
              {now.toLocaleTimeString('zh-TW', { timeZoneName: 'short' })}
            </p>
            {result?.success && (
              <div className="mt-4 pt-4 border-t border-border/50 flex items-center justify-center gap-2 text-xs text-muted-foreground">
                <Timer className="w-3 h-3" /><span>下次同步: {countdown} 秒</span>
              </div>
            )}
          </div>
        </div>

        <div className="bg-card/60 backdrop-blur-md rounded-lg border border-border/50 p-4 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Server className="w-4 h-4 text-primary" />
              <label className="text-sm font-semibold text-foreground">{t('home.serverLabel')}</label>
            </div>
            {result?.success && (
              <div className="flex items-center gap-2 text-xs text-emerald-500">
                <CheckCircle2 className="w-3 h-3" /><span>已同步</span>
              </div>
            )}
          </div>
          <div className="flex gap-2">
            <input
              type="text"
              value={server}
              onChange={e => setServer(e.target.value)}
              placeholder="pool.ntp.org"
              className="flex-1 px-3 py-2 bg-background/50 border border-border/50 rounded-lg text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary/50 transition-all"
              onKeyDown={e => e.key === 'Enter' && !isQuerying && query(server, true)}
            />
            <button
              onClick={() => query(server, true)}
              disabled={isQuerying || !server.trim()}
              className="px-4 py-2 bg-gradient-to-br from-primary/30 to-primary/15 backdrop-blur-md rounded-lg border-2 border-primary/60 hover:from-primary/40 hover:to-primary/20 hover:border-primary/80 transition-all duration-300 hover:scale-105 hover:shadow-xl hover:shadow-primary/20 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100 flex items-center gap-2"
            >
              {isQuerying ? <Loader2 className="w-4 h-4 animate-spin text-primary" /> : <RefreshCw className="w-4 h-4 text-primary" />}
              <span className="text-sm font-semibold text-foreground">{t(isQuerying ? 'home.querying' : 'home.query')}</span>
            </button>
          </div>
        </div>

        {result?.success && (
          <div className="bg-card/60 backdrop-blur-md rounded-lg border border-border/50 p-6 space-y-6 animate-in fade-in duration-300">
            <div className="flex items-center gap-2 text-emerald-500">
              <CheckCircle2 className="w-5 h-5" />
              <span className="text-sm font-semibold">NTP 同步成功</span>
              <span className="text-xs text-muted-foreground ml-auto">{result.server} ({result.server_ip})</span>
            </div>

            <Section title="時間戳記">
              <InfoCard label="T1 - 客戶端發送" value={formatTs(result.t1)} sub={`${result.t1.toFixed(3)} ms`} />
              <InfoCard label="T2 - 伺服器接收" value={formatTs(result.t2)} sub={`${result.t2.toFixed(3)} ms`} />
              <InfoCard label="T3 - 伺服器發送" value={formatTs(result.t3)} sub={`${result.t3.toFixed(3)} ms`} />
              <InfoCard label="T4 - 客戶端接收" value={formatTs(result.t4)} sub={`${result.t4.toFixed(3)} ms`} />
            </Section>

            <Section title="計算結果">
              <InfoCard
                label="時間偏差 (Offset)"
                value={<span className={`text-lg font-semibold ${getOffsetColor(result.offset)}`}>{result.offset >= 0 ? '+' : ''}{formatOffset(result.offset)}</span>}
                sub="((T2-T1) + (T3-T4)) / 2"
              />
              <InfoCard label="網路延遲 (Delay)" value={`${result.delay.toFixed(3)} ms`} sub="(T4-T1) - (T3-T2)" />
              <InfoCard label="往返時間 (RTT)" value={`${(result.t4 - result.t1).toFixed(3)} ms`} sub="T4 - T1" />
              <InfoCard label="根離散度" value={`${result.root_dispersion.toFixed(3)} ms`} />
            </Section>

            <Section title="封包資訊">
              <InfoCard
                label="LI/VN/Mode"
                value={`0x${((result.leap << 6) | (result.version << 3) | result.mode).toString(16)}`}
                sub={`LI=${result.leap} (${getLeapDesc(result.leap)}) VN=${result.version} Mode=${result.mode}`}
              />
              <InfoCard label="階層 (Stratum)" value={result.stratum} sub={getStratumDesc(result.stratum)} />
              <InfoCard label="輪詢間隔 (Poll)" value={result.poll} sub={`2^${result.poll} = ${Math.pow(2, result.poll)} s`} />
              <InfoCard label="精度 (Precision)" value={result.precision} sub={`${(Math.pow(2, result.precision) * 1000).toExponential(2)} ms`} />
              <InfoCard label="根延遲" value={`${result.root_delay.toFixed(3)} ms`} />
              <InfoCard label="根離散度" value={`${result.root_dispersion.toFixed(3)} ms`} />
              <InfoCard label="參考 ID" value={result.ref_id} />
              <InfoCard label="參考時間戳" value={formatTs(result.ref_time)} />
            </Section>

            <div className="space-y-4 pt-4 border-t border-border/50">
              <h3 className="text-sm font-semibold text-foreground">時間對比</h3>
              <div className="grid grid-cols-2 gap-4">
                <InfoCard label="本地時間 (T4)" value={formatTs(result.t4)} />
                <InfoCard label="伺服器時間 (T3)" value={formatTs(result.t3)} />
                <InfoCard label="校正後時間" value={formatTs(result.t4 + result.offset)} />
                <InfoCard
                  label="時鐘偏移"
                  value={<span className={`font-semibold ${getOffsetColor(result.offset)}`}>{result.offset >= 0 ? '+' : ''}{result.offset.toFixed(3)} ms</span>}
                />
              </div>
            </div>
          </div>
        )}

        {result && !result.success && (
          <div className="bg-card/60 backdrop-blur-md rounded-lg border border-border/50 p-4 animate-in fade-in duration-300">
            <div className="flex items-center gap-2 text-red-500">
              <AlertCircle className="w-5 h-5" />
              <div>
                <p className="text-sm font-semibold">{t('home.queryFailed')}</p>
                <p className="text-xs text-muted-foreground mt-1">請檢查伺服器地址或網路連線</p>
              </div>
            </div>
          </div>
        )}

        <div className="bg-card/60 backdrop-blur-md rounded-lg border border-border/50 p-4">
          <p className="text-xs text-muted-foreground mb-2">{t('home.popularServers')}</p>
          <div className="flex flex-wrap gap-2">
            {POPULAR_SERVERS.map(srv => (
              <button
                key={srv}
                onClick={() => { setServer(srv); if (!isQuerying) query(srv, true) }}
                className="px-3 py-1 text-xs bg-muted/50 hover:bg-muted/80 rounded-md transition-all hover:scale-105 text-foreground"
              >
                {srv}
              </button>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}
