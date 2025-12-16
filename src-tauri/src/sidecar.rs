use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use std::process::Command;

#[cfg(target_os = "macos")]
const SIDECAR_PORT: u16 = 12345;
#[cfg(target_os = "macos")]
const SIDECAR_BINARY_NAME: &str = "ntp-client-sidecar";
#[cfg(target_os = "macos")]
const SIDECAR_INSTALL_PATH: &str = "/usr/local/bin/ntp-client-sidecar";
#[cfg(target_os = "macos")]
const LAUNCHDAEMON_LABEL: &str = "com.exptech.ntp-client-sidecar";
#[cfg(target_os = "macos")]
const LAUNCHDAEMON_PATH: &str = "/Library/LaunchDaemons/com.exptech.ntp-client-sidecar.plist";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarStatus {
    pub installed: bool,
    pub running: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTimeRequest {
    pub unix_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTimeResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 檢查 sidecar 是否已安裝並運行
#[tauri::command]
#[cfg(target_os = "macos")]
pub async fn check_sidecar_status() -> Result<String, String> {
    let installed = std::path::Path::new(SIDECAR_INSTALL_PATH).exists()
        && std::path::Path::new(LAUNCHDAEMON_PATH).exists();

    // 檢查 LaunchDaemon 是否已載入
    let running = Command::new("launchctl")
        .args(["list", LAUNCHDAEMON_LABEL])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    // 嘗試連接 UDP 端口來確認服務是否真的在運行
    let udp_available = if running {
        UdpSocket::bind("127.0.0.1:0")
            .and_then(|socket| {
                socket.connect(format!("127.0.0.1:{}", SIDECAR_PORT))
            })
            .is_ok()
    } else {
        false
    };

    Ok(serde_json::json!({
        "installed": installed,
        "running": running && udp_available,
        "message": if installed && running && udp_available {
            "Sidecar 已安裝並運行中"
        } else if installed {
            "Sidecar 已安裝但未運行"
        } else {
            "Sidecar 未安裝"
        }
    })
    .to_string())
}

/// 安裝 sidecar server（使用 AppleScript 請求 sudo 權限）
#[tauri::command]
#[cfg(target_os = "macos")]
pub async fn install_sidecar() -> Result<String, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("無法取得執行檔路徑: {}", e))?;

    // 尋找 sidecar 二進制文件
    // 1. 先檢查開發模式下的構建目錄
    let mut sidecar_paths = Vec::new();
    
    // 開發模式：在 target/debug 或 target/release 目錄中
    if let Ok(current_dir) = std::env::current_dir() {
        sidecar_paths.push(current_dir.join("target/debug/ntp-client-sidecar"));
        sidecar_paths.push(current_dir.join("target/release/ntp-client-sidecar"));
    }
    
    // 生產模式：在 .app bundle 中
    if let Some(app_path) = exe_path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
    {
        sidecar_paths.push(app_path.join("Contents/Resources/ntp-client-sidecar"));
        sidecar_paths.push(app_path.join("Contents/MacOS/ntp-client-sidecar"));
        sidecar_paths.push(app_path.join("ntp-client-sidecar"));
    }
    
    // 也檢查執行文件所在目錄
    if let Some(exe_dir) = exe_path.parent() {
        sidecar_paths.push(exe_dir.join("ntp-client-sidecar"));
    }

    let sidecar_source = sidecar_paths
        .iter()
        .find(|p| p.exists())
        .ok_or_else(|| {
            format!(
                "找不到 sidecar 二進制文件。已檢查的路徑: {}",
                sidecar_paths
                    .iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    // 建立臨時腳本文件來安裝 sidecar
    let install_script = format!(
        r#"
# 複製 sidecar 二進制文件
cp '{}' '{}'
chmod 755 '{}'
chown root:wheel '{}'

# 建立 LaunchDaemon plist
cat > '{}' << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
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
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/ntp-client-sidecar.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/ntp-client-sidecar.err</string>
</dict>
</plist>
EOF

chmod 644 '{}'
chown root:wheel '{}'

# 先卸載舊的（如果存在）
launchctl unload '{}' 2>/dev/null || true

# 載入 LaunchDaemon（需要 root 權限）
launchctl load -w '{}'

# 等待一下確保服務啟動
sleep 1
"#,
        sidecar_source.to_string_lossy(),      // 1: cp source
        SIDECAR_INSTALL_PATH,                   // 2: cp dest
        SIDECAR_INSTALL_PATH,                   // 3: chmod
        SIDECAR_INSTALL_PATH,                   // 4: chown
        LAUNCHDAEMON_PATH,                      // 5: cat > plist
        LAUNCHDAEMON_LABEL,                     // 6: Label string
        SIDECAR_INSTALL_PATH,                   // 7: ProgramArguments string
        LAUNCHDAEMON_PATH,                      // 8: chmod plist
        LAUNCHDAEMON_PATH,                      // 9: chown plist
        LAUNCHDAEMON_PATH,                      // 10: launchctl unload
        LAUNCHDAEMON_PATH                       // 11: launchctl load
    );

    // 將腳本寫入臨時文件
    let temp_script = "/tmp/install-sidecar.sh";
    std::fs::write(temp_script, install_script)
        .map_err(|e| format!("無法寫入臨時腳本: {}", e))?;

    // 使用 osascript 請求管理員權限執行安裝腳本
    let script = format!(
        r#"do shell script "bash '{}'" with administrator privileges"#,
        temp_script
    );

    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|e| format!("執行失敗: {}", e))?;

    // 清理臨時文件
    let _ = std::fs::remove_file(temp_script);

    if output.status.success() {
        Ok(serde_json::json!({
            "success": true,
            "message": "Sidecar 已成功安裝並啟動"
        })
        .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("User canceled") || stderr.contains("-128") {
            Err("使用者取消授權".to_string())
        } else {
            Err(format!("安裝失敗: {}", stderr))
        }
    }
}

/// 卸載 sidecar server
#[tauri::command]
#[cfg(target_os = "macos")]
pub async fn uninstall_sidecar() -> Result<String, String> {
    let uninstall_script = format!(
        r#"
# 卸載 LaunchDaemon
launchctl unload -w '{}' 2>/dev/null || true
rm -f '{}'

# 刪除 sidecar 二進制文件
rm -f '{}'
"#,
        LAUNCHDAEMON_PATH, LAUNCHDAEMON_PATH, SIDECAR_INSTALL_PATH
    );

    let temp_script = "/tmp/uninstall-sidecar.sh";
    std::fs::write(temp_script, uninstall_script)
        .map_err(|e| format!("無法寫入臨時腳本: {}", e))?;

    let script = format!(
        r#"do shell script "bash '{}'" with administrator privileges"#,
        temp_script
    );

    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|e| format!("執行失敗: {}", e))?;

    let _ = std::fs::remove_file(temp_script);

    if output.status.success() {
        Ok(serde_json::json!({
            "success": true,
            "message": "Sidecar 已成功卸載"
        })
        .to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("User canceled") || stderr.contains("-128") {
            Err("使用者取消授權".to_string())
        } else {
            Err(format!("卸載失敗: {}", stderr))
        }
    }
}

/// 通過 UDP 與 sidecar 通信設定時間
#[cfg(target_os = "macos")]
pub fn set_time_via_sidecar(unix_ms: f64) -> Result<String, String> {
    // 序列化請求
    let request = SetTimeRequest { unix_ms };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| format!("序列化失敗: {}", e))?;

    // 創建 UDP socket
    let socket = UdpSocket::bind("127.0.0.1:0")
        .map_err(|e| format!("無法綁定 UDP socket: {}", e))?;

    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(2)))
        .map_err(|e| format!("無法設定超時: {}", e))?;

    // 發送請求
    socket
        .send_to(request_json.as_bytes(), format!("127.0.0.1:{}", SIDECAR_PORT))
        .map_err(|e| format!("無法發送請求: {}", e))?;

    // 接收回應
    let mut buffer = [0u8; 1024];
    let (size, _) = socket
        .recv_from(&mut buffer)
        .map_err(|e| format!("無法接收回應: {}", e))?;

    let response_json = String::from_utf8_lossy(&buffer[..size]);
    let response: SetTimeResponse = serde_json::from_str(&response_json)
        .map_err(|e| format!("解析回應失敗: {}", e))?;

    if response.success {
        Ok(response.message)
    } else {
        Err(response.error.unwrap_or_else(|| response.message))
    }
}

