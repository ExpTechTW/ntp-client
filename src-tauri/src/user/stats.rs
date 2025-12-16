use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub offset: f64,
    pub delay: f64,
    pub server: String,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResult {
    pub samples: usize,
    pub mean: f64,
    pub mean_delay: f64,
    pub median: f64,
    pub median_delay: f64,
    pub mode: f64,
    pub mode_delay: f64,
    pub geometric_mean: f64,
    pub harmonic_mean: f64,
    pub trimmed_mean: f64,
    pub trimmed_mean_delay: f64,
    pub sum: f64,
    pub sum_abs: f64,
    pub sum_delay: f64,
    pub min: f64,
    pub max: f64,
    pub min_delay: f64,
    pub max_delay: f64,

    pub variance: f64,
    pub variance_delay: f64,
    pub std_dev: f64,
    pub std_dev_delay: f64,
    pub cv: f64,
    pub cv_delay: f64,
    pub mad: f64,
    pub mad_delay: f64,
    pub aad: f64,
    pub aad_delay: f64,
    pub iqr: f64,
    pub iqr_delay: f64,
    pub sem: f64,
    pub sem_delay: f64,
    pub rms: f64,
    pub rms_delay: f64,
    pub mse: f64,
    pub mse_delay: f64,
    pub range: f64,
    pub range_delay: f64,

    pub skewness: f64,
    pub skewness_delay: f64,
    pub kurtosis: f64,
    pub kurtosis_delay: f64,
    pub skewness_abs: f64,
    pub kurtosis_abs: f64,
    pub skewness_type: String,
    pub kurtosis_type: String,

    pub p1: f64,
    pub p5: f64,
    pub p10: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,

    pub p1_delay: f64,
    pub p5_delay: f64,
    pub p10_delay: f64,
    pub p25_delay: f64,
    pub p50_delay: f64,
    pub p75_delay: f64,
    pub p90_delay: f64,
    pub p95_delay: f64,
    pub p99_delay: f64,

    pub slope: f64,
    pub slope_delay: f64,
    pub intercept: f64,
    pub intercept_delay: f64,
    pub r2: f64,
    pub r2_delay: f64,
    pub trend_direction: String,
    pub delay_trend_direction: String,
    pub drift_rate: f64,
    pub drift_total: f64,
    pub recent_avg: f64,
    pub older_avg: f64,

    pub autocorr1: f64,
    pub autocorr2: f64,
    pub autocorr3: f64,
    pub autocorr5: f64,
    pub autocorr10: f64,
    pub dw: f64,
    pub dw_delay: f64,
    pub dw_status: String,

    pub outliers: usize,
    pub outliers_delay: usize,
    pub outlier_pct: f64,
    pub outlier_pct_delay: f64,
    pub z_outliers: usize,
    pub z_outliers_delay: usize,
    pub z_outlier_pct: f64,
    pub z_outlier_pct_delay: f64,

    pub allan: f64,
    pub allan_delay: f64,
    pub mtie: f64,
    pub mtie_delay: f64,
    pub tdev: f64,
    pub tdev_delay: f64,
    pub jitter: f64,
    pub jitter_delay: f64,

    pub approx_ent: f64,
    pub approx_ent_delay: f64,
    pub sample_ent: f64,
    pub sample_ent_delay: f64,
    pub perm_ent: f64,
    pub perm_ent_delay: f64,
    pub hurst: f64,
    pub hurst_delay: f64,

    pub max_pos: usize,
    pub max_neg: usize,
    pub pos_count: usize,
    pub neg_count: usize,
    pub crossings: usize,
    pub crossing_rate: f64,
    pub balance: f64,
    pub bias: String,
    pub longest_streak: usize,

    pub ci95_lower: f64,
    pub ci95_upper: f64,
    pub ci95_margin: f64,
    pub ci99_lower: f64,
    pub ci99_upper: f64,
    pub ci99_margin: f64,

    pub within_1ms: f64,
    pub within_5ms: f64,
    pub within_10ms: f64,
    pub within_25ms: f64,
    pub within_50ms: f64,
    pub within_100ms: f64,
    pub within_500ms: f64,
    pub within_1s: f64,

    pub stability: String,

    pub moment3: f64,
    pub moment4: f64,
    pub moment5: f64,
    pub moment6: f64,

    pub winsorized_mean: f64,
    pub biweight_mean: f64,
    pub midrange: f64,
    pub midhinge: f64,
    pub trimean: f64,

    pub gini_mean_diff: f64,
    pub quartile_dev: f64,
    pub decile_range: f64,
    pub interdecile_range: f64,

    pub bowley_skewness: f64,
    pub pearson_skewness: f64,
    pub excess_kurtosis: f64,

    pub hadamard_variance: f64,
    pub modified_allan: f64,
    pub theo1: f64,
    pub total_variance: f64,

    pub lag1_diff_mean: f64,
    pub lag1_diff_std: f64,
    pub runs_test_z: f64,
    pub turning_points: usize,
    pub turning_point_rate: f64,

    pub spectral_flatness: f64,
    pub spectral_entropy: f64,
    pub dominant_freq: f64,

    pub shannon_entropy: f64,
    pub renyi_entropy: f64,
    pub tsallis_entropy: f64,

    pub jarque_bera: f64,
    pub dagostino_k2: f64,
    pub shapiro_wilk_approx: f64,

    pub cohens_d: f64,
    pub hedges_g: f64,
    pub glass_delta: f64,

    pub rolling_mean_5: f64,
    pub rolling_std_5: f64,
    pub rolling_mean_10: f64,
    pub rolling_std_10: f64,
    pub ewma: f64,

    pub peak_to_peak: f64,
    pub crest_factor: f64,
    pub impulse_factor: f64,
    pub shape_factor: f64,
    pub clearance_factor: f64,

    pub trend_strength: f64,
    pub seasonality_strength: f64,
    pub stl_trend: f64,
    pub stl_remainder_var: f64,

    pub partial_autocorr1: f64,
    pub partial_autocorr2: f64,
    pub ljung_box_q: f64,
    pub box_pierce_q: f64,

    pub lyapunov_approx: f64,
    pub correlation_dim: f64,
    pub recurrence_rate: f64,
    pub determinism: f64,

    pub embedding_dim: usize,
    pub false_nearest: f64,
    pub average_mutual_info: f64,

    pub largest_lyapunov: f64,
    pub kaplan_yorke_dim: f64,

    pub steady_state_mean: f64,
    pub steady_state_var: f64,
    pub transient_length: usize,

    pub forecast_1: f64,
    pub forecast_5: f64,
    pub forecast_10: f64,
    pub prediction_interval_lower: f64,
    pub prediction_interval_upper: f64,
}

