use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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
    let secs = (unix_ms / 1000.0) as i64;

    let datetime = chrono::DateTime::from_timestamp(secs, 0).ok_or_else(|| SetTimeError {
        success: false,
        error: "Invalid timestamp".to_string(),
        code: "INVALID_TIMESTAMP".to_string(),
    })?;

    let date_str = datetime.format("%m%d%H%M%Y.%S").to_string();

    let output = Command::new("sudo")
        .args(["date", "-u", &date_str])
        .output()
        .map_err(|e| SetTimeError {
            success: false,
            error: format!("Failed to execute date command: {}", e),
            code: "EXEC_ERROR".to_string(),
        })?;

    if output.status.success() {
        Ok(format!(
            "System time set (UTC): {}",
            datetime.format("%Y-%m-%d %H:%M:%S")
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(SetTimeError {
            success: false,
            error: format!("Failed to set time: {}", stderr.trim()),
            code: if stderr.contains("permission") || stderr.contains("Password") {
                "PERMISSION_DENIED".to_string()
            } else {
                "SET_TIME_ERROR".to_string()
            },
        })
    }
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

    if let Ok(output) = Command::new("sudo")
        .args(["timedatectl", "set-time", &date_str])
        .output()
    {
        if output.status.success() {
            return Ok(format!("System time set via timedatectl: {}", date_str));
        }
    }

    let output = Command::new("sudo")
        .args(["date", "-s", &date_str])
        .output()
        .map_err(|e| SetTimeError {
            success: false,
            error: format!("Failed to execute date command: {}", e),
            code: "EXEC_ERROR".to_string(),
        })?;

    if output.status.success() {
        Ok(format!("System time set via date command: {}", date_str))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(SetTimeError {
            success: false,
            error: format!("Failed to set time: {}", stderr.trim()),
            code: if stderr.contains("permission") || stderr.contains("Operation not permitted") {
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
        let output = Command::new("sudo").args(["-n", "true"]).output();
        let has_sudo = output.map(|o| o.status.success()).unwrap_or(false);

        Ok(serde_json::json!({
            "has_permission": has_sudo,
            "platform": "macos",
            "message": if has_sudo { "Has sudo privileges" } else { "Requires sudo" }
        })
        .to_string())
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("sudo").args(["-n", "true"]).output();
        let has_sudo = output.map(|o| o.status.success()).unwrap_or(false);

        Ok(serde_json::json!({
            "has_permission": has_sudo,
            "platform": "linux",
            "message": if has_sudo { "Has sudo privileges" } else { "Requires sudo" }
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
