use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutostartResult {
    pub success: bool,
    pub message: String,
}

#[cfg(target_os = "windows")]
const TASK_NAME: &str = "NTPClientAutoStart";

#[cfg(any(target_os = "macos", target_os = "linux"))]
const LAUNCHDAEMON_LABEL: &str = "com.exptech.ntp-client";

/// 啟用開機自啟動（以特權執行）
#[tauri::command]
pub async fn enable_autostart() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        enable_autostart_windows()
    }

    #[cfg(target_os = "macos")]
    {
        enable_autostart_macos()
    }

    #[cfg(target_os = "linux")]
    {
        enable_autostart_linux()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("不支援的作業系統".to_string())
    }
}

/// 停用開機自啟動
#[tauri::command]
pub async fn disable_autostart() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        disable_autostart_windows()
    }

    #[cfg(target_os = "macos")]
    {
        disable_autostart_macos()
    }

    #[cfg(target_os = "linux")]
    {
        disable_autostart_linux()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("不支援的作業系統".to_string())
    }
}

/// 檢查是否已啟用開機自啟動
#[tauri::command]
pub async fn is_autostart_enabled() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        is_autostart_enabled_windows()
    }

    #[cfg(target_os = "macos")]
    {
        is_autostart_enabled_macos()
    }

    #[cfg(target_os = "linux")]
    {
        is_autostart_enabled_linux()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok(serde_json::json!({
            "enabled": false,
            "message": "不支援的作業系統"
        })
        .to_string())
    }
}

// ============ Windows 實作 ============

#[cfg(target_os = "windows")]
fn enable_autostart_windows() -> Result<String, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("無法取得執行檔路徑: {}", e))?;
    let exe_path_str = exe_path.to_string_lossy();

    // 使用 PowerShell 以管理員權限執行 schtasks
    // Task Scheduler 可以以 HIGHEST 權限執行，無需每次 UAC 提示
    let ps_script = format!(
        r#"
        $taskName = '{}'
        $exePath = '{}'

        # 刪除舊任務（如果存在）
        schtasks /Delete /TN $taskName /F 2>$null

        # 建立新任務：登入時執行，最高權限
        schtasks /Create /TN $taskName /TR "`"$exePath`"" /SC ONLOGON /RL HIGHEST /F

        if ($LASTEXITCODE -eq 0) {{
            Write-Output "SUCCESS"
        }} else {{
            Write-Output "FAILED"
            exit 1
        }}
        "#,
        TASK_NAME, exe_path_str
    );

    // 使用 PowerShell -Verb RunAs 來請求管理員權限
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &format!(
                "Start-Process powershell -Verb RunAs -Wait -ArgumentList '-NoProfile -ExecutionPolicy Bypass -Command \"{}\"'",
                ps_script.replace("\"", "`\"").replace("\n", " ")
            ),
        ])
        .output()
        .map_err(|e| format!("執行失敗: {}", e))?;

    if output.status.success() {
        Ok(serde_json::json!({
            "success": true,
            "message": "已啟用開機自啟動（工作排程器）"
        })
        .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("啟用失敗: {}", stderr))
    }
}

#[cfg(target_os = "windows")]
fn disable_autostart_windows() -> Result<String, String> {
    let output = Command::new("schtasks")
        .args(["/Delete", "/TN", TASK_NAME, "/F"])
        .output()
        .map_err(|e| format!("執行失敗: {}", e))?;

    if output.status.success() {
        Ok(serde_json::json!({
            "success": true,
            "message": "已停用開機自啟動"
        })
        .to_string())
    } else {
        // 如果任務不存在也算成功
        Ok(serde_json::json!({
            "success": true,
            "message": "開機自啟動已停用"
        })
        .to_string())
    }
}

#[cfg(target_os = "windows")]
fn is_autostart_enabled_windows() -> Result<String, String> {
    let output = Command::new("schtasks")
        .args(["/Query", "/TN", TASK_NAME])
        .output()
        .map_err(|e| format!("查詢失敗: {}", e))?;

    let enabled = output.status.success();

    Ok(serde_json::json!({
        "enabled": enabled,
        "message": if enabled { "已啟用" } else { "未啟用" }
    })
    .to_string())
}

// ============ macOS 實作 ============
// 使用 LaunchDaemon（系統級別）以 root 執行應用程式
// 這樣應用程式啟動時就有權限設定系統時間，不需要每次都輸入密碼

#[cfg(target_os = "macos")]
fn get_launchdaemon_path() -> String {
    format!("/Library/LaunchDaemons/{}.plist", LAUNCHDAEMON_LABEL)
}

#[cfg(target_os = "macos")]
fn enable_autostart_macos() -> Result<String, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("無法取得執行檔路徑: {}", e))?;

    // 取得實際執行檔路徑（在 .app/Contents/MacOS/ 內）
    let exe_path_str = exe_path.to_string_lossy().to_string();

    let plist_path = get_launchdaemon_path();

    // 建立 LaunchDaemon plist 內容
    // 以 root 執行
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
    <key>StandardOutPath</key>
    <string>/tmp/ntp-client.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/ntp-client.err</string>