fn mean(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    arr.iter().sum::<f64>() / arr.len() as f64
}

fn sum(arr: &[f64]) -> f64 {
    arr.iter().sum()
}

fn variance(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    let m = mean(arr);
    arr.iter().map(|v| (v - m).powi(2)).sum::<f64>() / arr.len() as f64
}

fn std_dev(arr: &[f64]) -> f64 {
    variance(arr).sqrt()
}

fn percentile(arr: &[f64], p: f64) -> f64 {
    if arr.is_empty() { return 0.0; }
    let mut sorted = arr.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = idx.floor() as usize;
    let upper = idx.ceil() as usize;
    if lower == upper { sorted[lower] }
    else { sorted[lower] * (upper as f64 - idx) + sorted[upper] * (idx - lower as f64) }
}

fn median(arr: &[f64]) -> f64 {
    percentile(arr, 50.0)
}

fn mode(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    use std::collections::HashMap;
    let mut freq: HashMap<i64, usize> = HashMap::new();
    for &v in arr {
        let rounded = (v * 10.0).round() as i64;
        *freq.entry(rounded).or_insert(0) += 1;
    }
    freq.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(k, _)| k as f64 / 10.0)
        .unwrap_or(0.0)
}

fn geometric_mean(arr: &[f64]) -> f64 {
    let positive: Vec<f64> = arr.iter().filter(|&&v| v > 0.0).copied().collect();
    if positive.is_empty() { return 0.0; }
    positive.iter().product::<f64>().powf(1.0 / positive.len() as f64)
}

fn harmonic_mean(arr: &[f64]) -> f64 {
    let non_zero: Vec<f64> = arr.iter().filter(|&&v| v != 0.0).copied().collect();
    if non_zero.is_empty() { return 0.0; }
    non_zero.len() as f64 / non_zero.iter().map(|v| 1.0 / v).sum::<f64>()
}

fn trimmed_mean(arr: &[f64], trim: f64) -> f64 {
    if arr.len() < 4 { return mean(arr); }
    let mut sorted = arr.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let cut = (arr.len() as f64 * trim).floor() as usize;
    mean(&sorted[cut..sorted.len() - cut])
}

fn coefficient_of_variation(arr: &[f64]) -> f64 {
    let m = mean(arr);
    if m == 0.0 { return 0.0; }
    (std_dev(arr) / m.abs()) * 100.0
}

fn skewness(arr: &[f64]) -> f64 {
    if arr.len() < 3 { return 0.0; }
    let n = arr.len() as f64;
    let m = mean(arr);
    let m3: f64 = arr.iter().map(|v| (v - m).powi(3)).sum::<f64>() / n;
    let m2: f64 = arr.iter().map(|v| (v - m).powi(2)).sum::<f64>() / n;
    if m2 == 0.0 { return 0.0; }
    m3 / m2.powf(1.5)
}

fn kurtosis(arr: &[f64]) -> f64 {
    if arr.len() < 4 { return 0.0; }
    let n = arr.len() as f64;
    let m = mean(arr);
    let m4: f64 = arr.iter().map(|v| (v - m).powi(4)).sum::<f64>() / n;
    let m2: f64 = arr.iter().map(|v| (v - m).powi(2)).sum::<f64>() / n;
    if m2 == 0.0 { return 0.0; }
    (m4 / m2.powi(2)) - 3.0
}

fn median_absolute_deviation(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    let med = median(arr);
    let deviations: Vec<f64> = arr.iter().map(|v| (v - med).abs()).collect();
    median(&deviations)
}

fn average_absolute_deviation(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    let m = mean(arr);
    mean(&arr.iter().map(|v| (v - m).abs()).collect::<Vec<_>>())
}

fn interquartile_range(arr: &[f64]) -> f64 {
    percentile(arr, 75.0) - percentile(arr, 25.0)
}

fn standard_error(arr: &[f64]) -> f64 {
    if arr.len() < 2 { return 0.0; }
    std_dev(arr) / (arr.len() as f64).sqrt()
}

fn root_mean_square(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    (arr.iter().map(|v| v.powi(2)).sum::<f64>() / arr.len() as f64).sqrt()
}

fn mean_squared_error(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    arr.iter().map(|v| v.powi(2)).sum::<f64>() / arr.len() as f64
}

fn range(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    let min = arr.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = arr.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    max - min
}

struct LinearRegression {
    slope: f64,
    intercept: f64,
    r2: f64,
}

fn linear_regression(arr: &[f64]) -> LinearRegression {
    if arr.len() < 2 {
        return LinearRegression { slope: 0.0, intercept: 0.0, r2: 0.0 };
    }
    let n = arr.len() as f64;
    let x: Vec<f64> = (0..arr.len()).map(|i| i as f64).collect();
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = arr.iter().sum();
    let sum_xy: f64 = x.iter().zip(arr.iter()).map(|(xi, yi)| xi * yi).sum();
    let sum_x2: f64 = x.iter().map(|xi| xi.powi(2)).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
    let intercept = (sum_y - slope * sum_x) / n;

    let ss_res: f64 = arr.iter().enumerate()
        .map(|(i, yi)| (yi - (slope * i as f64 + intercept)).powi(2))
        .sum();
    let m = mean(arr);
    let ss_tot: f64 = arr.iter().map(|yi| (yi - m).powi(2)).sum();
    let r2 = if ss_tot == 0.0 { 1.0 } else { 1.0 - ss_res / ss_tot };

    LinearRegression { slope, intercept, r2 }
}

