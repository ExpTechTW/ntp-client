'use client'

import { useState, useEffect, useMemo, useRef, useCallback } from 'react'
import { ArrowLeft, Sun, Moon, Trash2, TrendingUp, TrendingDown, Minus, BarChart3, LineChart, Activity, Clock, Zap, Target, Gauge, AlertTriangle, CheckCircle, Timer, Sigma, Hash, ArrowUpDown, GitCompare, HelpCircle, Waves, Brain, Shuffle, Scale, Flame, Wind, Crosshair, Layers, Radio, Compass, Sparkles, Database, Calendar, RefreshCw } from 'lucide-react'
import { useRouter } from 'next/navigation'
import { useTranslation } from 'react-i18next'
import '@/i18n'
import { invoke } from '@tauri-apps/api/core'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip as ChartTooltip,
  Legend,
  Filler,
} from 'chart.js'
import { Line, Bar } from 'react-chartjs-2'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  ChartTooltip,
  Legend,
  Filler
)

interface NtpRecord {
  id?: number
  offset: number
  delay: number
  server: string
  timestamp: number
}

interface DbStats {
  total_records: number
  earliest_timestamp?: number
  latest_timestamp?: number
  servers: string[]
  db_size_bytes: number
}

type TimeRange = 'all' | '1h' | '6h' | '24h' | '7d' | '30d' | 'custom'

interface HistoryEntry {
  offset: number
  delay: number
  server: string
  time: Date
}

interface StatsResult {
  samples: number
  mean: number
  mean_delay: number
  median: number
  median_delay: number
  mode: number
  mode_delay: number
  geometric_mean: number
  harmonic_mean: number
  trimmed_mean: number
  trimmed_mean_delay: number
  sum: number
  sum_abs: number
  sum_delay: number
  min: number
  max: number
  min_delay: number
  max_delay: number
  variance: number
  variance_delay: number
  std_dev: number
  std_dev_delay: number
  cv: number
  cv_delay: number
  mad: number
  mad_delay: number
  aad: number
  aad_delay: number
  iqr: number
  iqr_delay: number
  sem: number
  sem_delay: number
  rms: number
  rms_delay: number
  mse: number
  mse_delay: number
  range: number
  range_delay: number
  skewness: number
  skewness_delay: number
  kurtosis: number
  kurtosis_delay: number
  skewness_abs: number
  kurtosis_abs: number
  skewness_type: string
  kurtosis_type: string
  p1: number
  p5: number
  p10: number
  p25: number
  p50: number
  p75: number
  p90: number
  p95: number
  p99: number
  p1_delay: number
  p5_delay: number
  p10_delay: number
  p25_delay: number
  p50_delay: number
  p75_delay: number
  p90_delay: number
  p95_delay: number
  p99_delay: number
  slope: number
  slope_delay: number
  intercept: number
  intercept_delay: number
  r2: number
  r2_delay: number
  trend_direction: string
  delay_trend_direction: string
  drift_rate: number
  drift_total: number
  recent_avg: number
  older_avg: number
  autocorr1: number
  autocorr2: number
  autocorr3: number
  autocorr5: number
  autocorr10: number
  dw: number
  dw_delay: number
  dw_status: string
  outliers: number
  outliers_delay: number
  outlier_pct: number
  outlier_pct_delay: number
  z_outliers: number
  z_outliers_delay: number
  z_outlier_pct: number
  z_outlier_pct_delay: number
  allan: number
  allan_delay: number
  mtie: number
  mtie_delay: number
  tdev: number
  tdev_delay: number
  jitter: number
  jitter_delay: number
  approx_ent: number
  approx_ent_delay: number
  sample_ent: number
  sample_ent_delay: number
  perm_ent: number
  perm_ent_delay: number
  hurst: number
  hurst_delay: number
  max_pos: number
  max_neg: number
  pos_count: number
  neg_count: number
  crossings: number
  crossing_rate: number
  balance: number
  bias: string
  longest_streak: number
  ci95_lower: number
  ci95_upper: number
  ci95_margin: number
  ci99_lower: number
  ci99_upper: number
  ci99_margin: number
  within_1ms: number
  within_5ms: number
  within_10ms: number
  within_25ms: number
  within_50ms: number
  within_100ms: number
  within_500ms: number
  within_1s: number
  stability: string
  moment3: number
  moment4: number
  moment5: number
  moment6: number
  winsorized_mean: number
  biweight_mean: number
  midrange: number
  midhinge: number
  trimean: number
  gini_mean_diff: number
  quartile_dev: number
  decile_range: number
  interdecile_range: number
  bowley_skewness: number
  pearson_skewness: number
  excess_kurtosis: number
  hadamard_variance: number
  modified_allan: number
  theo1: number
  total_variance: number
  lag1_diff_mean: number
  lag1_diff_std: number
  runs_test_z: number
  turning_points: number
  turning_point_rate: number
  spectral_flatness: number
  spectral_entropy: number
  dominant_freq: number
  shannon_entropy: number
  renyi_entropy: number
  tsallis_entropy: number
  jarque_bera: number
  dagostino_k2: number
  shapiro_wilk_approx: number
  cohens_d: number
  hedges_g: number
  glass_delta: number
  rolling_mean_5: number
  rolling_std_5: number
  rolling_mean_10: number
  rolling_std_10: number
  ewma: number
  peak_to_peak: number
  crest_factor: number
  impulse_factor: number
  shape_factor: number
  clearance_factor: number
  trend_strength: number
  seasonality_strength: number
  stl_trend: number
  stl_remainder_var: number
  partial_autocorr1: number
  partial_autocorr2: number
  ljung_box_q: number
  box_pierce_q: number
  lyapunov_approx: number
  correlation_dim: number
  recurrence_rate: number
  determinism: number
  embedding_dim: number
  false_nearest: number
  average_mutual_info: number
  largest_lyapunov: number
  kaplan_yorke_dim: number
  steady_state_mean: number
  steady_state_var: number
  transient_length: number
  forecast_1: number
  forecast_5: number
  forecast_10: number
  prediction_interval_lower: number
  prediction_interval_upper: number
}

const fmtMs = (ms: number) => `${ms.toFixed(2)}ms`
const fmtNum = (n: number, decimals = 3) => n.toFixed(decimals)
const fmtPct = (n: number) => `${n.toFixed(1)}%`
const fmtTime = (date: Date) =>
  `${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}:${String(date.getSeconds()).padStart(2, '0')}`

const StatCard = ({
  label,
  value,
  sub,
  icon: Icon,
  trend: trendValue,
  isDark,
  color,
  help
}: {
  label: string
  value: React.ReactNode
  sub?: string
  icon?: React.ComponentType<{ className?: string }>
  trend?: 'up' | 'down' | 'stable'
  isDark: boolean
  color?: string
  help?: string
}) => {
  const [showHelp, setShowHelp] = useState(false)
  const helpRef = useRef<HTMLDivElement>(null)
  const [tooltipStyle, setTooltipStyle] = useState<React.CSSProperties>({})

  const handleMouseEnter = () => {
    if (helpRef.current) {
      const rect = helpRef.current.getBoundingClientRect()
      const tooltipWidth = 192
      const tooltipHeight = 80
      const viewportWidth = window.innerWidth
      const viewportHeight = window.innerHeight
      const margin = 8

      let left: number | undefined
      let right: number | undefined
      let top: number | undefined
      let bottom: number | undefined

      if (rect.right + tooltipWidth + margin > viewportWidth) {
        right = 0
      } else {
        left = 0
      }

      if (rect.bottom + tooltipHeight + margin > viewportHeight) {
        bottom = rect.height + 4
        top = undefined
      } else {
        top = rect.height + 4
        bottom = undefined
      }

      setTooltipStyle({ left, right, top, bottom })
    }
    setShowHelp(true)
  }

  return (
    <div
      className={`relative rounded p-2 ${isDark ? 'bg-zinc-800/50' : 'bg-white border border-zinc-200'}`}
      onMouseLeave={() => setShowHelp(false)}
    >
      <div className="flex items-center justify-between mb-0.5">
        <p className={`text-[9px] ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`}>{label}</p>
        <div className="flex items-center gap-0.5">
          {trendValue && (
            trendValue === 'up' ? <TrendingUp className="w-2.5 h-2.5 text-red-400" /> :
            trendValue === 'down' ? <TrendingDown className="w-2.5 h-2.5 text-green-400" /> :
            <Minus className="w-2.5 h-2.5 text-zinc-500" />
          )}
          {Icon && <Icon className={`w-2.5 h-2.5 ${isDark ? 'text-zinc-600' : 'text-zinc-400'}`} />}
          {help && (
            <div
              ref={helpRef}
              className="relative"
              onMouseEnter={handleMouseEnter}
              onMouseLeave={() => setShowHelp(false)}
            >
              <HelpCircle className={`w-2.5 h-2.5 ml-0.5 cursor-help ${isDark ? 'text-zinc-600 hover:text-zinc-400' : 'text-zinc-400 hover:text-zinc-600'}`} />
              {showHelp && (
                <div
                  style={tooltipStyle}
                  className={`absolute z-50 p-2 rounded text-[9px] w-48 ${isDark ? 'bg-zinc-700 text-zinc-300' : 'bg-zinc-100 text-zinc-700'} shadow-lg border ${isDark ? 'border-zinc-600' : 'border-zinc-300'}`}
                >
                  {help}
                </div>
              )}
            </div>
          )}
        </div>
      </div>
      <p className={`text-xs font-mono font-medium ${color || (isDark ? 'text-zinc-200' : 'text-zinc-800')}`}>{value}</p>
      {sub && <p className={`text-[8px] mt-0.5 ${isDark ? 'text-zinc-600' : 'text-zinc-400'}`}>{sub}</p>}
    </div>
  )
}

