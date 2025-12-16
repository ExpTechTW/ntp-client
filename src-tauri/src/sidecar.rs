#[cfg(target_os = "macos")]
use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
use std::net::UdpSocket;
#[cfg(target_os = "macos")]
use std::process::Command;

#[cfg(target_os = "macos")]
const SIDECAR_PORT: u16 = 12345;
#[cfg(target_os = "macos")]
const SIDECAR_INSTALL_PATH: &str = "/usr/local/bin/ntp-client-sidecar";
#[cfg(target_os = "macos")]
const LAUNCHDAEMON_LABEL: &str = "com.exptech.ntp-client-sidecar";
#[cfg(target_os = "macos")]
const LAUNCHDAEMON_PATH: &str = "/Library/LaunchDaemons/com.exptech.ntp-client-sidecar.plist";

#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTimeRequest {
    pub unix_ms: f64,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTimeResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[tauri::command]
#[cfg(target_os = "macos")]
pub async fn check_sidecar_status() -> Result<String, String> {
    let installed = std::path::Path::new(SIDECAR_INSTALL_PATH).exists()
        && std::path::Path::new(LAUNCHDAEMON_PATH).exists();

    let running = test_sidecar_connection();

    Ok(serde_json::json!({
        "installed": installed,
        "running": running,
        "message": if installed && running {
            "Sidecar 已安裝並運行中"
        } else if installed {
            "Sidecar 已安裝但未運行"
        } else {
            "Sidecar 未安裝"
        }
    })
    .to_string())
}

#[cfg(target_os = "macos")]
fn test_sidecar_connection() -> bool {
    let socket = match UdpSocket::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(_) => return false,
    };

    if socket
        .set_read_timeout(Some(std::time::Duration::from_millis(500)))
        .is_err()
    {
        return false;
    }

    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0);

    let test_request = format!(r#"{{"unix_ms":{}}}"#, now_ms);
    if socket
        .send_to(
            test_request.as_bytes(),
            format!("127.0.0.1:{}", SIDECAR_PORT),
        )
        .is_err()
    {
        return false;
    }

    let mut buffer = [0u8; 256];
    socket.recv_from(&mut buffer).is_ok()
}

