use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ntp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTimeResult {
    pub success: bool,
    pub message: String,
    pub previous_time: Option<f64>,
    pub new_time: Option<f64>,
    pub adjusted_ms: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTimeError {
    pub success: bool,
    pub error: String,
    pub code: String,
}

fn get_current_time_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

pub fn adjust_system_time(offset_ms: f64) -> Result<SetTimeResult, SetTimeError> {
    let previous_time = get_current_time_ms();
    let target_time_ms = previous_time + offset_ms;
    set_time_internal(target_time_ms, previous_time, offset_ms)
}

pub fn set_system_time(unix_ms: f64) -> Result<SetTimeResult, SetTimeError> {
    let previous_time = get_current_time_ms();
    let offset_ms = unix_ms - previous_time;
    set_time_internal(unix_ms, previous_time, offset_ms)
}

fn set_time_internal(
    target_ms: f64,
    previous_time: f64,
    offset_ms: f64,
) -> Result<SetTimeResult, SetTimeError> {
    #[cfg(target_os = "windows")]
    let result = set_time_windows(target_ms);

    #[cfg(target_os = "macos")]
    let result = set_time_macos(target_ms);

    #[cfg(target_os = "linux")]
    let result = set_time_linux(target_ms);

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let result: Result<String, SetTimeError> = Err(SetTimeError {
        success: false,
        error: "Unsupported OS".to_string(),
        code: "UNSUPPORTED_OS".to_string(),
    });

    match result {
        Ok(msg) => {
            let new_time = get_current_time_ms();
            Ok(SetTimeResult {
                success: true,
                message: msg,
                previous_time: Some(previous_time),
                new_time: Some(new_time),
                adjusted_ms: Some(offset_ms),
            })
        }
        Err(e) => Err(e),
    }
}

#[cfg(target_os = "windows")]
fn set_time_windows(unix_ms: f64) -> Result<String, SetTimeError> {
    use chrono::{Datelike, Timelike};
    use windows_sys::Win32::Foundation::GetLastError;
    use windows_sys::Win32::System::SystemInformation::{SetSystemTime, SYSTEMTIME};

    let secs = (unix_ms / 1000.0) as i64;
    let millis = (unix_ms % 1000.0) as u16;

    let datetime = chrono::DateTime::from_timestamp(secs, 0).ok_or_else(|| SetTimeError {
        success: false,
        error: "Invalid timestamp".to_string(),
        code: "INVALID_TIMESTAMP".to_string(),
    })?;

    let utc: chrono::DateTime<chrono::Utc> = datetime.into();

    let system_time = SYSTEMTIME {
        wYear: utc.year() as u16,
        wMonth: utc.month() as u16,
        wDayOfWeek: utc.weekday().num_days_from_sunday() as u16,
        wDay: utc.day() as u16,
        wHour: utc.hour() as u16,
        wMinute: utc.minute() as u16,
        wSecond: utc.second() as u16,
        wMilliseconds: millis,
    };

    let result = unsafe { SetSystemTime(&system_time) };

    if result != 0 {
        Ok(format!(
            "System time set (UTC): {:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            system_time.wYear,
            system_time.wMonth,
            system_time.wDay,
            system_time.wHour,
            system_time.wMinute,
            system_time.wSecond,
            system_time.wMilliseconds
        ))
    } else {
        let error_code = unsafe { GetLastError() };
        Err(SetTimeError {
            success: false,
            error: format!("SetSystemTime failed, error code: {}", error_code),
            code: if error_code == 5 {
                "PERMISSION_DENIED".to_string()
            } else {
                "SET_TIME_ERROR".to_string()
            },
        })
    }
}

#[cfg(target_os = "macos")]
fn set_time_macos(unix_ms: f64) -> Result<String, SetTimeError> {
    let secs = (unix_ms / 1000.0).floor() as i64;
    let usecs = ((unix_ms % 1000.0) * 1000.0) as i64;

    let tv = libc::timeval {
        tv_sec: secs,
        tv_usec: usecs as i32,
    };

    let result = unsafe { libc::settimeofday(&tv, std::ptr::null()) };

    if result == 0 {
        let datetime = chrono::DateTime::from_timestamp(secs, (usecs * 1000) as u32)
            .unwrap_or_else(|| chrono::DateTime::from_timestamp(secs, 0).unwrap());
        return Ok(format!(
            "System time set (UTC): {}.{:03}",
            datetime.format("%Y-%m-%d %H:%M:%S"),
            (usecs / 1000)
        ));
    }

    let datetime = chrono::DateTime::from_timestamp(secs, 0).ok_or_else(|| SetTimeError {
        success: false,
        error: "Invalid timestamp".to_string(),
        code: "INVALID_TIMESTAMP".to_string(),
    })?;

    let date_str = datetime.format("%m%d%H%M%Y.%S").to_string();

    if let Ok(output) = Command::new("sudo")
        .args(["-n", "date", "-u", &date_str])
        .output()
    {
        if output.status.success() {
            return Ok(format!(
                "System time set (UTC): {}",
                datetime.format("%Y-%m-%d %H:%M:%S")
            ));
        }
    }

    Err(SetTimeError {
        success: false,
        error: "需要管理員權限".to_string(),
        code: "PERMISSION_DENIED".to_string(),
    })
}


#[cfg(target_os = "linux")]
fn set_time_linux(unix_ms: f64) -> Result<String, SetTimeError> {
    let secs = (unix_ms / 1000.0) as i64;
    let nanos = ((unix_ms % 1000.0) * 1_000_000.0) as u32;

    let datetime = chrono::DateTime::from_timestamp(secs, nanos).ok_or_else(|| SetTimeError {
        success: false,
        error: "Invalid timestamp".to_string(),
        code: "INVALID_TIMESTAMP".to_string(),
    })?;

    let date_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    if let Ok(output) = Command::new("pkexec")
        .args(["timedatectl", "set-time", &date_str])
        .output()
    {
        if output.status.success() {
            return Ok(format!("System time set via timedatectl: {}", date_str));
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("dismissed") || stderr.contains("canceled") {
            return Err(SetTimeError {
                success: false,
                error: "User canceled".to_string(),
                code: "USER_CANCELED".to_string(),
            });
        }
    }

    let output = Command::new("pkexec")
        .args(["date", "-s", &date_str])
        .output()
        .map_err(|e| SetTimeError {
            success: false,
            error: format!("Failed to execute pkexec: {}", e),
            code: "EXEC_ERROR".to_string(),
        })?;

    if output.status.success() {
        Ok(format!("System time set via date command: {}", date_str))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(SetTimeError {
            success: false,
            error: format!("Failed to set time: {}", stderr.trim()),
            code: if stderr.contains("dismissed") || stderr.contains("canceled") {
                "USER_CANCELED".to_string()
            } else if stderr.contains("permission") || stderr.contains("not authorized") {
                "PERMISSION_DENIED".to_string()
            } else {
                "SET_TIME_ERROR".to_string()
            },
        })
    }
}

#[tauri::command]
pub async fn adjust_time_by_offset(offset_ms: f64) -> Result<String, String> {
    println!("[TIME] Adjusting system time, offset = {:.3} ms", offset_ms);

    match adjust_system_time(offset_ms) {
        Ok(result) => {
            println!(
                "[TIME] OK: {:.3} -> {:.3} (adjusted {:.3} ms)",
                result.previous_time.unwrap_or(0.0),
                result.new_time.unwrap_or(0.0),
                result.adjusted_ms.unwrap_or(0.0)
            );
            serde_json::to_string(&result).map_err(|e| e.to_string())
        }
        Err(error) => {
            println!("[TIME] ERR: {} ({})", error.error, error.code);
            serde_json::to_string(&error).map_err(|e| e.to_string())
        }
    }
}

#[tauri::command]
pub async fn set_system_time_ms(unix_ms: f64) -> Result<String, String> {
    println!("[TIME] Setting system time to {:.3} ms", unix_ms);

    match set_system_time(unix_ms) {
        Ok(result) => {
            println!(
                "[TIME] OK: {:.3} -> {:.3}",
                result.previous_time.unwrap_or(0.0),
                result.new_time.unwrap_or(0.0)
            );
            serde_json::to_string(&result).map_err(|e| e.to_string())
        }
        Err(error) => {
            println!("[TIME] ERR: {} ({})", error.error, error.code);
            serde_json::to_string(&error).map_err(|e| e.to_string())
        }
    }
}

#[tauri::command]
pub async fn check_time_permission() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("net").args(["session"]).output();
        let is_admin = output.map(|o| o.status.success()).unwrap_or(false);

        Ok(serde_json::json!({
            "has_permission": is_admin,
            "platform": "windows",
            "message": if is_admin { "Running as Administrator" } else { "Requires Administrator" }
        })
        .to_string())
    }

    #[cfg(target_os = "macos")]
    {
        Ok(serde_json::json!({
            "has_permission": true,
            "platform": "macos",
            "message": "Will prompt for password"
        })
        .to_string())
    }

    #[cfg(target_os = "linux")]
    {
        Ok(serde_json::json!({
            "has_permission": true,
            "platform": "linux",
            "message": "Will prompt for password"
        })
        .to_string())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok(serde_json::json!({
            "has_permission": false,
            "platform": "unknown",
            "message": "Unsupported OS"
        })
        .to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub message: String,
    pub server: String,
    pub server_ip: String,
    pub offset: f64,
    pub delay: f64,
    pub previous_time: f64,
    pub new_time: f64,
    pub t1: f64,
    pub t2: f64,
    pub t3: f64,
    pub t4: f64,
    pub pre_sync_offset: f64,
    pub post_sync_offset: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncError {
    pub success: bool,
    pub error: String,
    pub code: String,
}

#[tauri::command]
pub async fn sync_ntp_time(server: String) -> Result<String, String> {
    println!("[SYNC] 開始同步: {}", server);

    let previous_time = get_current_time_ms();

    // === 3 次 NTP 測量取中位數 ===
    let mut offsets: Vec<f64> = Vec::new();
    let mut delays: Vec<f64> = Vec::new();
    let mut last_result: Option<ntp::NtpResult> = None;

    for i in 1..=3 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        match ntp::query_ntp(&server) {
            Ok(r) => {
                println!(
                    "[SYNC] 測量 {}/3: offset={:.3}ms delay={:.3}ms",
                    i, r.offset, r.delay
                );
                offsets.push(r.offset);
                delays.push(r.delay);
                last_result = Some(r);
            }
            Err(e) => {
                println!("[SYNC] 測量 {}/3 失敗: {}", i, e.error);
            }
        }
    }

    if offsets.is_empty() {
        return serde_json::to_string(&SyncError {
            success: false,
            error: "所有 NTP 查詢都失敗".to_string(),
            code: "NTP_ERROR".to_string(),
        })
        .map_err(|e| e.to_string());
    }

    // 取中位數
    offsets.sort_by(|a, b| a.partial_cmp(b).unwrap());
    delays.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_offset = offsets[offsets.len() / 2];
    let median_delay = delays[delays.len() / 2];
    let ntp_result = last_result.unwrap();

    println!(
        "[SYNC] 中位數: offset={:.3}ms delay={:.3}ms (共{}次測量)",
        median_offset, median_delay, offsets.len()
    );

    // === 精準同步：等到整秒時設定時間 ===
    fn do_sync(target_ms: f64, wait_until_local: f64) -> Result<(), SetTimeError> {
        let wait_ms = wait_until_local - get_current_time_ms();

        if wait_ms > 5.0 && wait_ms < 2000.0 {
            let sleep_ms = wait_ms - 2.0;
            if sleep_ms > 0.0 {
                std::thread::sleep(std::time::Duration::from_micros((sleep_ms * 1000.0) as u64));
            }
            loop {
                if get_current_time_ms() >= wait_until_local {
                    break;
                }
                std::hint::spin_loop();
            }
        }

        set_system_time(target_ms)?;
        Ok(())
    }

    // 計算目標時間
    let now_local = get_current_time_ms();
    let correct_time_now = now_local + median_offset;
    let next_second = ((correct_time_now / 1000.0).floor() + 1.0) * 1000.0;
    let wait_until_local = now_local + (next_second - correct_time_now);

    println!(
        "[SYNC] 同步: 目標={:.3} 等待={:.3}ms",
        next_second,
        wait_until_local - now_local
    );

    if let Err(e) = do_sync(next_second, wait_until_local) {
        println!("[SYNC] 同步失敗: {}", e.error);
        return serde_json::to_string(&SyncError {
            success: false,
            error: e.error,
            code: e.code,
        })
        .map_err(|e| e.to_string());
    }

    // 驗證結果
    std::thread::sleep(std::time::Duration::from_millis(100));
    let new_time = get_current_time_ms();

    let post_sync_offset = match ntp::query_ntp(&server) {
        Ok(r) => {
            println!("[SYNC] 驗證: offset={:.3}ms delay={:.3}ms", r.offset, r.delay);
            r.offset
        }
        Err(_) => 0.0,
    };

    println!(
        "[SYNC] 完成: 原始偏差={:.3}ms 最終偏差={:.3}ms",
        median_offset, post_sync_offset
    );

    serde_json::to_string(&SyncResult {
        success: true,
        message: format!("同步完成 (3次測量中位數)"),
        server: ntp_result.server,
        server_ip: ntp_result.server_ip,
        offset: post_sync_offset,
        delay: median_delay,
        previous_time,
        new_time,
        t1: ntp_result.t1,
        t2: ntp_result.t2,
        t3: ntp_result.t3,
        t4: ntp_result.t4,
        pre_sync_offset: median_offset,
        post_sync_offset,
    })
    .map_err(|e| e.to_string())
}