fn autocorrelation(arr: &[f64], lag: usize) -> f64 {
    if arr.len() <= lag { return 0.0; }
    let m = mean(arr);
    let num: f64 = (0..arr.len() - lag)
        .map(|i| (arr[i] - m) * (arr[i + lag] - m))
        .sum();
    let den: f64 = arr.iter().map(|v| (v - m).powi(2)).sum();
    if den == 0.0 { 0.0 } else { num / den }
}

fn durbin_watson(arr: &[f64]) -> f64 {
    if arr.len() < 2 { return 2.0; }
    let sum_sq_diff: f64 = (1..arr.len())
        .map(|i| (arr[i] - arr[i - 1]).powi(2))
        .sum();
    let sum_sq: f64 = arr.iter().map(|v| v.powi(2)).sum();
    if sum_sq == 0.0 { 2.0 } else { sum_sq_diff / sum_sq }
}

fn count_outliers(arr: &[f64]) -> usize {
    if arr.len() < 4 { return 0; }
    let q1 = percentile(arr, 25.0);
    let q3 = percentile(arr, 75.0);
    let iqr = q3 - q1;
    arr.iter().filter(|&&v| v < q1 - 1.5 * iqr || v > q3 + 1.5 * iqr).count()
}

fn count_z_score_outliers(arr: &[f64], threshold: f64) -> usize {
    if arr.len() < 2 { return 0; }
    let m = mean(arr);
    let s = std_dev(arr);
    if s == 0.0 { return 0; }
    arr.iter().filter(|&&v| ((v - m) / s).abs() > threshold).count()
}

fn allan_deviation(arr: &[f64]) -> f64 {
    if arr.len() < 3 { return 0.0; }
    let sum_sq_diff: f64 = (0..arr.len() - 2)
        .map(|i| (arr[i + 2] - 2.0 * arr[i + 1] + arr[i]).powi(2))
        .sum();
    (sum_sq_diff / (2.0 * (arr.len() - 2) as f64)).sqrt()
}

fn max_time_interval_error(arr: &[f64]) -> f64 {
    if arr.len() < 2 { return 0.0; }
    let mut max_error = 0.0f64;
    let max_window = arr.len().min(100);
    for window in 1..=max_window {
        for i in 0..=arr.len() - window {
            let slice = &arr[i..i + window];
            let min = slice.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = slice.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            max_error = max_error.max(max - min);
        }
    }
    max_error
}

fn time_deviation(arr: &[f64]) -> f64 {
    allan_deviation(arr) / 3.0f64.sqrt()
}

fn jitter(arr: &[f64]) -> f64 {
    if arr.len() < 2 { return 0.0; }
    let diffs: Vec<f64> = (1..arr.len()).map(|i| arr[i] - arr[i - 1]).collect();
    std_dev(&diffs)
}

fn approximate_entropy(arr: &[f64], m: usize) -> f64 {
    if arr.len() < m + 1 { return 0.0; }
    let r = std_dev(arr) * 0.2;
    if r == 0.0 { return 0.0; }

    fn count_matches(template: &[f64], data: &[Vec<f64>], r: f64) -> usize {
        data.iter().filter(|d| {
            template.iter().zip(d.iter()).all(|(t, di)| (t - di).abs() <= r)
        }).count()
    }

    fn patterns(arr: &[f64], len: usize) -> Vec<Vec<f64>> {
        (0..=arr.len() - len).map(|i| arr[i..i + len].to_vec()).collect()
    }

    let patterns_m = patterns(arr, m);
    let patterns_m1 = patterns(arr, m + 1);

    let mut phi_m = 0.0;
    for p in &patterns_m {
        let c = count_matches(p, &patterns_m, r);
        if c > 0 {
            phi_m += (c as f64 / patterns_m.len() as f64).ln();
        }
    }
    phi_m /= patterns_m.len() as f64;

    let mut phi_m1 = 0.0;
    for p in &patterns_m1 {
        let c = count_matches(&p[..m], &patterns_m, r);
        if c > 0 {
            phi_m1 += (c as f64 / patterns_m.len() as f64).ln();
        }
    }
    phi_m1 /= patterns_m1.len() as f64;

    (phi_m - phi_m1).abs()
}

fn sample_entropy(arr: &[f64], m: usize) -> f64 {
    if arr.len() < m + 2 { return 0.0; }
    let r = std_dev(arr) * 0.2;
    if r == 0.0 { return 0.0; }

    let mut a = 0;
    let mut b = 0;

    for i in 0..arr.len() - m {
        for j in (i + 1)..arr.len() - m {
            let mut match_m = true;
            for k in 0..m {
                if (arr[i + k] - arr[j + k]).abs() > r {
                    match_m = false;
                    break;
                }
            }
            if match_m {
                b += 1;
                if (arr[i + m] - arr[j + m]).abs() <= r {
                    a += 1;
                }
            }
        }
    }

    if b == 0 { 0.0 } else { -(a as f64 / b as f64).ln() }
}

fn permutation_entropy(arr: &[f64], m: usize) -> f64 {
    if arr.len() < m { return 0.0; }

    use std::collections::HashMap;
    let mut patterns: HashMap<String, usize> = HashMap::new();

    for i in 0..=arr.len() - m {
        let slice = &arr[i..i + m];
        let mut indexed: Vec<(usize, &f64)> = slice.iter().enumerate().collect();
        indexed.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        let pattern: String = indexed.iter().map(|(idx, _)| idx.to_string()).collect::<Vec<_>>().join(",");
        *patterns.entry(pattern).or_insert(0) += 1;
    }

    let total = (arr.len() - m + 1) as f64;
    let mut entropy = 0.0;
    for &count in patterns.values() {
        let p = count as f64 / total;
        if p > 0.0 {
            entropy -= p * p.log2();
        }
    }

    let max_entropy = (factorial(m) as f64).log2();
    if max_entropy == 0.0 { 0.0 } else { entropy / max_entropy }
}