#[tauri::command]
#[cfg(target_os = "macos")]
pub async fn install_sidecar() -> Result<String, String> {
    let exe_path =
        std::env::current_exe().map_err(|e| format!("無法取得執行檔路徑: {}", e))?;

    let mut sidecar_paths = Vec::new();

    if let Some(macos_dir) = exe_path.parent() {
        // Tauri externalBin 會將 sidecar 放在與主執行檔相同的目錄 (Contents/MacOS/)
        sidecar_paths.push(macos_dir.join("ntp-client-sidecar"));
        if let Some(contents_dir) = macos_dir.parent() {
            sidecar_paths.push(contents_dir.join("Resources/ntp-client-sidecar"));
            sidecar_paths.push(contents_dir.join("MacOS/ntp-client-sidecar"));
        }
    }

    if let Ok(current_dir) = std::env::current_dir() {
        sidecar_paths.push(current_dir.join("target/debug/ntp-client-sidecar"));
        sidecar_paths.push(current_dir.join("target/release/ntp-client-sidecar"));
        sidecar_paths.push(current_dir.join("src-tauri/target/debug/ntp-client-sidecar"));
        sidecar_paths.push(current_dir.join("src-tauri/target/release/ntp-client-sidecar"));
    }

    let sidecar_source = sidecar_paths.iter().find(|p| p.exists()).ok_or_else(|| {
        format!(
            "找不到 sidecar 二進制文件。已檢查的路徑:\n{}",
            sidecar_paths
                .iter()
                .map(|p| format!("  - {}", p.to_string_lossy()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    })?;

    println!(
        "[SIDECAR] 找到 sidecar 二進制文件: {}",
        sidecar_source.to_string_lossy()
    );

    let temp_sidecar = "/tmp/ntp-client-sidecar-install";
    std::fs::copy(&sidecar_source, temp_sidecar)
        .map_err(|e| format!("無法複製 sidecar 到臨時目錄: {}", e))?;

    let install_script = format!(
        r#"
set -e

# 先卸載並清理舊的安裝（如果存在）
launchctl unload -w '{plist}' 2>/dev/null || true
rm -f '{plist}' 2>/dev/null || true
rm -f '{bin}' 2>/dev/null || true

# 複製新的 sidecar 二進制文件
cp '{temp}' '{bin}'
chmod 755 '{bin}'
chown root:wheel '{bin}'

# 清理臨時文件
rm -f '{temp}' 2>/dev/null || true

# 建立 LaunchDaemon plist
cat > '{plist}' << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{label}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{bin}</string>
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

chmod 644 '{plist}'
chown root:wheel '{plist}'

# 載入 LaunchDaemon（使用新版 bootstrap 語法，相容舊版 load）
# 先嘗試 bootout 移除舊的（如果存在）
launchctl bootout system '{plist}' 2>/dev/null || true

# 使用 bootstrap 載入到 system domain
if launchctl bootstrap system '{plist}' 2>/dev/null; then
    echo "使用 bootstrap 載入成功"
elif launchctl load -w '{plist}' 2>/dev/null; then
    echo "使用 load 載入成功"
else
    echo "LaunchDaemon 載入失敗" >&2
    exit 1
fi

# 等待服務啟動
sleep 2

# 驗證安裝（使用 print 檢查 system domain）
if launchctl print system/{label} >/dev/null 2>&1; then
    echo "安裝成功"
elif launchctl list '{label}' >/dev/null 2>&1; then
    echo "安裝成功"
else
    echo "警告: 服務可能未正常啟動，請檢查 /tmp/ntp-client-sidecar.err" >&2
fi
"#,
        temp = temp_sidecar,
        bin = SIDECAR_INSTALL_PATH,
        plist = LAUNCHDAEMON_PATH,
        label = LAUNCHDAEMON_LABEL
    );

    let temp_script = "/tmp/install-sidecar.sh";
    std::fs::write(temp_script, install_script)
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

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("User canceled") || stderr.contains("-128") {
            return Err("使用者取消授權".to_string());
        } else {
            return Err(format!("安裝失敗: {}", stderr));
        }
    }

    println!("[SIDECAR] 安裝腳本執行完成，正在驗證服務...");

    std::thread::sleep(std::time::Duration::from_millis(500));

    let mut test_passed = false;
    for attempt in 1..=3 {
        if test_sidecar_connection() {
            test_passed = true;
            println!("[SIDECAR] UDP 連接測試成功 (嘗試 {}/3)", attempt);
            break;
        }
        println!(
            "[SIDECAR] UDP 連接測試失敗，重試中... (嘗試 {}/3)",
            attempt
        );
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    if test_passed {
        Ok(serde_json::json!({
            "success": true,
            "message": "Sidecar 已成功安裝並驗證運行正常",
            "verified": true
        })
        .to_string())
    } else {
        Ok(serde_json::json!({
            "success": true,
            "message": "Sidecar 已安裝，但無法驗證服務狀態。請檢查 /tmp/ntp-client-sidecar.err",
            "verified": false
        })
        .to_string())
    }
}

#[tauri::command]
#[cfg(target_os = "macos")]
pub async fn uninstall_sidecar() -> Result<String, String> {
    let uninstall_script = format!(
        r#"
set -e

launchctl bootout system '{plist}' 2>/dev/null || \
launchctl unload -w '{plist}' 2>/dev/null || true

rm -f '{plist}'

rm -f '{bin}'

echo "卸載完成"
"#,
        plist = LAUNCHDAEMON_PATH,
        bin = SIDECAR_INSTALL_PATH
    );

    let temp_script = "/tmp/uninstall-sidecar.sh";
    std::fs::write(temp_script, &uninstall_script)
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

#[cfg(target_os = "macos")]
pub fn set_time_via_sidecar(unix_ms: f64) -> Result<String, String> {
    let request = SetTimeRequest { unix_ms };
    let request_json =
        serde_json::to_string(&request).map_err(|e| format!("序列化失敗: {}", e))?;

    let socket =
        UdpSocket::bind("127.0.0.1:0").map_err(|e| format!("無法綁定 UDP socket: {}", e))?;

    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(2)))
        .map_err(|e| format!("無法設定超時: {}", e))?;

    socket
        .send_to(
            request_json.as_bytes(),
            format!("127.0.0.1:{}", SIDECAR_PORT),
        )
        .map_err(|e| format!("無法發送請求: {}", e))?;

    let mut buffer = [0u8; 1024];
    let (size, _) = socket
        .recv_from(&mut buffer)
        .map_err(|e| format!("無法接收回應: {}", e))?;

    let response_json = String::from_utf8_lossy(&buffer[..size]);
    let response: SetTimeResponse =
        serde_json::from_str(&response_json).map_err(|e| format!("解析回應失敗: {}", e))?;

    if response.success {
        Ok(response.message)
    } else {
        Err(response.error.unwrap_or(response.message))
    }
}

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