type StatCardConfig = {
  labelKey: string
  getValue: (stats: StatsResult) => number | string | React.ReactNode
  getSub?: (stats: StatsResult, t: any) => string | React.ReactNode
  icon?: React.ComponentType<{ className?: string }>
  getTrend?: (stats: StatsResult) => 'up' | 'down' | 'stable' | undefined
  getColor?: (stats: StatsResult) => string | undefined
  helpKey?: string
  format?: 'ms' | 'num' | 'pct' | 'raw'
  decimals?: number
}

type SectionConfig = {
  sectionKey: string
  icon: React.ComponentType<{ className?: string }>
  gridCols?: string
  cards: StatCardConfig[]
}

const SectionTitle = ({ icon: Icon, children, isDark }: { icon: React.ComponentType<{ className?: string }>; children: React.ReactNode; isDark: boolean }) => (
  <h2 className={`text-[11px] font-medium mb-1.5 flex items-center gap-1 ${isDark ? 'text-zinc-400' : 'text-zinc-600'}`}>
    <Icon className="w-3 h-3" />
    {children}
  </h2>
)

const StatSection = ({ config, stats, t, isDark, getHelp, fmtMs, fmtNum, fmtPct }: {
  config: SectionConfig
  stats: StatsResult
  t: any
  isDark: boolean
  getHelp: (key: string) => string | undefined
  fmtMs: (val: number) => string
  fmtNum: (val: number, decimals?: number) => string
  fmtPct: (val: number) => string
}) => {
  const formatValue = (val: number | string | React.ReactNode, format?: string, decimals?: number): React.ReactNode => {
    if (typeof val !== 'number') return val
    if (format === 'ms') return fmtMs(val)
    if (format === 'pct') return fmtPct(val)
    if (format === 'num') return fmtNum(val, decimals)
    if (format === 'raw') return val.toString()
    return val.toString()
  }

  return (
    <section>
      <SectionTitle icon={config.icon} isDark={isDark}>{t(`history.sections.${config.sectionKey}`)}</SectionTitle>
      <div className={`grid ${config.gridCols || 'grid-cols-3 sm:grid-cols-6'} gap-1`}>
        {config.cards.map((card, idx) => {
          const value = card.getValue(stats)
          const formattedValue = card.format ? formatValue(value as number, card.format, card.decimals) : value
          const sub = card.getSub ? card.getSub(stats, t) : undefined
          const trend = card.getTrend ? card.getTrend(stats) : undefined
          const color = card.getColor ? card.getColor(stats) : undefined
          const label = card.labelKey.startsWith('P') ? card.labelKey : t(`history.stats.${card.labelKey}`)
          
          return (
            <StatCard
              key={idx}
              label={label}
              value={formattedValue}
              sub={typeof sub === 'string' ? sub : undefined}
              icon={card.icon}
              trend={trend}
              color={color}
              help={card.helpKey ? getHelp(card.helpKey) : undefined}
              isDark={isDark}
            />
          )
        })}
      </div>
    </section>
  )
}