</dict>
</plist>"#,
        LAUNCHDAEMON_LABEL, exe_path_str
    );

    // 使用 osascript 請求管理員權限來寫入 /Library/LaunchDaemons/
    // 先將內容寫入臨時檔案，再用 sudo 移動到目標位置
    let temp_path = "/tmp/com.exptech.ntp-client.plist.tmp";
    std::fs::write(temp_path, &plist_content)
        .map_err(|e| format!("無法寫入臨時檔案: {}", e))?;

    let script = format!(
        r#"do shell script "mv '{}' '{}' && chown root:wheel '{}' && chmod 644 '{}'" with administrator privileges"#,
        temp_path, plist_path, plist_path, plist_path
    );

    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|e| format!("執行失敗: {}", e))?;

    if output.status.success() {
        Ok(serde_json::json!({
            "success": true,
            "message": "已啟用開機自啟動（以管理員權限）"
        })
        .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("User canceled") || stderr.contains("-128") {
            Err("使用者取消授權".to_string())
        } else {
            Err(format!("啟用失敗: {}", stderr))
        }
    }
}

#[cfg(target_os = "macos")]
fn disable_autostart_macos() -> Result<String, String> {
    let plist_path = get_launchdaemon_path();

    // 使用 osascript 請求管理員權限來移除檔案
    let script = format!(
        r#"do shell script "launchctl unload -w '{}' 2>/dev/null; rm -f '{}'" with administrator privileges"#,
        plist_path, plist_path
    );

    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|e| format!("執行失敗: {}", e))?;

    if output.status.success() {
        Ok(serde_json::json!({
            "success": true,
            "message": "已停用開機自啟動"
        })
        .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("User canceled") || stderr.contains("-128") {
            Err("使用者取消授權".to_string())
        } else {
            // 如果檔案不存在也算成功
            Ok(serde_json::json!({
                "success": true,
                "message": "開機自啟動已停用"
            })
            .to_string())
        }
    }
}

#[cfg(target_os = "macos")]
fn is_autostart_enabled_macos() -> Result<String, String> {
    let plist_path = get_launchdaemon_path();
    let enabled = std::path::Path::new(&plist_path).exists();

    Ok(serde_json::json!({
        "enabled": enabled,
        "message": if enabled { "已啟用" } else { "未啟用" }
    })
    .to_string())
}

// ============ Linux 實作 ============

#[cfg(target_os = "linux")]
fn get_systemd_service_path() -> String {
    format!(
        "/etc/systemd/system/{}.service",
        LAUNCHDAEMON_LABEL.replace(".", "-")
    )
}

#[cfg(target_os = "linux")]
fn enable_autostart_linux() -> Result<String, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("無法取得執行檔路徑: {}", e))?;
    let exe_path_str = exe_path.to_string_lossy();

    let service_name = LAUNCHDAEMON_LABEL.replace(".", "-");
    let service_path = get_systemd_service_path();

    // 建立 systemd service 內容
    let service_content = format!(
        r#"[Unit]
Description=NTP Client Time Sync
After=network.target

[Service]
Type=simple
ExecStart={}
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
"#,
        exe_path_str
    );

    // 使用 pkexec 請求管理員權限
    let script = format!(
        "echo '{}' > '{}' && systemctl daemon-reload && systemctl enable {}",
        service_content, service_path, service_name
    );

    let output = Command::new("pkexec")
        .args(["bash", "-c", &script])
        .output()
        .map_err(|e| format!("執行失敗: {}", e))?;

    if output.status.success() {
        Ok(serde_json::json!({
            "success": true,
            "message": "已啟用開機自啟動（systemd）"
        })
        .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("dismissed") || stderr.contains("canceled") {
            Err("使用者取消授權".to_string())
        } else {
            Err(format!("啟用失敗: {}", stderr))
        }
    }
}

#[cfg(target_os = "linux")]
fn disable_autostart_linux() -> Result<String, String> {
    let service_name = LAUNCHDAEMON_LABEL.replace(".", "-");
    let service_path = get_systemd_service_path();

    let script = format!(
        "systemctl disable {} 2>/dev/null; rm -f '{}'",
        service_name, service_path
    );

    let output = Command::new("pkexec")
        .args(["bash", "-c", &script])
        .output()
        .map_err(|e| format!("執行失敗: {}", e))?;

    if output.status.success() {
        Ok(serde_json::json!({
            "success": true,
            "message": "已停用開機自啟動"
        })
        .to_string())
    } else {
        Ok(serde_json::json!({
            "success": true,
            "message": "開機自啟動已停用"
        })
        .to_string())
    }
}

#[cfg(target_os = "linux")]
fn is_autostart_enabled_linux() -> Result<String, String> {
    let service_path = get_systemd_service_path();
    let enabled = std::path::Path::new(&service_path).exists();

    Ok(serde_json::json!({
        "enabled": enabled,
        "message": if enabled { "已啟用" } else { "未啟用" }
    })
    .to_string())
}