fn factorial(n: usize) -> usize {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

fn hurst_exponent(arr: &[f64]) -> f64 {
    if arr.len() < 20 { return 0.5; }

    let mut rs_values: Vec<(f64, f64)> = Vec::new();
    let mut n = 10;

    while n <= arr.len() / 2 {
        let num_blocks = arr.len() / n;
        let mut sum_rs = 0.0;

        for b in 0..num_blocks {
            let block = &arr[b * n..(b + 1) * n];
            let m = mean(block);
            let cum_dev: Vec<f64> = (0..block.len())
                .map(|i| block[..=i].iter().map(|x| x - m).sum())
                .collect();
            let r = cum_dev.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                - cum_dev.iter().cloned().fold(f64::INFINITY, f64::min);
            let s = std_dev(block);
            if s > 0.0 {
                sum_rs += r / s;
            }
        }

        if num_blocks > 0 {
            rs_values.push((n as f64, sum_rs / num_blocks as f64));
        }

        n = (n as f64 * 1.5).floor() as usize;
    }

    if rs_values.len() < 2 { return 0.5; }

    let log_n: Vec<f64> = rs_values.iter().map(|(n, _)| n.ln()).collect();
    let log_rs: Vec<f64> = rs_values.iter().map(|(_, rs)| rs.ln()).collect();

    let n_len = log_n.len() as f64;
    let sum_x: f64 = log_n.iter().sum();
    let sum_y: f64 = log_rs.iter().sum();
    let sum_xy: f64 = log_n.iter().zip(log_rs.iter()).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = log_n.iter().map(|x| x.powi(2)).sum();

    let slope = (n_len * sum_xy - sum_x * sum_y) / (n_len * sum_x2 - sum_x.powi(2));
    slope.max(0.0).min(1.0)
}

struct ConsecutiveAnalysis {
    max_pos: usize,
    max_neg: usize,
    pos_count: usize,
    neg_count: usize,
    crossings: usize,
}

fn consecutive_analysis(arr: &[f64]) -> ConsecutiveAnalysis {
    if arr.is_empty() {
        return ConsecutiveAnalysis {
            max_pos: 0, max_neg: 0, pos_count: 0, neg_count: 0, crossings: 0,
        };
    }

    let mut max_pos = 0;
    let mut max_neg = 0;
    let mut cur_pos = 0;
    let mut cur_neg = 0;
    let mut pos_count = 0;
    let mut neg_count = 0;
    let mut crossings = 0;
    let mut prev_sign = arr[0] >= 0.0;

    for (i, &v) in arr.iter().enumerate() {
        if v >= 0.0 {
            pos_count += 1;
            cur_pos += 1;
            cur_neg = 0;
            max_pos = max_pos.max(cur_pos);
        } else {
            neg_count += 1;
            cur_neg += 1;
            cur_pos = 0;
            max_neg = max_neg.max(cur_neg);
        }
        if i > 0 && (v >= 0.0) != prev_sign {
            crossings += 1;
        }
        prev_sign = v >= 0.0;
    }

    ConsecutiveAnalysis { max_pos, max_neg, pos_count, neg_count, crossings }
}

struct ConfidenceInterval {
    lower: f64,
    upper: f64,
    margin: f64,
}

fn confidence_interval(arr: &[f64], confidence: f64) -> ConfidenceInterval {
    let z = if confidence == 0.95 { 1.96 } else if confidence == 0.99 { 2.576 } else { 1.645 };
    let se = standard_error(arr);
    let m = mean(arr);
    ConfidenceInterval {
        lower: m - z * se,
        upper: m + z * se,
        margin: z * se,
    }
}

fn central_moment(arr: &[f64], order: i32) -> f64 {
    if arr.is_empty() { return 0.0; }
    let m = mean(arr);
    arr.iter().map(|v| (v - m).powi(order)).sum::<f64>() / arr.len() as f64
}

fn winsorized_mean(arr: &[f64], pct: f64) -> f64 {
    if arr.len() < 4 { return mean(arr); }
    let mut sorted = arr.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let k = (arr.len() as f64 * pct).floor() as usize;
    for i in 0..k { sorted[i] = sorted[k]; }
    for i in (arr.len() - k)..arr.len() { sorted[i] = sorted[arr.len() - k - 1]; }
    mean(&sorted)
}

fn biweight_mean(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    let med = median(arr);
    let mad_val = median_absolute_deviation(arr);
    if mad_val == 0.0 { return med; }
    let c = 9.0;
    let u: Vec<f64> = arr.iter().map(|x| (x - med) / (c * mad_val)).collect();
    let weights: Vec<f64> = u.iter().map(|ui| if ui.abs() < 1.0 { (1.0 - ui.powi(2)).powi(2) } else { 0.0 }).collect();
    let sum_w: f64 = weights.iter().sum();
    if sum_w == 0.0 { return med; }
    arr.iter().zip(weights.iter()).map(|(x, w)| x * w).sum::<f64>() / sum_w
}

fn midrange(arr: &[f64]) -> f64 {
    if arr.is_empty() { return 0.0; }
    let min = arr.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = arr.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (min + max) / 2.0
}

fn midhinge(arr: &[f64]) -> f64 {
    (percentile(arr, 25.0) + percentile(arr, 75.0)) / 2.0
}

fn trimean(arr: &[f64]) -> f64 {
    (percentile(arr, 25.0) + 2.0 * percentile(arr, 50.0) + percentile(arr, 75.0)) / 4.0
}

fn gini_mean_difference(arr: &[f64]) -> f64 {
    if arr.len() < 2 { return 0.0; }
    let n = arr.len();
    let mut sum = 0.0;
    for i in 0..n {
        for j in (i + 1)..n {
            sum += (arr[i] - arr[j]).abs();
        }
    }
    2.0 * sum / (n * (n - 1)) as f64
}

fn quartile_deviation(arr: &[f64]) -> f64 {
    interquartile_range(arr) / 2.0
}

fn bowley_skewness(arr: &[f64]) -> f64 {
    let q1 = percentile(arr, 25.0);
    let q2 = percentile(arr, 50.0);
    let q3 = percentile(arr, 75.0);
    let denom = q3 - q1;
    if denom == 0.0 { return 0.0; }
    (q3 + q1 - 2.0 * q2) / denom
}

fn pearson_skewness(arr: &[f64]) -> f64 {
    let s = std_dev(arr);
    if s == 0.0 { return 0.0; }
    3.0 * (mean(arr) - median(arr)) / s
}

fn hadamard_variance(arr: &[f64]) -> f64 {
    if arr.len() < 4 { return 0.0; }
    let mut sum = 0.0;
    for i in 0..arr.len() - 3 {
        let h = arr[i] - 3.0 * arr[i + 1] + 3.0 * arr[i + 2] - arr[i + 3];
        sum += h.powi(2);
    }
    sum / (20.0 * (arr.len() - 3) as f64)
}

fn modified_allan_deviation(arr: &[f64]) -> f64 {
    if arr.len() < 4 { return 0.0; }
    let tau = 1;
    let mut sum = 0.0;
    let n = arr.len();
    for i in 0..n - 2 * tau {
        for j in i..i + tau {
            if j + 2 * tau < n {
                let diff = arr[j + 2 * tau] - 2.0 * arr[j + tau] + arr[j];
                sum += diff.powi(2);
            }
        }
    }
    (sum / (2.0 * tau.pow(2) as f64 * (n - 2 * tau) as f64)).sqrt()
}

fn total_variance(arr: &[f64]) -> f64 {
    variance(arr)
}

fn lag1_differences(arr: &[f64]) -> Vec<f64> {
    if arr.len() < 2 { return vec![]; }
    (1..arr.len()).map(|i| arr[i] - arr[i - 1]).collect()
}

fn runs_test_z(arr: &[f64]) -> f64 {
    if arr.len() < 2 { return 0.0; }
    let med = median(arr);
    let signs: Vec<i32> = arr.iter().map(|&v| if v > med { 1 } else { -1 }).collect();
    let n1 = signs.iter().filter(|&&s| s > 0).count() as f64;
    let n2 = signs.iter().filter(|&&s| s < 0).count() as f64;
    let n = n1 + n2;
    if n1 == 0.0 || n2 == 0.0 { return 0.0; }
    let mut runs = 1;
    for i in 1..signs.len() {
        if signs[i] != signs[i - 1] { runs += 1; }
    }
    let expected = 1.0 + 2.0 * n1 * n2 / n;
    let var = 2.0 * n1 * n2 * (2.0 * n1 * n2 - n) / (n.powi(2) * (n - 1.0));
    if var <= 0.0 { return 0.0; }
    (runs as f64 - expected) / var.sqrt()
}

fn turning_points_count(arr: &[f64]) -> usize {
    if arr.len() < 3 { return 0; }
    let mut count = 0;
    for i in 1..arr.len() - 1 {
        if (arr[i] > arr[i - 1] && arr[i] > arr[i + 1]) ||
           (arr[i] < arr[i - 1] && arr[i] < arr[i + 1]) {
            count += 1;
        }
    }
    count
}

fn spectral_flatness(arr: &[f64]) -> f64 {
    if arr.len() < 2 { return 0.0; }
    let squared: Vec<f64> = arr.iter().map(|v| v.powi(2)).collect();
    let geo = geometric_mean(&squared);
    let arith = mean(&squared);
    if arith == 0.0 { return 0.0; }
    geo / arith
}

fn shannon_entropy(arr: &[f64]) -> f64 {
    if arr.len() < 2 { return 0.0; }
    use std::collections::HashMap;
    let mut freq: HashMap<i64, usize> = HashMap::new();
    for &v in arr {
        let bin = (v * 10.0).round() as i64;
        *freq.entry(bin).or_insert(0) += 1;
    }
    let n = arr.len() as f64;
    freq.values().filter(|&&c| c > 0).map(|&c| {
        let p = c as f64 / n;
        -p * p.ln()
    }).sum()
}

fn renyi_entropy(arr: &[f64], alpha: f64) -> f64 {
    if arr.len() < 2 || alpha == 1.0 { return shannon_entropy(arr); }
    use std::collections::HashMap;
    let mut freq: HashMap<i64, usize> = HashMap::new();
    for &v in arr {
        let bin = (v * 10.0).round() as i64;
        *freq.entry(bin).or_insert(0) += 1;
    }
    let n = arr.len() as f64;
    let sum: f64 = freq.values().map(|&c| (c as f64 / n).powf(alpha)).sum();
    if sum <= 0.0 { return 0.0; }
    sum.ln() / (1.0 - alpha)
}

fn tsallis_entropy(arr: &[f64], q: f64) -> f64 {
    if arr.len() < 2 { return 0.0; }
    use std::collections::HashMap;
    let mut freq: HashMap<i64, usize> = HashMap::new();
    for &v in arr {
        let bin = (v * 10.0).round() as i64;
        *freq.entry(bin).or_insert(0) += 1;
    }
    let n = arr.len() as f64;
    let sum: f64 = freq.values().map(|&c| (c as f64 / n).powf(q)).sum();
    (1.0 - sum) / (q - 1.0)
}

fn jarque_bera(arr: &[f64]) -> f64 {
    let n = arr.len() as f64;
    let s = skewness(arr);
    let k = kurtosis(arr);
    n / 6.0 * (s.powi(2) + k.powi(2) / 4.0)
}

fn dagostino_k2(arr: &[f64]) -> f64 {
    let s = skewness(arr);
    let k = kurtosis(arr);
    s.powi(2) + k.powi(2)
}

fn cohens_d(arr: &[f64]) -> f64 {
    let s = std_dev(arr);
    if s == 0.0 { return 0.0; }
    mean(arr) / s
}

fn hedges_g(arr: &[f64]) -> f64 {
    let n = arr.len();
    if n < 4 { return cohens_d(arr); }
    let correction = 1.0 - 3.0 / (4.0 * (n as f64) - 9.0);
    cohens_d(arr) * correction
}

fn rolling_mean(arr: &[f64], window: usize) -> f64 {
    if arr.len() < window { return mean(arr); }
    mean(&arr[arr.len() - window..])
}

fn rolling_std(arr: &[f64], window: usize) -> f64 {
    if arr.len() < window { return std_dev(arr); }
    std_dev(&arr[arr.len() - window..])
}

fn ewma(arr: &[f64], alpha: f64) -> f64 {
    if arr.is_empty() { return 0.0; }
    let mut result = arr[0];
    for &v in arr.iter().skip(1) {
        result = alpha * v + (1.0 - alpha) * result;
    }
    result
}

fn crest_factor(arr: &[f64]) -> f64 {
    let rms_val = root_mean_square(arr);
    if rms_val == 0.0 { return 0.0; }
    let peak = arr.iter().map(|v| v.abs()).fold(0.0f64, f64::max);
    peak / rms_val
}

fn impulse_factor(arr: &[f64]) -> f64 {
    let mean_abs = mean(&arr.iter().map(|v| v.abs()).collect::<Vec<_>>());
    if mean_abs == 0.0 { return 0.0; }
    let peak = arr.iter().map(|v| v.abs()).fold(0.0f64, f64::max);
    peak / mean_abs
}

fn shape_factor(arr: &[f64]) -> f64 {
    let mean_abs = mean(&arr.iter().map(|v| v.abs()).collect::<Vec<_>>());
    if mean_abs == 0.0 { return 0.0; }
    root_mean_square(arr) / mean_abs
}

fn clearance_factor(arr: &[f64]) -> f64 {
    let sqrt_mean = mean(&arr.iter().map(|v| v.abs().sqrt()).collect::<Vec<_>>());
    if sqrt_mean == 0.0 { return 0.0; }
    let peak = arr.iter().map(|v| v.abs()).fold(0.0f64, f64::max);
    peak / sqrt_mean.powi(2)
}

fn trend_strength(arr: &[f64]) -> f64 {
    let reg = linear_regression(arr);
    reg.r2.sqrt()
}

fn partial_autocorrelation(arr: &[f64], lag: usize) -> f64 {
    if arr.len() <= lag { return 0.0; }
    if lag == 1 { return autocorrelation(arr, 1); }
    let rho: Vec<f64> = (1..=lag).map(|k| autocorrelation(arr, k)).collect();
    if lag == 2 {
        let r1 = rho[0];
        let r2 = rho[1];
        return (r2 - r1.powi(2)) / (1.0 - r1.powi(2));
    }
    rho[lag - 1]
}

fn ljung_box_q(arr: &[f64], max_lag: usize) -> f64 {
    let n = arr.len() as f64;
    let mut q = 0.0;
    for k in 1..=max_lag {
        let rk = autocorrelation(arr, k);
        q += rk.powi(2) / (n - k as f64);
    }
    n * (n + 2.0) * q
}

fn box_pierce_q(arr: &[f64], max_lag: usize) -> f64 {
    let n = arr.len() as f64;
    let mut q = 0.0;
    for k in 1..=max_lag {
        let rk = autocorrelation(arr, k);
        q += rk.powi(2);
    }
    n * q
}

fn recurrence_rate(arr: &[f64], threshold: f64) -> f64 {
    if arr.len() < 2 { return 0.0; }
    let n = arr.len();
    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if (arr[i] - arr[j]).abs() < threshold {
                count += 1;
            }
        }
    }
    2.0 * count as f64 / (n * (n - 1)) as f64
}

