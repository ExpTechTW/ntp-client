use serde::{Deserialize, Serialize};
use std::net::UdpSocket;

#[cfg(target_os = "macos")]
const SIDECAR_PORT: u16 = 12345;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SetTimeRequest {
    unix_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SetTimeResponse {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
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

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                        } else {
                            println!("[SIDECAR] 已回應: {}", response_json);
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

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("Sidecar server 僅支援 macOS");
    std::process::exit(1);
}