export default function HistoryPage() {
  const { t, i18n } = useTranslation()
  const router = useRouter()
  const [isDark, setIsDark] = useState(true)
  const [stats, setStats] = useState<StatsResult | null>(null)
  const [autocorrData, setAutocorrData] = useState<number[]>([])
  const [history, setHistory] = useState<HistoryEntry[]>([])
  const [dbStats, setDbStats] = useState<DbStats | null>(null)
  const [timeRange, setTimeRange] = useState<TimeRange>('all')
  const [isLoading, setIsLoading] = useState(true)

  const getTimeRangeStart = useCallback((range: TimeRange): number | null => {
    const now = Date.now()
    switch (range) {
      case '1h': return now - 60 * 60 * 1000
      case '6h': return now - 6 * 60 * 60 * 1000
      case '24h': return now - 24 * 60 * 60 * 1000
      case '7d': return now - 7 * 24 * 60 * 60 * 1000
      case '30d': return now - 30 * 24 * 60 * 60 * 1000
      case 'all':
      default: return null
    }
  }, [])

  const loadData = useCallback(async (range: TimeRange) => {
    setIsLoading(true)
    try {
      const startTime = getTimeRangeStart(range)
      let records: NtpRecord[]

      if (startTime === null) {
        records = await invoke<NtpRecord[]>('db_query', {
          startTime: null,
          endTime: null,
          server: null,
          minOffset: null,
          maxOffset: null,
          minDelay: null,
          maxDelay: null,
          limit: null,
          offset: null,
          orderDesc: false
        })
      } else {
        records = await invoke<NtpRecord[]>('db_query_range', {
          start: startTime,
          end: Date.now()
        })
      }

      const entries: HistoryEntry[] = records.map(r => ({
        offset: r.offset,
        delay: r.delay,
        server: r.server,
        time: new Date(r.timestamp)
      }))

      setHistory(entries)

      const stats = await invoke<DbStats>('db_get_stats')
      setDbStats(stats)
    } catch (err) {
      console.error('Failed to load data from database:', err)
      setHistory([])
    } finally {
      setIsLoading(false)
    }
  }, [getTimeRangeStart])

  useEffect(() => {
    const savedTheme = localStorage.getItem('theme')
    if (savedTheme) setIsDark(savedTheme === 'dark')
    loadData(timeRange)
  }, [])

  useEffect(() => {
    loadData(timeRange)
  }, [timeRange, loadData])

  const clearHistory = async () => {
    try {
      await invoke('db_clear')
      setHistory([])
      setStats(null)
      setDbStats(null)
    } catch (err) {
      console.error('Failed to clear database:', err)
    }
  }

  useEffect(() => {
    if (history.length < 2) {
      setStats(null)
      setAutocorrData([])
      return
    }

    const historyData = history.map((h: HistoryEntry) => ({
      offset: h.offset,
      delay: h.delay,
      server: h.server,
      timestamp: h.time.getTime()
    }))

    invoke<StatsResult | null>('calculate_history_stats', { history: historyData })
      .then(result => setStats(result))
      .catch(err => console.error('Failed to calculate stats:', err))

    if (history.length >= 20) {
      invoke<number[]>('calculate_autocorr_data', { history: historyData, maxLag: 20 })
        .then(result => setAutocorrData(result))
        .catch(err => console.error('Failed to calculate autocorr:', err))
    }
  }, [history])

  const toggleTheme = () => {
    const newTheme = !isDark
    setIsDark(newTheme)
    localStorage.setItem('theme', newTheme ? 'dark' : 'light')
  }

  const getHelp = (key: string) => {
    const desc = t(`history.descriptions.${key}`, { defaultValue: '' })
    return desc || undefined
  }

  const percentileKeys = ['p1', 'p5', 'p10', 'p25', 'p50', 'p75', 'p90', 'p95', 'p99'] as const
  const percentileLabels = ['P1', 'P5', 'P10', 'P25', 'P50', 'P75', 'P90', 'P95', 'P99']
  const percentileSubs: (string | undefined)[] = [undefined, undefined, undefined, t('history.subs.q1'), t('history.stats.median'), t('history.subs.q3'), undefined, undefined, undefined]

  const sectionsConfig: SectionConfig[] = useMemo(() => stats ? [
    {
      sectionKey: 'overview',
      icon: Activity,
      cards: [
        { labelKey: 'samples', getValue: () => stats.samples, icon: Hash },
        { labelKey: 'meanOffset', getValue: () => stats.mean, format: 'ms', getTrend: () => stats.trend_direction as 'up' | 'down' | 'stable', helpKey: 'mean' },
        { labelKey: 'meanDelay', getValue: () => stats.mean_delay, format: 'ms', icon: Timer },
        { labelKey: 'stability', getValue: () => <span className={getStabilityColor(stats.stability)}>{t(`history.stability.${stats.stability}`)}</span>, icon: Gauge, helpKey: 'stability' },
        { labelKey: 'stdDev', getValue: () => stats.std_dev, format: 'ms', helpKey: 'stdDev' },
        { labelKey: 'rms', getValue: () => stats.rms, format: 'ms', helpKey: 'rms' },
      ]
    },
    {
      sectionKey: 'centralTendency',
      icon: Crosshair,
      cards: [
        { labelKey: 'mean', getValue: () => stats.mean, format: 'ms', helpKey: 'mean' },
        { labelKey: 'median', getValue: () => stats.median, format: 'ms', helpKey: 'median' },
        { labelKey: 'mode', getValue: () => stats.mode, format: 'ms', helpKey: 'mode' },
        { labelKey: 'geoMean', getValue: () => stats.geometric_mean, format: 'ms', helpKey: 'geometricMean' },
        { labelKey: 'harmonic', getValue: () => stats.harmonic_mean, format: 'ms', helpKey: 'harmonicMean' },
        { labelKey: 'trimmed', getValue: () => stats.trimmed_mean, format: 'ms', getSub: () => t('history.subs.10pctTrim'), helpKey: 'trimmedMean' },
      ]
    },
    {
      sectionKey: 'dispersion',
      icon: Sigma,
      cards: [
        { labelKey: 'variance', getValue: () => `${fmtNum(stats.variance, 2)}ms²`, helpKey: 'variance' },
        { labelKey: 'stdDev', getValue: () => stats.std_dev, format: 'ms', helpKey: 'stdDev' },
        { labelKey: 'cv', getValue: () => stats.cv, format: 'pct', helpKey: 'cv' },
        { labelKey: 'mad', getValue: () => stats.mad, format: 'ms', helpKey: 'mad' },
        { labelKey: 'aad', getValue: () => stats.aad, format: 'ms', helpKey: 'aad' },
        { labelKey: 'iqr', getValue: () => stats.iqr, format: 'ms', helpKey: 'iqr' },
        { labelKey: 'sem', getValue: () => stats.sem, format: 'ms', helpKey: 'sem' },
        { labelKey: 'mse', getValue: () => `${fmtNum(stats.mse, 2)}ms²`, helpKey: 'mse' },
        { labelKey: 'range', getValue: () => stats.range, format: 'ms', helpKey: 'range' },
        { labelKey: 'min', getValue: () => stats.min, format: 'ms' },
        { labelKey: 'max', getValue: () => stats.max, format: 'ms' },
        { labelKey: 'sumAbs', getValue: () => stats.sum_abs, format: 'ms' },
      ]
    },
    {
      sectionKey: 'shape',
      icon: Waves,
      cards: [
        { labelKey: 'skewness', getValue: () => stats.skewness, format: 'num', getSub: () => stats.skewness_type, helpKey: 'skewness' },
        { labelKey: 'kurtosis', getValue: () => stats.kurtosis, format: 'num', getSub: () => stats.kurtosis_type, helpKey: 'kurtosis' },
        { labelKey: 'skewDelay', getValue: () => stats.skewness_delay, format: 'num' },
        { labelKey: 'kurtDelay', getValue: () => stats.kurtosis_delay, format: 'num' },
        { labelKey: 'skewAbs', getValue: () => stats.skewness_abs, format: 'num' },
        { labelKey: 'kurtAbs', getValue: () => stats.kurtosis_abs, format: 'num' },
      ]
    },
    {
      sectionKey: 'offsetPercentiles',
      icon: Target,
      gridCols: 'grid-cols-3 sm:grid-cols-9',
      cards: percentileKeys.map((key, i) => ({
        labelKey: percentileLabels[i],
        getValue: () => stats[key],
        format: 'ms' as const,
        getSub: () => percentileSubs[i],
      }))
    },
    {
      sectionKey: 'delayPercentiles',
      icon: Timer,
      gridCols: 'grid-cols-3 sm:grid-cols-9',
      cards: percentileKeys.map((key, i) => ({
        labelKey: percentileLabels[i],
        getValue: () => stats[`${key}_delay` as keyof StatsResult] as number,
        format: 'ms' as const,
        getSub: () => percentileSubs[i],
      }))
    },
    {
      sectionKey: 'trendAnalysis',
      icon: TrendingUp,
      cards: [
        { labelKey: 'slope', getValue: () => `${stats.slope >= 0 ? '+' : ''}${fmtNum(stats.slope, 4)}`, format: 'raw', getSub: () => t('history.subs.msPerSample'), getTrend: () => stats.trend_direction as 'up' | 'down' | 'stable', helpKey: 'slope' },
        { labelKey: 'r2', getValue: () => stats.r2, format: 'num', decimals: 4, helpKey: 'r2' },
        { labelKey: 'intercept', getValue: () => stats.intercept, format: 'ms' },
        { labelKey: 'driftRate', getValue: () => stats.drift_rate, format: 'num', decimals: 4, getSub: () => t('history.subs.msPerSample'), helpKey: 'drift' },
        { labelKey: 'totalDrift', getValue: () => stats.drift_total, format: 'ms' },
        { labelKey: 'recentAvg', getValue: () => stats.recent_avg, format: 'ms', getSub: () => t('history.subs.last10') },
        { labelKey: 'slopeDelay', getValue: () => `${stats.slope_delay >= 0 ? '+' : ''}${fmtNum(stats.slope_delay, 4)}`, format: 'raw', getTrend: () => stats.delay_trend_direction as 'up' | 'down' | 'stable' },
        { labelKey: 'r2Delay', getValue: () => stats.r2_delay, format: 'num', decimals: 4 },
        { labelKey: 'olderAvg', getValue: () => stats.older_avg, format: 'ms' },
      ]
    },
    {
      sectionKey: 'autocorrelation',
      icon: Radio,
      cards: [
        { labelKey: 'acf1', getValue: () => stats.autocorr1, format: 'num', helpKey: 'autocorr' },
        { labelKey: 'acf2', getValue: () => stats.autocorr2, format: 'num' },
        { labelKey: 'acf3', getValue: () => stats.autocorr3, format: 'num' },
        { labelKey: 'acf5', getValue: () => stats.autocorr5, format: 'num' },
        { labelKey: 'acf10', getValue: () => stats.autocorr10, format: 'num' },
        { labelKey: 'durbinWatson', getValue: () => stats.dw, format: 'num', decimals: 3, getSub: () => stats.dw_status, helpKey: 'dw' },
      ]
    },
    {
      sectionKey: 'outliers',
      icon: AlertTriangle,
      cards: [
        { labelKey: 'iqrOutliers', getValue: () => stats.outliers, format: 'raw', getSub: () => fmtPct(stats.outlier_pct), getColor: () => stats.outliers > 0 ? 'text-yellow-500' : 'text-emerald-500', helpKey: 'outliers' },
        { labelKey: 'zOutliers', getValue: () => stats.z_outliers, format: 'raw', getSub: () => fmtPct(stats.z_outlier_pct), getColor: () => stats.z_outliers > 0 ? 'text-yellow-500' : 'text-emerald-500', helpKey: 'zOutliers' },
        { labelKey: 'iqrDelay', getValue: () => stats.outliers_delay, format: 'raw', getSub: () => fmtPct(stats.outlier_pct_delay) },
        { labelKey: 'zDelay', getValue: () => stats.z_outliers_delay, format: 'raw', getSub: () => fmtPct(stats.z_outlier_pct_delay) },
        { labelKey: 'iqrWidth', getValue: () => stats.iqr, format: 'ms' },
        { labelKey: 'fence', getValue: () => `${fmtNum(stats.p25 - 1.5 * stats.iqr, 1)}/${fmtNum(stats.p75 + 1.5 * stats.iqr, 1)}` },
      ]
    },
    {
      sectionKey: 'stability',
      icon: Gauge,
      cards: [
        { labelKey: 'allanDev', getValue: () => stats.allan, format: 'ms', helpKey: 'allan' },
        { labelKey: 'mtie', getValue: () => stats.mtie, format: 'ms', helpKey: 'mtie' },
        { labelKey: 'tdev', getValue: () => stats.tdev, format: 'ms', helpKey: 'tdev' },
        { labelKey: 'jitter', getValue: () => stats.jitter, format: 'ms', helpKey: 'jitter' },
        { labelKey: 'allanDelay', getValue: () => stats.allan_delay, format: 'ms' },
        { labelKey: 'jitterDelay', getValue: () => stats.jitter_delay, format: 'ms' },
      ]
    },
    {
      sectionKey: 'complexity',
      icon: Brain,
      cards: [
        { labelKey: 'approxEnt', getValue: () => stats.approx_ent, format: 'num', helpKey: 'approxEnt' },
        { labelKey: 'sampleEnt', getValue: () => stats.sample_ent, format: 'num', helpKey: 'sampleEnt' },
        { labelKey: 'permEnt', getValue: () => stats.perm_ent, format: 'num', helpKey: 'permEnt' },
        { labelKey: 'hurst', getValue: () => stats.hurst, format: 'num', getSub: () => stats.hurst > 0.6 ? t('history.labels.persistent') : stats.hurst < 0.4 ? t('history.labels.antiPersist') : t('history.labels.random'), helpKey: 'hurst' },
        { labelKey: 'apEnDelay', getValue: () => stats.approx_ent_delay, format: 'num' },
        { labelKey: 'hurstDelay', getValue: () => stats.hurst_delay, format: 'num' },
      ]
    },
    {
      sectionKey: 'consecutive',
      icon: GitCompare,
      cards: [
        { labelKey: 'maxPosStreak', getValue: () => stats.max_pos, format: 'raw' },
        { labelKey: 'maxNegStreak', getValue: () => stats.max_neg, format: 'raw' },
        { labelKey: 'longest', getValue: () => stats.longest_streak, format: 'raw' },
        { labelKey: 'posCount', getValue: () => stats.pos_count, format: 'raw', getSub: () => fmtPct(stats.balance * 100) },
        { labelKey: 'negCount', getValue: () => stats.neg_count, format: 'raw', getSub: () => fmtPct((1 - stats.balance) * 100) },
        { labelKey: 'crossings', getValue: () => stats.crossings, format: 'raw', getSub: () => `${t('history.subs.rate')}: ${fmtNum(stats.crossing_rate, 3)}`, helpKey: 'crossings' },
        { labelKey: 'bias', getValue: () => stats.bias, format: 'raw', getColor: () => stats.bias === 'Balanced' ? 'text-emerald-500' : 'text-yellow-500' },
        { labelKey: 'balance', getValue: () => `${fmtPct(stats.balance * 100)} / ${fmtPct((1 - stats.balance) * 100)}`, getSub: () => t('history.subs.plusMinus') },
      ]
    },
    {
      sectionKey: 'confidence',
      icon: Scale,
      cards: [
        { labelKey: 'ci95Lower', getValue: () => stats.ci95_lower, format: 'ms', helpKey: 'ci' },
        { labelKey: 'ci95Upper', getValue: () => stats.ci95_upper, format: 'ms' },
        { labelKey: 'ci95Margin', getValue: () => `±${fmtMs(stats.ci95_margin)}` },
        { labelKey: 'ci99Lower', getValue: () => stats.ci99_lower, format: 'ms' },
        { labelKey: 'ci99Upper', getValue: () => stats.ci99_upper, format: 'ms' },
        { labelKey: 'ci99Margin', getValue: () => `±${fmtMs(stats.ci99_margin)}` },
      ]
    },
    {
      sectionKey: 'accuracy',
      icon: CheckCircle,
      gridCols: 'grid-cols-4 sm:grid-cols-8',
      cards: [
        { labelKey: 'within1ms', getValue: () => stats.within_1ms, format: 'pct', getColor: () => stats.within_1ms > 50 ? 'text-emerald-500' : undefined },
        { labelKey: 'within5ms', getValue: () => stats.within_5ms, format: 'pct', getColor: () => stats.within_5ms > 80 ? 'text-emerald-500' : undefined },
        { labelKey: 'within10ms', getValue: () => stats.within_10ms, format: 'pct', getColor: () => stats.within_10ms > 90 ? 'text-emerald-500' : undefined },
        { labelKey: 'within25ms', getValue: () => stats.within_25ms, format: 'pct' },
        { labelKey: 'within50ms', getValue: () => stats.within_50ms, format: 'pct' },
        { labelKey: 'within100ms', getValue: () => stats.within_100ms, format: 'pct' },
        { labelKey: 'within500ms', getValue: () => stats.within_500ms, format: 'pct' },
        { labelKey: 'within1s', getValue: () => stats.within_1s, format: 'pct' },
      ]
    },
    {
      sectionKey: 'delayStats',
      icon: Wind,
      cards: [
        { labelKey: 'delayMean', getValue: () => stats.mean_delay, format: 'ms' },
        { labelKey: 'delayMedian', getValue: () => stats.median_delay, format: 'ms' },
        { labelKey: 'delayMode', getValue: () => stats.mode_delay, format: 'ms' },
        { labelKey: 'delayStdDev', getValue: () => stats.std_dev_delay, format: 'ms' },
        { labelKey: 'delayCv', getValue: () => stats.cv_delay, format: 'pct' },
        { labelKey: 'delayMad', getValue: () => stats.mad_delay, format: 'ms' },
        { labelKey: 'delayIqr', getValue: () => stats.iqr_delay, format: 'ms' },
        { labelKey: 'delayRms', getValue: () => stats.rms_delay, format: 'ms' },
        { labelKey: 'delayRange', getValue: () => stats.range_delay, format: 'ms' },
        { labelKey: 'delayMin', getValue: () => stats.min_delay, format: 'ms' },
        { labelKey: 'delayMax', getValue: () => stats.max_delay, format: 'ms' },
        { labelKey: 'delayVariance', getValue: () => `${fmtNum(stats.variance_delay, 2)}ms²` },
      ]
    },
    {
      sectionKey: 'robust',
      icon: Layers,
      cards: [
        { labelKey: 'winsorized', getValue: () => stats.winsorized_mean, format: 'ms', helpKey: 'winsorized' },
        { labelKey: 'biweight', getValue: () => stats.biweight_mean, format: 'ms', helpKey: 'biweight' },
        { labelKey: 'midrange', getValue: () => stats.midrange, format: 'ms' },
        { labelKey: 'midhinge', getValue: () => stats.midhinge, format: 'ms' },
        { labelKey: 'trimean', getValue: () => stats.trimean, format: 'ms', helpKey: 'trimean' },
        { labelKey: 'giniDiff', getValue: () => stats.gini_mean_diff, format: 'ms', helpKey: 'gini' },
        { labelKey: 'quartileDev', getValue: () => stats.quartile_dev, format: 'ms' },
        { labelKey: 'decileRange', getValue: () => stats.decile_range, format: 'ms' },
        { labelKey: 'interdecile', getValue: () => stats.interdecile_range, format: 'ms' },
      ]
    },
    {
      sectionKey: 'higherOrder',
      icon: Sigma,
      cards: [
        { labelKey: 'moment3', getValue: () => stats.moment3, format: 'num', decimals: 2 },
        { labelKey: 'moment4', getValue: () => stats.moment4, format: 'num', decimals: 2 },
        { labelKey: 'moment5', getValue: () => stats.moment5, format: 'num', decimals: 2 },
        { labelKey: 'moment6', getValue: () => stats.moment6, format: 'num', decimals: 2 },
        { labelKey: 'bowleySkew', getValue: () => stats.bowley_skewness, format: 'num', helpKey: 'bowley' },
        { labelKey: 'pearsonSkew', getValue: () => stats.pearson_skewness, format: 'num', helpKey: 'pearsonSkew' },
      ]
    },
    {
      sectionKey: 'advancedStability',
      icon: Gauge,
      cards: [
        { labelKey: 'hadamardVar', getValue: () => stats.hadamard_variance, format: 'num', decimals: 4, helpKey: 'hadamard' },
        { labelKey: 'modifiedAllan', getValue: () => stats.modified_allan, format: 'ms', helpKey: 'modifiedAllan' },
        { labelKey: 'theo1', getValue: () => stats.theo1, format: 'ms' },
        { labelKey: 'totalVar', getValue: () => stats.total_variance, format: 'num', decimals: 2 },
      ]
    },
    {
      sectionKey: 'timeSeries',
      icon: Activity,
      cards: [
        { labelKey: 'lag1DiffMu', getValue: () => stats.lag1_diff_mean, format: 'ms' },
        { labelKey: 'lag1DiffSigma', getValue: () => stats.lag1_diff_std, format: 'ms' },
        { labelKey: 'runsTestZ', getValue: () => stats.runs_test_z, format: 'num', decimals: 2, helpKey: 'runsTest' },
        { labelKey: 'turnPoints', getValue: () => stats.turning_points, format: 'raw', helpKey: 'turningPoints' },
        { labelKey: 'turnRate', getValue: () => stats.turning_point_rate * 100, format: 'pct' },
        { labelKey: 'trendStr', getValue: () => stats.trend_strength, format: 'num', helpKey: 'trendStrength' },
      ]
    },
    {
      sectionKey: 'information',
      icon: Brain,
      cards: [
        { labelKey: 'shannon', getValue: () => stats.shannon_entropy, format: 'num', helpKey: 'shannonEnt' },
        { labelKey: 'renyi', getValue: () => stats.renyi_entropy, format: 'num', helpKey: 'renyiEnt' },
        { labelKey: 'tsallis', getValue: () => stats.tsallis_entropy, format: 'num' },
        { labelKey: 'spectralFlat', getValue: () => stats.spectral_flatness, format: 'num', helpKey: 'spectralFlat' },
        { labelKey: 'spectralEnt', getValue: () => stats.spectral_entropy, format: 'num' },
      ]
    },
    {
      sectionKey: 'distribution',
      icon: Target,
      cards: [
        { labelKey: 'jarqueBera', getValue: () => stats.jarque_bera, format: 'num', decimals: 2, helpKey: 'jarqueBera' },
        { labelKey: 'dagostinoK2', getValue: () => stats.dagostino_k2, format: 'num', decimals: 2 },
        { labelKey: 'cohensD', getValue: () => stats.cohens_d, format: 'num', helpKey: 'cohensD' },
        { labelKey: 'hedgesG', getValue: () => stats.hedges_g, format: 'num' },
      ]
    },
    {
      sectionKey: 'rolling',
      icon: TrendingUp,
      cards: [
        { labelKey: 'rollMean5', getValue: () => stats.rolling_mean_5, format: 'ms' },
        { labelKey: 'rollStd5', getValue: () => stats.rolling_std_5, format: 'ms' },
        { labelKey: 'rollMean10', getValue: () => stats.rolling_mean_10, format: 'ms' },
        { labelKey: 'rollStd10', getValue: () => stats.rolling_std_10, format: 'ms' },
        { labelKey: 'ewma', getValue: () => stats.ewma, format: 'ms', helpKey: 'ewma' },
      ]
    },
    {
      sectionKey: 'extreme',
      icon: Zap,
      cards: [
        { labelKey: 'peakToPeak', getValue: () => stats.peak_to_peak, format: 'ms' },
        { labelKey: 'crestFactor', getValue: () => stats.crest_factor, format: 'num', helpKey: 'crestFactor' },
        { labelKey: 'impulse', getValue: () => stats.impulse_factor, format: 'num', helpKey: 'impulseFactor' },
        { labelKey: 'shape', getValue: () => stats.shape_factor, format: 'num' },
        { labelKey: 'clearance', getValue: () => stats.clearance_factor, format: 'num' },
      ]
    },
    {
      sectionKey: 'partialAutocorr',
      icon: Radio,
      cards: [
        { labelKey: 'pacf1', getValue: () => stats.partial_autocorr1, format: 'num', helpKey: 'pacf' },
        { labelKey: 'pacf2', getValue: () => stats.partial_autocorr2, format: 'num' },
        { labelKey: 'ljungBoxQ', getValue: () => stats.ljung_box_q, format: 'num', decimals: 2, helpKey: 'ljungBox' },
        { labelKey: 'boxPierceQ', getValue: () => stats.box_pierce_q, format: 'num', decimals: 2 },
      ]
    },
    {
      sectionKey: 'chaos',
      icon: Shuffle,
      cards: [
        { labelKey: 'lyapunov', getValue: () => stats.lyapunov_approx, format: 'num', helpKey: 'lyapunov' },
        { labelKey: 'corrDim', getValue: () => stats.correlation_dim, format: 'num' },
        { labelKey: 'recurrence', getValue: () => stats.recurrence_rate * 100, format: 'pct', helpKey: 'recurrence' },
        { labelKey: 'determinism', getValue: () => stats.determinism, format: 'num', helpKey: 'determinism' },
        { labelKey: 'kaplanYorke', getValue: () => stats.kaplan_yorke_dim, format: 'num' },
      ]
    },
    {
      sectionKey: 'forecast',
      icon: Compass,
      cards: [
        { labelKey: 'next1', getValue: () => stats.forecast_1, format: 'ms', helpKey: 'forecast' },
        { labelKey: 'next5', getValue: () => stats.forecast_5, format: 'ms' },
        { labelKey: 'next10', getValue: () => stats.forecast_10, format: 'ms' },
        { labelKey: 'piLower', getValue: () => stats.prediction_interval_lower, format: 'ms', getSub: () => t('history.subs.percent95') },
        { labelKey: 'piUpper', getValue: () => stats.prediction_interval_upper, format: 'ms', getSub: () => t('history.subs.percent95') },
        { labelKey: 'steadyMu', getValue: () => stats.steady_state_mean, format: 'ms' },
      ]
    },
  ] : [], [stats, t])

  const chartOptions = useMemo(() => ({
    responsive: true,
    maintainAspectRatio: false,
    animation: { duration: 0 },
    plugins: {
      legend: { display: false },
      tooltip: {
        backgroundColor: isDark ? '#27272a' : '#ffffff',
        titleColor: isDark ? '#fafafa' : '#09090b',
        bodyColor: isDark ? '#a1a1aa' : '#52525b',
        borderColor: isDark ? '#3f3f46' : '#e4e4e7',
        borderWidth: 1,
        padding: 6,
        displayColors: false,
        callbacks: {
          label: (ctx: { parsed: { y: number | null } }) => `${(ctx.parsed.y ?? 0).toFixed(3)} ms`
        }
      }
    },
    scales: {
      x: {
        grid: { color: isDark ? '#27272a' : '#e4e4e7' },
        ticks: { color: isDark ? '#71717a' : '#a1a1aa', font: { size: 8 }, maxRotation: 0, maxTicksLimit: 8 }
      },
      y: {
        grid: { color: isDark ? '#27272a' : '#e4e4e7' },
        ticks: { color: isDark ? '#71717a' : '#a1a1aa', font: { size: 8 }, callback: (v: number | string) => `${v}ms` }
      }
    },
    interaction: { intersect: false, mode: 'index' as const },
  }), [isDark])

  const offsetChartData = useMemo(() => ({
    labels: history.map(h => fmtTime(h.time)),
    datasets: [{
      label: 'Offset',
      data: history.map(h => h.offset),
      borderColor: '#3b82f6',
      backgroundColor: 'rgba(59, 130, 246, 0.1)',
      borderWidth: 1,
      pointRadius: history.length > 100 ? 0 : 1,
      fill: true,
      tension: 0.3,
    }],
  }), [history])

  const delayChartData = useMemo(() => ({
    labels: history.map(h => fmtTime(h.time)),
    datasets: [{
      label: 'Delay',
      data: history.map(h => h.delay),
      borderColor: '#22c55e',
      backgroundColor: 'rgba(34, 197, 94, 0.1)',
      borderWidth: 1,
      pointRadius: history.length > 100 ? 0 : 1,
      fill: true,
      tension: 0.3,
    }],
  }), [history])

  const histogramData = useMemo(() => {
    if (history.length < 2) return null
    const offsets = history.map(h => Math.abs(h.offset))
    const min = Math.min(...offsets)
    const max = Math.max(...offsets)
    const binCount = Math.min(30, Math.ceil(Math.sqrt(history.length)))
    const binSize = (max - min) / binCount || 1
    const bins = Array(binCount).fill(0)
    offsets.forEach(v => { bins[Math.min(Math.floor((v - min) / binSize), binCount - 1)]++ })
    return {
      labels: bins.map((_, i) => `${(min + i * binSize).toFixed(0)}`),
      datasets: [{ data: bins, backgroundColor: isDark ? 'rgba(59, 130, 246, 0.5)' : 'rgba(59, 130, 246, 0.7)', borderColor: '#3b82f6', borderWidth: 1, borderRadius: 1 }]
    }
  }, [history, isDark])

  const delayHistogramData = useMemo(() => {
    if (history.length < 2) return null
    const delays = history.map(h => h.delay)
    const min = Math.min(...delays)
    const max = Math.max(...delays)
    const binCount = Math.min(30, Math.ceil(Math.sqrt(history.length)))
    const binSize = (max - min) / binCount || 1
    const bins = Array(binCount).fill(0)
    delays.forEach(v => { bins[Math.min(Math.floor((v - min) / binSize), binCount - 1)]++ })
    return {
      labels: bins.map((_, i) => `${(min + i * binSize).toFixed(0)}`),
      datasets: [{ data: bins, backgroundColor: isDark ? 'rgba(34, 197, 94, 0.5)' : 'rgba(34, 197, 94, 0.7)', borderColor: '#22c55e', borderWidth: 1, borderRadius: 1 }]
    }
  }, [history, isDark])

  const autocorrChartData = useMemo(() => {
    if (autocorrData.length === 0) return null
    return {
      labels: autocorrData.map((_, i) => `${i + 1}`),
      datasets: [{ data: autocorrData, backgroundColor: isDark ? 'rgba(168, 85, 247, 0.5)' : 'rgba(168, 85, 247, 0.7)', borderColor: '#a855f7', borderWidth: 1, borderRadius: 1 }]
    }
  }, [autocorrData, isDark])

  const getStabilityColor = (s: string) => s === 'excellent' ? 'text-emerald-500' : s === 'good' ? 'text-green-500' : s === 'normal' ? 'text-yellow-500' : 'text-red-500'

  return (
    <div className={`min-h-screen select-none overflow-x-hidden custom-scrollbar ${isDark ? 'bg-zinc-950 text-white dark' : 'bg-zinc-100 text-zinc-900'}`}>
      <div className={`sticky top-0 z-10 px-2 py-1.5 border-b ${isDark ? 'bg-zinc-950/95 border-zinc-800' : 'bg-zinc-100/95 border-zinc-300'} backdrop-blur-sm`}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <button onClick={() => router.back()} className={`p-1 rounded ${isDark ? 'hover:bg-zinc-800' : 'hover:bg-zinc-200'}`}>
              <ArrowLeft className="w-4 h-4" />
            </button>
            <div>
              <h1 className="text-xs font-medium flex items-center gap-1">
                {t('history.title')}
                {isLoading && <RefreshCw className="w-3 h-3 animate-spin text-blue-400" />}
              </h1>
              <p className={`text-[9px] ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`}>
                {dbStats ? (
                  <>
                    <Database className="w-2.5 h-2.5 inline mr-0.5" />
                    {history.length} / {dbStats.total_records} {t('history.records')}
                  </>
                ) : (
                  t('history.subtitle', { count: history.length, max: 3600 })
                )}
              </p>
            </div>
          </div>
          <div className="flex items-center gap-1">
            {/* 時間範圍選擇器 */}
            <select
              value={timeRange}
              onChange={(e) => setTimeRange(e.target.value as TimeRange)}
              className={`text-[9px] px-1.5 py-0.5 rounded border ${
                isDark
                  ? 'bg-zinc-800 border-zinc-700 text-zinc-300'
                  : 'bg-white border-zinc-300 text-zinc-700'
              } focus:outline-none focus:ring-1 focus:ring-blue-500`}
            >
              <option value="all">{t('history.timeRange.all')}</option>
              <option value="1h">{t('history.timeRange.1h')}</option>
              <option value="6h">{t('history.timeRange.6h')}</option>
              <option value="24h">{t('history.timeRange.24h')}</option>
              <option value="7d">{t('history.timeRange.7d')}</option>
              <option value="30d">{t('history.timeRange.30d')}</option>
            </select>
            <button
              onClick={() => loadData(timeRange)}
              disabled={isLoading}
              className={`p-1 rounded ${isLoading ? 'opacity-50' : isDark ? 'hover:bg-zinc-800' : 'hover:bg-zinc-200'}`}
              title={t('history.reload')}
            >
              <RefreshCw className={`w-3 h-3 ${isLoading ? 'animate-spin' : ''}`} />
            </button>
            <button onClick={clearHistory} disabled={history.length === 0} className={`p-1 rounded ${history.length === 0 ? 'opacity-50' : isDark ? 'hover:bg-zinc-800 text-red-400' : 'hover:bg-zinc-200 text-red-500'}`}>
              <Trash2 className="w-3 h-3" />
            </button>
            <button onClick={toggleTheme} className={`p-1 rounded ${isDark ? 'hover:bg-zinc-800' : 'hover:bg-zinc-200'}`}>
              {isDark ? <Sun className="w-3 h-3" /> : <Moon className="w-3 h-3" />}
            </button>
          </div>
        </div>
      </div>

      <div className="p-2 space-y-2 pb-4">
        {history.length < 2 || !stats ? (
          <div className={`flex flex-col items-center justify-center py-12 ${isDark ? 'text-zinc-600' : 'text-zinc-400'}`}>
            <BarChart3 className="w-8 h-8 mb-2 opacity-50" />
            <p className="text-[10px]">{t('history.noData')}</p>
          </div>
        ) : (
          <>
            {sectionsConfig.map((section) => (
              <StatSection
                key={section.sectionKey}
                config={section}
                stats={stats}
                t={t}
                isDark={isDark}
                getHelp={getHelp}
                fmtMs={fmtMs}
                fmtNum={fmtNum}
                fmtPct={fmtPct}
              />
            ))}

            <section>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.mean')} value={fmtMs(stats.mean)} isDark={isDark} help={getHelp('mean')} />
                <StatCard label={t('history.stats.median')} value={fmtMs(stats.median)} isDark={isDark} help={getHelp('median')} />
                <StatCard label={t('history.stats.mode')} value={fmtMs(stats.mode)} isDark={isDark} help={getHelp('mode')} />
                <StatCard label={t('history.stats.geoMean')} value={fmtMs(stats.geometric_mean)} isDark={isDark} help={getHelp('geometricMean')} />
                <StatCard label={t('history.stats.harmonic')} value={fmtMs(stats.harmonic_mean)} isDark={isDark} help={getHelp('harmonicMean')} />
                <StatCard label={t('history.stats.trimmed')} value={fmtMs(stats.trimmed_mean)} sub={t('history.subs.10pctTrim')} isDark={isDark} help={getHelp('trimmedMean')} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Sigma} isDark={isDark}>{t('history.sections.dispersion')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.variance')} value={`${fmtNum(stats.variance, 2)}ms²`} isDark={isDark} help={getHelp('variance')} />
                <StatCard label={t('history.stats.stdDev')} value={fmtMs(stats.std_dev)} isDark={isDark} help={getHelp('stdDev')} />
                <StatCard label={t('history.stats.cv')} value={fmtPct(stats.cv)} isDark={isDark} help={getHelp('cv')} />
                <StatCard label={t('history.stats.mad')} value={fmtMs(stats.mad)} isDark={isDark} help={getHelp('mad')} />
                <StatCard label={t('history.stats.aad')} value={fmtMs(stats.aad)} isDark={isDark} help={getHelp('aad')} />
                <StatCard label={t('history.stats.iqr')} value={fmtMs(stats.iqr)} isDark={isDark} help={getHelp('iqr')} />
                <StatCard label={t('history.stats.sem')} value={fmtMs(stats.sem)} isDark={isDark} help={getHelp('sem')} />
                <StatCard label={t('history.stats.mse')} value={`${fmtNum(stats.mse, 2)}ms²`} isDark={isDark} help={getHelp('mse')} />
                <StatCard label={t('history.stats.range')} value={fmtMs(stats.range)} isDark={isDark} help={getHelp('range')} />
                <StatCard label={t('history.stats.min')} value={fmtMs(stats.min)} isDark={isDark} />
                <StatCard label={t('history.stats.max')} value={fmtMs(stats.max)} isDark={isDark} />
                <StatCard label={t('history.stats.sumAbs')} value={fmtMs(stats.sum_abs)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Waves} isDark={isDark}>{t('history.sections.shape')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.skewness')} value={fmtNum(stats.skewness)} sub={stats.skewness_type} isDark={isDark} help={getHelp('skewness')} />
                <StatCard label={t('history.stats.kurtosis')} value={fmtNum(stats.kurtosis)} sub={stats.kurtosis_type} isDark={isDark} help={getHelp('kurtosis')} />
                <StatCard label={t('history.stats.skewDelay')} value={fmtNum(stats.skewness_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.kurtDelay')} value={fmtNum(stats.kurtosis_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.skewAbs')} value={fmtNum(stats.skewness_abs)} isDark={isDark} />
                <StatCard label={t('history.stats.kurtAbs')} value={fmtNum(stats.kurtosis_abs)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Target} isDark={isDark}>{t('history.sections.offsetPercentiles')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-9 gap-1">
                <StatCard label="P1" value={fmtMs(stats.p1)} isDark={isDark} />
                <StatCard label="P5" value={fmtMs(stats.p5)} isDark={isDark} />
                <StatCard label="P10" value={fmtMs(stats.p10)} isDark={isDark} />
                <StatCard label="P25" value={fmtMs(stats.p25)} sub={t('history.subs.q1')} isDark={isDark} />
                <StatCard label="P50" value={fmtMs(stats.p50)} sub={t('history.stats.median')} isDark={isDark} />
                <StatCard label="P75" value={fmtMs(stats.p75)} sub={t('history.subs.q3')} isDark={isDark} />
                <StatCard label="P90" value={fmtMs(stats.p90)} isDark={isDark} />
                <StatCard label="P95" value={fmtMs(stats.p95)} isDark={isDark} />
                <StatCard label="P99" value={fmtMs(stats.p99)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Timer} isDark={isDark}>{t('history.sections.delayPercentiles')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-9 gap-1">
                <StatCard label="P1" value={fmtMs(stats.p1_delay)} isDark={isDark} />
                <StatCard label="P5" value={fmtMs(stats.p5_delay)} isDark={isDark} />
                <StatCard label="P10" value={fmtMs(stats.p10_delay)} isDark={isDark} />
                <StatCard label="P25" value={fmtMs(stats.p25_delay)} sub={t('history.subs.q1')} isDark={isDark} />
                <StatCard label="P50" value={fmtMs(stats.p50_delay)} sub={t('history.stats.median')} isDark={isDark} />
                <StatCard label="P75" value={fmtMs(stats.p75_delay)} sub={t('history.subs.q3')} isDark={isDark} />
                <StatCard label="P90" value={fmtMs(stats.p90_delay)} isDark={isDark} />
                <StatCard label="P95" value={fmtMs(stats.p95_delay)} isDark={isDark} />
                <StatCard label="P99" value={fmtMs(stats.p99_delay)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={TrendingUp} isDark={isDark}>{t('history.sections.trendAnalysis')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.slope')} value={`${stats.slope >= 0 ? '+' : ''}${fmtNum(stats.slope, 4)}`} sub={t('history.subs.msPerSample')} trend={stats.trend_direction as 'up' | 'down' | 'stable'} isDark={isDark} help={getHelp('slope')} />
                <StatCard label={t('history.stats.r2')} value={fmtNum(stats.r2, 4)} isDark={isDark} help={getHelp('r2')} />
                <StatCard label={t('history.stats.intercept')} value={fmtMs(stats.intercept)} isDark={isDark} />
                <StatCard label={t('history.stats.driftRate')} value={`${fmtNum(stats.drift_rate, 4)}`} sub={t('history.subs.msPerSample')} isDark={isDark} help={getHelp('drift')} />
                <StatCard label={t('history.stats.totalDrift')} value={fmtMs(stats.drift_total)} isDark={isDark} />
                <StatCard label={t('history.stats.recentAvg')} value={fmtMs(stats.recent_avg)} sub={t('history.subs.last10')} isDark={isDark} />
                <StatCard label={t('history.stats.slopeDelay')} value={`${stats.slope_delay >= 0 ? '+' : ''}${fmtNum(stats.slope_delay, 4)}`} trend={stats.delay_trend_direction as 'up' | 'down' | 'stable'} isDark={isDark} />
                <StatCard label={t('history.stats.r2Delay')} value={fmtNum(stats.r2_delay, 4)} isDark={isDark} />
                <StatCard label={t('history.stats.olderAvg')} value={fmtMs(stats.older_avg)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Radio} isDark={isDark}>{t('history.sections.autocorrelation')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.acf1')} value={fmtNum(stats.autocorr1)} isDark={isDark} help={getHelp('autocorr')} />
                <StatCard label={t('history.stats.acf2')} value={fmtNum(stats.autocorr2)} isDark={isDark} />
                <StatCard label={t('history.stats.acf3')} value={fmtNum(stats.autocorr3)} isDark={isDark} />
                <StatCard label={t('history.stats.acf5')} value={fmtNum(stats.autocorr5)} isDark={isDark} />
                <StatCard label={t('history.stats.acf10')} value={fmtNum(stats.autocorr10)} isDark={isDark} />
                <StatCard label={t('history.stats.durbinWatson')} value={fmtNum(stats.dw, 3)} sub={stats.dw_status} isDark={isDark} help={getHelp('dw')} />
              </div>
            </section>

            <section>
              <SectionTitle icon={AlertTriangle} isDark={isDark}>{t('history.sections.outliers')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.iqrOutliers')} value={stats.outliers} sub={fmtPct(stats.outlier_pct)} color={stats.outliers > 0 ? 'text-yellow-500' : 'text-emerald-500'} isDark={isDark} help={getHelp('outliers')} />
                <StatCard label={t('history.stats.zOutliers')} value={stats.z_outliers} sub={fmtPct(stats.z_outlier_pct)} color={stats.z_outliers > 0 ? 'text-yellow-500' : 'text-emerald-500'} isDark={isDark} help={getHelp('zOutliers')} />
                <StatCard label={t('history.stats.iqrDelay')} value={stats.outliers_delay} sub={fmtPct(stats.outlier_pct_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.zDelay')} value={stats.z_outliers_delay} sub={fmtPct(stats.z_outlier_pct_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.iqrWidth')} value={fmtMs(stats.iqr)} isDark={isDark} />
                <StatCard label={t('history.stats.fence')} value={`${fmtNum(stats.p25 - 1.5 * stats.iqr, 1)}/${fmtNum(stats.p75 + 1.5 * stats.iqr, 1)}`} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Gauge} isDark={isDark}>{t('history.sections.stability')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.allanDev')} value={fmtMs(stats.allan)} isDark={isDark} help={getHelp('allan')} />
                <StatCard label={t('history.stats.mtie')} value={fmtMs(stats.mtie)} isDark={isDark} help={getHelp('mtie')} />
                <StatCard label={t('history.stats.tdev')} value={fmtMs(stats.tdev)} isDark={isDark} help={getHelp('tdev')} />
                <StatCard label={t('history.stats.jitter')} value={fmtMs(stats.jitter)} isDark={isDark} help={getHelp('jitter')} />
                <StatCard label={t('history.stats.allanDelay')} value={fmtMs(stats.allan_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.jitterDelay')} value={fmtMs(stats.jitter_delay)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Brain} isDark={isDark}>{t('history.sections.complexity')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.approxEnt')} value={fmtNum(stats.approx_ent)} isDark={isDark} help={getHelp('approxEnt')} />
                <StatCard label={t('history.stats.sampleEnt')} value={fmtNum(stats.sample_ent)} isDark={isDark} help={getHelp('sampleEnt')} />
                <StatCard label={t('history.stats.permEnt')} value={fmtNum(stats.perm_ent)} isDark={isDark} help={getHelp('permEnt')} />
                <StatCard label={t('history.stats.hurst')} value={fmtNum(stats.hurst)} sub={stats.hurst > 0.6 ? t('history.labels.persistent') : stats.hurst < 0.4 ? t('history.labels.antiPersist') : t('history.labels.random')} isDark={isDark} help={getHelp('hurst')} />
                <StatCard label={t('history.stats.apEnDelay')} value={fmtNum(stats.approx_ent_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.hurstDelay')} value={fmtNum(stats.hurst_delay)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={GitCompare} isDark={isDark}>{t('history.sections.consecutive')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.maxPosStreak')} value={stats.max_pos} isDark={isDark} />
                <StatCard label={t('history.stats.maxNegStreak')} value={stats.max_neg} isDark={isDark} />
                <StatCard label={t('history.stats.longest')} value={stats.longest_streak} isDark={isDark} />
                <StatCard label={t('history.stats.posCount')} value={stats.pos_count} sub={fmtPct(stats.balance * 100)} isDark={isDark} />
                <StatCard label={t('history.stats.negCount')} value={stats.neg_count} sub={fmtPct((1 - stats.balance) * 100)} isDark={isDark} />
                <StatCard label={t('history.stats.crossings')} value={stats.crossings} sub={`${t('history.subs.rate')}: ${fmtNum(stats.crossing_rate, 3)}`} isDark={isDark} help={getHelp('crossings')} />
                <StatCard label={t('history.stats.bias')} value={stats.bias} color={stats.bias === 'Balanced' ? 'text-emerald-500' : 'text-yellow-500'} isDark={isDark} />
                <StatCard label={t('history.stats.balance')} value={`${fmtPct(stats.balance * 100)} / ${fmtPct((1 - stats.balance) * 100)}`} sub={t('history.subs.plusMinus')} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Scale} isDark={isDark}>{t('history.sections.confidence')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.ci95Lower')} value={fmtMs(stats.ci95_lower)} isDark={isDark} help={getHelp('ci')} />
                <StatCard label={t('history.stats.ci95Upper')} value={fmtMs(stats.ci95_upper)} isDark={isDark} />
                <StatCard label={t('history.stats.ci95Margin')} value={`±${fmtMs(stats.ci95_margin)}`} isDark={isDark} />
                <StatCard label={t('history.stats.ci99Lower')} value={fmtMs(stats.ci99_lower)} isDark={isDark} />
                <StatCard label={t('history.stats.ci99Upper')} value={fmtMs(stats.ci99_upper)} isDark={isDark} />
                <StatCard label={t('history.stats.ci99Margin')} value={`±${fmtMs(stats.ci99_margin)}`} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={CheckCircle} isDark={isDark}>{t('history.sections.accuracy')}</SectionTitle>
              <div className="grid grid-cols-4 sm:grid-cols-8 gap-1">
                <StatCard label={t('history.stats.within1ms')} value={fmtPct(stats.within_1ms)} color={stats.within_1ms > 50 ? 'text-emerald-500' : undefined} isDark={isDark} />
                <StatCard label={t('history.stats.within5ms')} value={fmtPct(stats.within_5ms)} color={stats.within_5ms > 80 ? 'text-emerald-500' : undefined} isDark={isDark} />
                <StatCard label={t('history.stats.within10ms')} value={fmtPct(stats.within_10ms)} color={stats.within_10ms > 90 ? 'text-emerald-500' : undefined} isDark={isDark} />
                <StatCard label={t('history.stats.within25ms')} value={fmtPct(stats.within_25ms)} isDark={isDark} />
                <StatCard label={t('history.stats.within50ms')} value={fmtPct(stats.within_50ms)} isDark={isDark} />
                <StatCard label={t('history.stats.within100ms')} value={fmtPct(stats.within_100ms)} isDark={isDark} />
                <StatCard label={t('history.stats.within500ms')} value={fmtPct(stats.within_500ms)} isDark={isDark} />
                <StatCard label={t('history.stats.within1s')} value={fmtPct(stats.within_1s)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Wind} isDark={isDark}>{t('history.sections.delayStats')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.delayMean')} value={fmtMs(stats.mean_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayMedian')} value={fmtMs(stats.median_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayMode')} value={fmtMs(stats.mode_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayStdDev')} value={fmtMs(stats.std_dev_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayCv')} value={fmtPct(stats.cv_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayMad')} value={fmtMs(stats.mad_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayIqr')} value={fmtMs(stats.iqr_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayRms')} value={fmtMs(stats.rms_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayRange')} value={fmtMs(stats.range_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayMin')} value={fmtMs(stats.min_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayMax')} value={fmtMs(stats.max_delay)} isDark={isDark} />
                <StatCard label={t('history.stats.delayVariance')} value={`${fmtNum(stats.variance_delay, 2)}ms²`} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Layers} isDark={isDark}>{t('history.sections.robust')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.winsorized')} value={fmtMs(stats.winsorized_mean)} isDark={isDark} help={getHelp('winsorized')} />
                <StatCard label={t('history.stats.biweight')} value={fmtMs(stats.biweight_mean)} isDark={isDark} help={getHelp('biweight')} />
                <StatCard label={t('history.stats.midrange')} value={fmtMs(stats.midrange)} isDark={isDark} />
                <StatCard label={t('history.stats.midhinge')} value={fmtMs(stats.midhinge)} isDark={isDark} />
                <StatCard label={t('history.stats.trimean')} value={fmtMs(stats.trimean)} isDark={isDark} help={getHelp('trimean')} />
                <StatCard label={t('history.stats.giniDiff')} value={fmtMs(stats.gini_mean_diff)} isDark={isDark} help={getHelp('gini')} />
                <StatCard label={t('history.stats.quartileDev')} value={fmtMs(stats.quartile_dev)} isDark={isDark} />
                <StatCard label={t('history.stats.decileRange')} value={fmtMs(stats.decile_range)} isDark={isDark} />
                <StatCard label={t('history.stats.interdecile')} value={fmtMs(stats.interdecile_range)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Sigma} isDark={isDark}>{t('history.sections.higherOrder')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.moment3')} value={fmtNum(stats.moment3, 2)} isDark={isDark} />
                <StatCard label={t('history.stats.moment4')} value={fmtNum(stats.moment4, 2)} isDark={isDark} />
                <StatCard label={t('history.stats.moment5')} value={fmtNum(stats.moment5, 2)} isDark={isDark} />
                <StatCard label={t('history.stats.moment6')} value={fmtNum(stats.moment6, 2)} isDark={isDark} />
                <StatCard label={t('history.stats.bowleySkew')} value={fmtNum(stats.bowley_skewness)} isDark={isDark} help={getHelp('bowley')} />
                <StatCard label={t('history.stats.pearsonSkew')} value={fmtNum(stats.pearson_skewness)} isDark={isDark} help={getHelp('pearsonSkew')} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Gauge} isDark={isDark}>{t('history.sections.advancedStability')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.hadamardVar')} value={fmtNum(stats.hadamard_variance, 4)} isDark={isDark} help={getHelp('hadamard')} />
                <StatCard label={t('history.stats.modifiedAllan')} value={fmtMs(stats.modified_allan)} isDark={isDark} help={getHelp('modifiedAllan')} />
                <StatCard label={t('history.stats.theo1')} value={fmtMs(stats.theo1)} isDark={isDark} />
                <StatCard label={t('history.stats.totalVar')} value={fmtNum(stats.total_variance, 2)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Activity} isDark={isDark}>{t('history.sections.timeSeries')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.lag1DiffMu')} value={fmtMs(stats.lag1_diff_mean)} isDark={isDark} />
                <StatCard label={t('history.stats.lag1DiffSigma')} value={fmtMs(stats.lag1_diff_std)} isDark={isDark} />
                <StatCard label={t('history.stats.runsTestZ')} value={fmtNum(stats.runs_test_z, 2)} isDark={isDark} help={getHelp('runsTest')} />
                <StatCard label={t('history.stats.turnPoints')} value={stats.turning_points} isDark={isDark} help={getHelp('turningPoints')} />
                <StatCard label={t('history.stats.turnRate')} value={fmtPct(stats.turning_point_rate * 100)} isDark={isDark} />
                <StatCard label={t('history.stats.trendStr')} value={fmtNum(stats.trend_strength)} isDark={isDark} help={getHelp('trendStrength')} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Brain} isDark={isDark}>{t('history.sections.information')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.shannon')} value={fmtNum(stats.shannon_entropy)} isDark={isDark} help={getHelp('shannonEnt')} />
                <StatCard label={t('history.stats.renyi')} value={fmtNum(stats.renyi_entropy)} isDark={isDark} help={getHelp('renyiEnt')} />
                <StatCard label={t('history.stats.tsallis')} value={fmtNum(stats.tsallis_entropy)} isDark={isDark} />
                <StatCard label={t('history.stats.spectralFlat')} value={fmtNum(stats.spectral_flatness)} isDark={isDark} help={getHelp('spectralFlat')} />
                <StatCard label={t('history.stats.spectralEnt')} value={fmtNum(stats.spectral_entropy)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Target} isDark={isDark}>{t('history.sections.distribution')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.jarqueBera')} value={fmtNum(stats.jarque_bera, 2)} isDark={isDark} help={getHelp('jarqueBera')} />
                <StatCard label={t('history.stats.dagostinoK2')} value={fmtNum(stats.dagostino_k2, 2)} isDark={isDark} />
                <StatCard label={t('history.stats.cohensD')} value={fmtNum(stats.cohens_d)} isDark={isDark} help={getHelp('cohensD')} />
                <StatCard label={t('history.stats.hedgesG')} value={fmtNum(stats.hedges_g)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={TrendingUp} isDark={isDark}>{t('history.sections.rolling')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.rollMean5')} value={fmtMs(stats.rolling_mean_5)} isDark={isDark} />
                <StatCard label={t('history.stats.rollStd5')} value={fmtMs(stats.rolling_std_5)} isDark={isDark} />
                <StatCard label={t('history.stats.rollMean10')} value={fmtMs(stats.rolling_mean_10)} isDark={isDark} />
                <StatCard label={t('history.stats.rollStd10')} value={fmtMs(stats.rolling_std_10)} isDark={isDark} />
                <StatCard label={t('history.stats.ewma')} value={fmtMs(stats.ewma)} isDark={isDark} help={getHelp('ewma')} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Zap} isDark={isDark}>{t('history.sections.extreme')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.peakToPeak')} value={fmtMs(stats.peak_to_peak)} isDark={isDark} />
                <StatCard label={t('history.stats.crestFactor')} value={fmtNum(stats.crest_factor)} isDark={isDark} help={getHelp('crestFactor')} />
                <StatCard label={t('history.stats.impulse')} value={fmtNum(stats.impulse_factor)} isDark={isDark} help={getHelp('impulseFactor')} />
                <StatCard label={t('history.stats.shape')} value={fmtNum(stats.shape_factor)} isDark={isDark} />
                <StatCard label={t('history.stats.clearance')} value={fmtNum(stats.clearance_factor)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Radio} isDark={isDark}>{t('history.sections.partialAutocorr')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.pacf1')} value={fmtNum(stats.partial_autocorr1)} isDark={isDark} help={getHelp('pacf')} />
                <StatCard label={t('history.stats.pacf2')} value={fmtNum(stats.partial_autocorr2)} isDark={isDark} />
                <StatCard label={t('history.stats.ljungBoxQ')} value={fmtNum(stats.ljung_box_q, 2)} isDark={isDark} help={getHelp('ljungBox')} />
                <StatCard label={t('history.stats.boxPierceQ')} value={fmtNum(stats.box_pierce_q, 2)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Shuffle} isDark={isDark}>{t('history.sections.chaos')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.lyapunov')} value={fmtNum(stats.lyapunov_approx)} isDark={isDark} help={getHelp('lyapunov')} />
                <StatCard label={t('history.stats.corrDim')} value={fmtNum(stats.correlation_dim)} isDark={isDark} />
                <StatCard label={t('history.stats.recurrence')} value={fmtPct(stats.recurrence_rate * 100)} isDark={isDark} help={getHelp('recurrence')} />
                <StatCard label={t('history.stats.determinism')} value={fmtNum(stats.determinism)} isDark={isDark} help={getHelp('determinism')} />
                <StatCard label={t('history.stats.kaplanYorke')} value={fmtNum(stats.kaplan_yorke_dim)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={Compass} isDark={isDark}>{t('history.sections.forecast')}</SectionTitle>
              <div className="grid grid-cols-3 sm:grid-cols-6 gap-1">
                <StatCard label={t('history.stats.next1')} value={fmtMs(stats.forecast_1)} isDark={isDark} help={getHelp('forecast')} />
                <StatCard label={t('history.stats.next5')} value={fmtMs(stats.forecast_5)} isDark={isDark} />
                <StatCard label={t('history.stats.next10')} value={fmtMs(stats.forecast_10)} isDark={isDark} />
                <StatCard label={t('history.stats.piLower')} value={fmtMs(stats.prediction_interval_lower)} sub={t('history.subs.percent95')} isDark={isDark} />
                <StatCard label={t('history.stats.piUpper')} value={fmtMs(stats.prediction_interval_upper)} sub={t('history.subs.percent95')} isDark={isDark} />
                <StatCard label={t('history.stats.steadyMu')} value={fmtMs(stats.steady_state_mean)} isDark={isDark} />
              </div>
            </section>

            <section>
              <SectionTitle icon={LineChart} isDark={isDark}>{t('history.charts.offsetTrend')}</SectionTitle>
              <div className={`rounded p-2 ${isDark ? 'bg-zinc-800/50' : 'bg-white border border-zinc-200'}`}>
                <div className="h-[140px]"><Line data={offsetChartData} options={chartOptions} /></div>
              </div>
            </section>

            <section>
              <SectionTitle icon={LineChart} isDark={isDark}>{t('history.charts.delayTrend')}</SectionTitle>
              <div className={`rounded p-2 ${isDark ? 'bg-zinc-800/50' : 'bg-white border border-zinc-200'}`}>
                <div className="h-[140px]"><Line data={delayChartData} options={chartOptions} /></div>
              </div>
            </section>

            {histogramData && (
              <section>
                <SectionTitle icon={BarChart3} isDark={isDark}>{t('history.charts.distribution')}</SectionTitle>
                <div className={`rounded p-2 ${isDark ? 'bg-zinc-800/50' : 'bg-white border border-zinc-200'}`}>
                  <div className="h-[140px]"><Bar data={histogramData} options={{ ...chartOptions, scales: { ...chartOptions.scales, y: { ...chartOptions.scales.y, ticks: { ...chartOptions.scales.y.ticks, callback: (v: number | string) => `${v}` } } } }} /></div>
                </div>
              </section>
            )}

            {delayHistogramData && (
              <section>
                <SectionTitle icon={BarChart3} isDark={isDark}>{t('history.charts.delayDistribution')}</SectionTitle>
                <div className={`rounded p-2 ${isDark ? 'bg-zinc-800/50' : 'bg-white border border-zinc-200'}`}>
                  <div className="h-[140px]"><Bar data={delayHistogramData} options={{ ...chartOptions, scales: { ...chartOptions.scales, y: { ...chartOptions.scales.y, ticks: { ...chartOptions.scales.y.ticks, callback: (v: number | string) => `${v}` } } } }} /></div>
                </div>
              </section>
            )}

            {autocorrChartData && (
              <section>
                <SectionTitle icon={Sparkles} isDark={isDark}>{t('history.charts.acf')}</SectionTitle>
                <div className={`rounded p-2 ${isDark ? 'bg-zinc-800/50' : 'bg-white border border-zinc-200'}`}>
                  <div className="h-[140px]"><Bar data={autocorrChartData} options={{ ...chartOptions, scales: { ...chartOptions.scales, x: { ...chartOptions.scales.x, title: { display: true, text: 'Lag', color: isDark ? '#71717a' : '#a1a1aa', font: { size: 9 } } }, y: { ...chartOptions.scales.y, min: -1, max: 1, ticks: { ...chartOptions.scales.y.ticks, callback: (v: number | string) => `${v}` } } } }} /></div>
                </div>
              </section>
            )}

            <section>
              <SectionTitle icon={Clock} isDark={isDark}>{t('history.rawData')} ({Math.min(100, history.length)} {t('history.recent')})</SectionTitle>
              <div className={`rounded overflow-hidden ${isDark ? 'bg-zinc-800/50' : 'bg-white border border-zinc-200'}`}>
                <div className="max-h-[200px] overflow-y-auto custom-scrollbar">
                  <table className="w-full text-[9px]">
                    <thead className={`sticky top-0 ${isDark ? 'bg-zinc-800' : 'bg-zinc-100'}`}>
                      <tr>
                        <th className={`text-left px-2 py-1 font-medium ${isDark ? 'text-zinc-400' : 'text-zinc-600'}`}>{t('history.table.time')}</th>
                        <th className={`text-right px-2 py-1 font-medium ${isDark ? 'text-zinc-400' : 'text-zinc-600'}`}>{t('history.table.offset')}</th>
                        <th className={`text-right px-2 py-1 font-medium ${isDark ? 'text-zinc-400' : 'text-zinc-600'}`}>{t('history.table.delay')}</th>
                        <th className={`text-left px-2 py-1 font-medium ${isDark ? 'text-zinc-400' : 'text-zinc-600'}`}>{t('history.table.server')}</th>
                      </tr>
                    </thead>
                    <tbody className={`divide-y ${isDark ? 'divide-zinc-700/30' : 'divide-zinc-200'}`}>
                      {[...history].reverse().slice(0, 100).map((entry, idx) => (
                        <tr key={idx} className={isDark ? 'hover:bg-zinc-700/20' : 'hover:bg-zinc-50'}>
                          <td className={`px-2 py-0.5 font-mono ${isDark ? 'text-zinc-300' : 'text-zinc-700'}`}>{fmtTime(entry.time)}</td>
                          <td className={`px-2 py-0.5 text-right font-mono ${Math.abs(entry.offset) < 10 ? 'text-emerald-500' : Math.abs(entry.offset) < 50 ? 'text-green-500' : Math.abs(entry.offset) < 100 ? 'text-yellow-500' : 'text-red-500'}`}>{entry.offset >= 0 ? '+' : ''}{entry.offset.toFixed(3)}ms</td>
                          <td className={`px-2 py-0.5 text-right font-mono ${isDark ? 'text-zinc-400' : 'text-zinc-600'}`}>{entry.delay.toFixed(3)}ms</td>
                          <td className={`px-2 py-0.5 ${isDark ? 'text-zinc-500' : 'text-zinc-500'}`}>{entry.server.replace('time.', '').replace('.com', '').replace('.tw', '')}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
            </section>
          </>
        )}
      </div>
    </div>
  )
}