fn steady_state_detection(arr: &[f64]) -> (f64, f64, usize) {
    if arr.len() < 10 { return (mean(arr), variance(arr), 0); }
    let window = arr.len().min(20);
    let tail = &arr[arr.len() - window..];
    (mean(tail), variance(tail), arr.len().saturating_sub(window))
}

fn simple_forecast(arr: &[f64], steps: usize) -> f64 {
    let reg = linear_regression(arr);
    reg.slope * (arr.len() + steps) as f64 + reg.intercept
}

pub fn calculate_stats(history: &[HistoryEntry]) -> Option<StatsResult> {
    if history.is_empty() { return None; }

    let offsets: Vec<f64> = history.iter().map(|h| h.offset).collect();
    let delays: Vec<f64> = history.iter().map(|h| h.delay).collect();
    let abs_offsets: Vec<f64> = offsets.iter().map(|v| v.abs()).collect();

    let offset_reg = linear_regression(&offsets);
    let delay_reg = linear_regression(&delays);
    let consecutive = consecutive_analysis(&offsets);
    let ci95 = confidence_interval(&offsets, 0.95);
    let ci99 = confidence_interval(&offsets, 0.99);

    let std = std_dev(&offsets);
    let stability = if std < 5.0 { "excellent" }
        else if std < 15.0 { "good" }
        else if std < 50.0 { "normal" }
        else { "poor" };

    let skew = skewness(&offsets);
    let kurt = kurtosis(&offsets);
    let skewness_type = if skew > 0.5 { "Right" } else if skew < -0.5 { "Left" } else { "Symmetric" };
    let kurtosis_type = if kurt > 1.0 { "Lepto" } else if kurt < -1.0 { "Platy" } else { "Meso" };

    let trend_direction = if offset_reg.slope.abs() < 0.001 { "stable" }
        else if offset_reg.slope > 0.0 { "up" }
        else { "down" };
    let delay_trend_direction = if delay_reg.slope.abs() < 0.001 { "stable" }
        else if delay_reg.slope > 0.0 { "up" }
        else { "down" };

    let dw_val = durbin_watson(&offsets);
    let dw_status = if dw_val < 1.5 { "Positive" } else if dw_val > 2.5 { "Negative" } else { "None" };

    let bias = if consecutive.pos_count > consecutive.neg_count { "Positive" }
        else if consecutive.pos_count < consecutive.neg_count { "Negative" }
        else { "Balanced" };

    let recent_offsets: Vec<f64> = offsets.iter().rev().take(100).cloned().collect();
    let recent_delays: Vec<f64> = delays.iter().rev().take(100).cloned().collect();

    Some(StatsResult {
        samples: history.len(),
        mean: mean(&offsets),
        mean_delay: mean(&delays),
        median: median(&offsets),
        median_delay: median(&delays),
        mode: mode(&offsets),
        mode_delay: mode(&delays),
        geometric_mean: geometric_mean(&abs_offsets),
        harmonic_mean: harmonic_mean(&abs_offsets),
        trimmed_mean: trimmed_mean(&offsets, 0.1),
        trimmed_mean_delay: trimmed_mean(&delays, 0.1),
        sum: sum(&offsets),
        sum_abs: sum(&abs_offsets),
        sum_delay: sum(&delays),
        min: offsets.iter().cloned().fold(f64::INFINITY, f64::min),
        max: offsets.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        min_delay: delays.iter().cloned().fold(f64::INFINITY, f64::min),
        max_delay: delays.iter().cloned().fold(f64::NEG_INFINITY, f64::max),

        variance: variance(&offsets),
        variance_delay: variance(&delays),
        std_dev: std,
        std_dev_delay: std_dev(&delays),
        cv: coefficient_of_variation(&offsets),
        cv_delay: coefficient_of_variation(&delays),
        mad: median_absolute_deviation(&offsets),
        mad_delay: median_absolute_deviation(&delays),
        aad: average_absolute_deviation(&offsets),
        aad_delay: average_absolute_deviation(&delays),
        iqr: interquartile_range(&offsets),
        iqr_delay: interquartile_range(&delays),
        sem: standard_error(&offsets),
        sem_delay: standard_error(&delays),
        rms: root_mean_square(&offsets),
        rms_delay: root_mean_square(&delays),
        mse: mean_squared_error(&offsets),
        mse_delay: mean_squared_error(&delays),
        range: range(&offsets),
        range_delay: range(&delays),

        skewness: skew,
        skewness_delay: skewness(&delays),
        kurtosis: kurt,
        kurtosis_delay: kurtosis(&delays),
        skewness_abs: skewness(&abs_offsets),
        kurtosis_abs: kurtosis(&abs_offsets),
        skewness_type: skewness_type.to_string(),
        kurtosis_type: kurtosis_type.to_string(),

        p1: percentile(&abs_offsets, 1.0),
        p5: percentile(&abs_offsets, 5.0),
        p10: percentile(&abs_offsets, 10.0),
        p25: percentile(&abs_offsets, 25.0),
        p50: percentile(&abs_offsets, 50.0),
        p75: percentile(&abs_offsets, 75.0),
        p90: percentile(&abs_offsets, 90.0),
        p95: percentile(&abs_offsets, 95.0),
        p99: percentile(&abs_offsets, 99.0),

        p1_delay: percentile(&delays, 1.0),
        p5_delay: percentile(&delays, 5.0),
        p10_delay: percentile(&delays, 10.0),
        p25_delay: percentile(&delays, 25.0),
        p50_delay: percentile(&delays, 50.0),
        p75_delay: percentile(&delays, 75.0),
        p90_delay: percentile(&delays, 90.0),
        p95_delay: percentile(&delays, 95.0),
        p99_delay: percentile(&delays, 99.0),

        slope: offset_reg.slope,
        slope_delay: delay_reg.slope,
        intercept: offset_reg.intercept,
        intercept_delay: delay_reg.intercept,
        r2: offset_reg.r2,
        r2_delay: delay_reg.r2,
        trend_direction: trend_direction.to_string(),
        delay_trend_direction: delay_trend_direction.to_string(),
        drift_rate: if history.len() > 1 { (offsets.last().unwrap() - offsets.first().unwrap()) / history.len() as f64 } else { 0.0 },
        drift_total: offsets.last().unwrap_or(&0.0) - offsets.first().unwrap_or(&0.0),
        recent_avg: mean(&offsets.iter().rev().take(10).cloned().collect::<Vec<_>>()),
        older_avg: mean(&offsets.iter().take(offsets.len().saturating_sub(10)).cloned().collect::<Vec<_>>()),

        autocorr1: autocorrelation(&offsets, 1),
        autocorr2: autocorrelation(&offsets, 2),
        autocorr3: autocorrelation(&offsets, 3),
        autocorr5: autocorrelation(&offsets, 5),
        autocorr10: autocorrelation(&offsets, 10),
        dw: dw_val,
        dw_delay: durbin_watson(&delays),
        dw_status: dw_status.to_string(),

        outliers: count_outliers(&offsets),
        outliers_delay: count_outliers(&delays),
        outlier_pct: count_outliers(&offsets) as f64 / offsets.len() as f64 * 100.0,
        outlier_pct_delay: count_outliers(&delays) as f64 / delays.len() as f64 * 100.0,
        z_outliers: count_z_score_outliers(&offsets, 3.0),
        z_outliers_delay: count_z_score_outliers(&delays, 3.0),
        z_outlier_pct: count_z_score_outliers(&offsets, 3.0) as f64 / offsets.len() as f64 * 100.0,
        z_outlier_pct_delay: count_z_score_outliers(&delays, 3.0) as f64 / delays.len() as f64 * 100.0,

        allan: allan_deviation(&offsets),
        allan_delay: allan_deviation(&delays),
        mtie: max_time_interval_error(&offsets),
        mtie_delay: max_time_interval_error(&delays),
        tdev: time_deviation(&offsets),
        tdev_delay: time_deviation(&delays),
        jitter: jitter(&offsets),
        jitter_delay: jitter(&delays),

        approx_ent: approximate_entropy(&recent_offsets, 2),
        approx_ent_delay: approximate_entropy(&recent_delays, 2),
        sample_ent: sample_entropy(&recent_offsets, 2),
        sample_ent_delay: sample_entropy(&recent_delays, 2),
        perm_ent: permutation_entropy(&recent_offsets, 3),
        perm_ent_delay: permutation_entropy(&recent_delays, 3),
        hurst: hurst_exponent(&offsets),
        hurst_delay: hurst_exponent(&delays),

        max_pos: consecutive.max_pos,
        max_neg: consecutive.max_neg,
        pos_count: consecutive.pos_count,
        neg_count: consecutive.neg_count,
        crossings: consecutive.crossings,
        crossing_rate: if history.len() > 1 { consecutive.crossings as f64 / (history.len() - 1) as f64 } else { 0.0 },
        balance: consecutive.pos_count as f64 / history.len() as f64,
        bias: bias.to_string(),
        longest_streak: consecutive.max_pos.max(consecutive.max_neg),

        ci95_lower: ci95.lower,
        ci95_upper: ci95.upper,
        ci95_margin: ci95.margin,
        ci99_lower: ci99.lower,
        ci99_upper: ci99.upper,
        ci99_margin: ci99.margin,

        within_1ms: abs_offsets.iter().filter(|&&v| v <= 1.0).count() as f64 / abs_offsets.len() as f64 * 100.0,
        within_5ms: abs_offsets.iter().filter(|&&v| v <= 5.0).count() as f64 / abs_offsets.len() as f64 * 100.0,
        within_10ms: abs_offsets.iter().filter(|&&v| v <= 10.0).count() as f64 / abs_offsets.len() as f64 * 100.0,
        within_25ms: abs_offsets.iter().filter(|&&v| v <= 25.0).count() as f64 / abs_offsets.len() as f64 * 100.0,
        within_50ms: abs_offsets.iter().filter(|&&v| v <= 50.0).count() as f64 / abs_offsets.len() as f64 * 100.0,
        within_100ms: abs_offsets.iter().filter(|&&v| v <= 100.0).count() as f64 / abs_offsets.len() as f64 * 100.0,
        within_500ms: abs_offsets.iter().filter(|&&v| v <= 500.0).count() as f64 / abs_offsets.len() as f64 * 100.0,
        within_1s: abs_offsets.iter().filter(|&&v| v <= 1000.0).count() as f64 / abs_offsets.len() as f64 * 100.0,

        stability: stability.to_string(),

        moment3: central_moment(&offsets, 3),
        moment4: central_moment(&offsets, 4),
        moment5: central_moment(&offsets, 5),
        moment6: central_moment(&offsets, 6),

        winsorized_mean: winsorized_mean(&offsets, 0.1),
        biweight_mean: biweight_mean(&offsets),
        midrange: midrange(&offsets),
        midhinge: midhinge(&offsets),
        trimean: trimean(&offsets),

        gini_mean_diff: gini_mean_difference(&recent_offsets),
        quartile_dev: quartile_deviation(&offsets),
        decile_range: percentile(&abs_offsets, 90.0) - percentile(&abs_offsets, 10.0),
        interdecile_range: percentile(&offsets, 90.0) - percentile(&offsets, 10.0),

        bowley_skewness: bowley_skewness(&offsets),
        pearson_skewness: pearson_skewness(&offsets),
        excess_kurtosis: kurt,

        hadamard_variance: hadamard_variance(&offsets),
        modified_allan: modified_allan_deviation(&offsets),
        theo1: allan_deviation(&offsets) * 3.0f64.sqrt(),
        total_variance: total_variance(&offsets),

        lag1_diff_mean: mean(&lag1_differences(&offsets)),
        lag1_diff_std: std_dev(&lag1_differences(&offsets)),
        runs_test_z: runs_test_z(&offsets),
        turning_points: turning_points_count(&offsets),
        turning_point_rate: if history.len() > 2 { turning_points_count(&offsets) as f64 / (history.len() - 2) as f64 } else { 0.0 },

        spectral_flatness: spectral_flatness(&recent_offsets),
        spectral_entropy: shannon_entropy(&recent_offsets),
        dominant_freq: 0.0,

        shannon_entropy: shannon_entropy(&recent_offsets),
        renyi_entropy: renyi_entropy(&recent_offsets, 2.0),
        tsallis_entropy: tsallis_entropy(&recent_offsets, 2.0),

        jarque_bera: jarque_bera(&offsets),
        dagostino_k2: dagostino_k2(&offsets),
        shapiro_wilk_approx: 1.0 - jarque_bera(&offsets) / (history.len() as f64 * 10.0).min(1.0),

        cohens_d: cohens_d(&offsets),
        hedges_g: hedges_g(&offsets),
        glass_delta: cohens_d(&offsets),

        rolling_mean_5: rolling_mean(&offsets, 5),
        rolling_std_5: rolling_std(&offsets, 5),
        rolling_mean_10: rolling_mean(&offsets, 10),
        rolling_std_10: rolling_std(&offsets, 10),
        ewma: ewma(&offsets, 0.3),

        peak_to_peak: range(&offsets),
        crest_factor: crest_factor(&offsets),
        impulse_factor: impulse_factor(&offsets),
        shape_factor: shape_factor(&offsets),
        clearance_factor: clearance_factor(&offsets),

        trend_strength: trend_strength(&offsets),
        seasonality_strength: 0.0,
        stl_trend: offset_reg.slope * history.len() as f64 / 2.0,
        stl_remainder_var: variance(&offsets) * (1.0 - offset_reg.r2),

        partial_autocorr1: partial_autocorrelation(&offsets, 1),
        partial_autocorr2: partial_autocorrelation(&offsets, 2),
        ljung_box_q: ljung_box_q(&offsets, 10.min(history.len() / 5)),
        box_pierce_q: box_pierce_q(&offsets, 10.min(history.len() / 5)),

        lyapunov_approx: hurst_exponent(&offsets) - 0.5,
        correlation_dim: 2.0 * hurst_exponent(&offsets),
        recurrence_rate: recurrence_rate(&recent_offsets, std * 0.1),
        determinism: 1.0 - permutation_entropy(&recent_offsets, 3),

        embedding_dim: 3,
        false_nearest: 0.0,
        average_mutual_info: shannon_entropy(&recent_offsets),

        largest_lyapunov: hurst_exponent(&offsets) - 0.5,
        kaplan_yorke_dim: 1.0 + hurst_exponent(&offsets),

        steady_state_mean: steady_state_detection(&offsets).0,
        steady_state_var: steady_state_detection(&offsets).1,
        transient_length: steady_state_detection(&offsets).2,

        forecast_1: simple_forecast(&offsets, 1),
        forecast_5: simple_forecast(&offsets, 5),
        forecast_10: simple_forecast(&offsets, 10),
        prediction_interval_lower: simple_forecast(&offsets, 1) - 1.96 * std,
        prediction_interval_upper: simple_forecast(&offsets, 1) + 1.96 * std,
    })
}

pub fn calculate_autocorr_chart(history: &[HistoryEntry], max_lag: usize) -> Vec<f64> {
    let offsets: Vec<f64> = history.iter().map(|h| h.offset).collect();
    (1..=max_lag).map(|lag| autocorrelation(&offsets, lag)).collect()
}

#[tauri::command]
pub async fn calculate_history_stats(history: Vec<HistoryEntry>) -> Option<StatsResult> {
    calculate_stats(&history)
}

#[tauri::command]
pub async fn calculate_autocorr_data(history: Vec<HistoryEntry>, max_lag: usize) -> Vec<f64> {
    calculate_autocorr_chart(&history, max_lag)
}
