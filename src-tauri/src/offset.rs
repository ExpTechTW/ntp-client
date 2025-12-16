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
    let secs = (unix_ms / 1000.0) as i64;
    let nanos = ((unix_ms % 1000.0) * 1_000_000.0) as u32;

    let datetime = chrono::DateTime::from_timestamp(secs, nanos).ok_or_else(|| SetTimeError {
        success: false,
        error: "Invalid timestamp".to_string(),
        code: "INVALID_TIMESTAMP".to_string(),
    })?;

    let local: chrono::DateTime<chrono::Local> = datetime.into();
    let time_str = local.format("%Y-%m-%d %H:%M:%S.%3f").to_string();

    let output = Command::new("powershell")
        .args(["-Command", &format!("Set-Date -Date '{}'", time_str)])
        .output()
        .map_err(|e| SetTimeError {
            success: false,
            error: format!("Failed to execute PowerShell: {}", e),
            code: "EXEC_ERROR".to_string(),
        })?;

    if output.status.success() {
        Ok(format!("System time set to {}", time_str))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(SetTimeError {
            success: false,
            error: format!("Failed to set time: {}", stderr.trim()),
            code: if stderr.contains("Access") || stderr.contains("denied") {
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

    let ntp_result = match ntp::query_ntp(&server) {
        Ok(r) => {
            println!(
                "[SYNC] NTP 查詢: delay={:.3}ms offset={:.3}ms",
                r.delay, r.offset
            );
            r
        }
        Err(e) => {
            println!("[SYNC] NTP 查詢失敗: {}", e.error);
            return serde_json::to_string(&SyncError {
                success: false,
                error: e.error,
                code: e.code,
            })
            .map_err(|e| e.to_string());
        }
    };

    // === 輔助函數：執行一次精準同步 ===
    // 等到本地時間到達 wait_until_local 時，設定系統時間為 target_ms
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

    // === 第一次同步：修正本地大偏差 ===
    let correct_time_at_t4 = ntp_result.t4 + ntp_result.offset;
    let now_local = get_current_time_ms();
    let elapsed_since_t4 = now_local - ntp_result.t4;
    let correct_time_now = correct_time_at_t4 + elapsed_since_t4;
    let next_second = ((correct_time_now / 1000.0).floor() + 1.0) * 1000.0;

    // 當正確時間到達 next_second 時，本地時間應該是多少？
    let wait_until_local = now_local + (next_second - correct_time_now);

    println!("[SYNC] 第一次同步: 修正本地大偏差, 目標={:.3}", next_second);

    if let Err(e) = do_sync(next_second, wait_until_local) {
        println!("[SYNC] 第一次同步失敗: {}", e.error);
        return serde_json::to_string(&SyncError {
            success: false,
            error: e.error,
            code: e.code,
        })
        .map_err(|e| e.to_string());
    }

    // === 第二次同步：測量 estimated_cmd_delay_ms ===
    // 第一次同步後，本地時間應該接近正確，直接用本地時間計算
    std::thread::sleep(std::time::Duration::from_millis(200));

    let now_local2 = get_current_time_ms();
    let next_second2 = ((now_local2 / 1000.0).floor() + 1.0) * 1000.0;
    let wait_until_local2 = next_second2; // 本地時間 = 正確時間

    println!("[SYNC] 第二次同步: 測量執行延遲, 目標={:.3}", next_second2);

    if let Err(e) = do_sync(next_second2, wait_until_local2) {
        println!("[SYNC] 第二次同步失敗: {}", e.error);
        return serde_json::to_string(&SyncError {
            success: false,
            error: e.error,
            code: e.code,
        })
        .map_err(|e| e.to_string());
    }

    // 測量第二次同步後的 offset = 執行延遲造成的誤差
    std::thread::sleep(std::time::Duration::from_millis(100));
    let measured_offset = match ntp::query_ntp(&server) {
        Ok(r) => {
            println!("[SYNC] 第二次同步後 offset={:.3}ms (執行延遲造成的誤差)", r.offset);
            r.offset
        }
        Err(_) => {
            println!("[SYNC] 無法測量第二次同步結果");
            0.0
        }
    };

    // === 第三次同步：補償執行延遲 ===
    // measured_offset > 0 表示本地快了（settimeofday 執行太慢）
    // 需要提前執行 settimeofday，即減少等待時間
    let now_local3 = get_current_time_ms();
    let correct_time_now3 = now_local3 + measured_offset;
    let mut next_second3 = ((correct_time_now3 / 1000.0).floor() + 1.0) * 1000.0;

    // 原本要等到本地時間 = next_second3 時執行
    // 但要提前 measured_offset 毫秒執行
    let mut wait_until_local3 = next_second3 - measured_offset;

    // 如果已經錯過執行時間，延後一個週期
    if wait_until_local3 < now_local3 + 10.0 {
        next_second3 += 1000.0;
        wait_until_local3 += 1000.0;
        println!("[SYNC] 錯過執行時間，延後一個週期");
    }

    println!(
        "[SYNC] 第三次同步: 目標={:.3} 提前={:.3}ms 等待={:.3}ms",
        next_second3, measured_offset, wait_until_local3 - now_local3
    );

    if let Err(e) = do_sync(next_second3, wait_until_local3) {
        println!("[SYNC] 第三次同步失敗: {}", e.error);
        return serde_json::to_string(&SyncError {
            success: false,
            error: e.error,
            code: e.code,
        })
        .map_err(|e| e.to_string());
    }

    // 驗證最終結果
    std::thread::sleep(std::time::Duration::from_millis(100));
    let new_time = get_current_time_ms();

    let post_sync_offset = match ntp::query_ntp(&server) {
        Ok(verify_result) => {
            println!(
                "[SYNC] 最終驗證: offset={:.3}ms delay={:.3}ms",
                verify_result.offset, verify_result.delay
            );
            verify_result.offset
        }
        Err(_) => {
            println!("[SYNC] 驗證查詢失敗");
            0.0
        }
    };

    println!(
        "[SYNC] 完成: 原始偏差={:.3}ms 最終偏差={:.3}ms 執行延遲={:.3}ms",
        ntp_result.offset, post_sync_offset, measured_offset
    );

    serde_json::to_string(&SyncResult {
        success: true,
        message: format!("三次同步完成，補償延遲 {:.1}ms", measured_offset),
        server: ntp_result.server,
        server_ip: ntp_result.server_ip,
        offset: post_sync_offset,
        delay: ntp_result.delay,
        previous_time,
        new_time,
        t1: ntp_result.t1,
        t2: ntp_result.t2,
        t3: ntp_result.t3,
        t4: ntp_result.t4,
    })
    .map_err(|e| e.to_string())
}