/// Sidecar server 主程序（以 root 運行）
#[cfg(target_os = "macos")]
pub fn run_sidecar_server() -> Result<(), Box<dyn std::error::Error>> {
    // 檢查是否以 root 運行
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("錯誤: sidecar server 必須以 root 權限運行");
        std::process::exit(1);
    }

    println!("[SIDECAR] 啟動 sidecar server，監聽端口 {}", SIDECAR_PORT);

    // 綁定 UDP socket
    let socket = UdpSocket::bind(format!("127.0.0.1:{}", SIDECAR_PORT))?;
    println!("[SIDECAR] 已綁定到 127.0.0.1:{}", SIDECAR_PORT);

    let mut buffer = [0u8; 1024];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                let request_json = String::from_utf8_lossy(&buffer[..size]);
                println!("[SIDECAR] 收到來自 {} 的請求: {}", addr, request_json);

                match serde_json::from_str::<SetTimeRequest>(&request_json) {
                    Ok(request) => {
                        let response = handle_set_time_request(request);
                        let response_json = serde_json::to_string(&response)
                            .unwrap_or_else(|_| r#"{"success":false,"message":"序列化失敗"}"#.to_string());

                        if let Err(e) = socket.send_to(response_json.as_bytes(), addr) {
                            eprintln!("[SIDECAR] 發送回應失敗: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("[SIDECAR] 解析請求失敗: {}", e);
                        let error_response = SetTimeResponse {
                            success: false,
                            message: "無效的請求格式".to_string(),
                            error: Some(e.to_string()),
                        };
                        let response_json = serde_json::to_string(&error_response)
                            .unwrap_or_else(|_| r#"{"success":false,"message":"序列化失敗"}"#.to_string());
                        let _ = socket.send_to(response_json.as_bytes(), addr);
                    }
                }
            }
            Err(e) => {
                eprintln!("[SIDECAR] 接收數據失敗: {}", e);
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn handle_set_time_request(request: SetTimeRequest) -> SetTimeResponse {
    let secs = (request.unix_ms / 1000.0).floor() as i64;
    let usecs = ((request.unix_ms % 1000.0) * 1000.0) as i64;

    let tv = libc::timeval {
        tv_sec: secs,
        tv_usec: usecs as i32,
    };

    let result = unsafe { libc::settimeofday(&tv, std::ptr::null()) };

    if result == 0 {
        let datetime = chrono::DateTime::from_timestamp(secs, (usecs * 1000) as u32)
            .unwrap_or_else(|| chrono::DateTime::from_timestamp(secs, 0).unwrap());
        SetTimeResponse {
            success: true,
            message: format!(
                "System time set (UTC): {}.{:03}",
                datetime.format("%Y-%m-%d %H:%M:%S"),
                (usecs / 1000)
            ),
            error: None,
        }
    } else {
        let errno = std::io::Error::last_os_error();
        SetTimeResponse {
            success: false,
            message: format!("settimeofday failed: {}", errno),
            error: Some(errno.to_string()),
        }
    }
}

// 非 macOS 平台的空實現
#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn check_sidecar_status() -> Result<String, String> {
    Ok(serde_json::json!({
        "installed": false,
        "running": false,
        "message": "此功能僅支援 macOS"
    })
    .to_string())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn install_sidecar() -> Result<String, String> {
    Err("此功能僅支援 macOS".to_string())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn uninstall_sidecar() -> Result<String, String> {
    Err("此功能僅支援 macOS".to_string())
}

