use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtpResult {
    pub success: bool,
    pub server: String,
    pub server_ip: String,
    pub t1: f64,
    pub t2: f64,
    pub t3: f64,
    pub t4: f64,
    pub offset: f64,
    pub delay: f64,
    pub leap: u8,
    pub version: u8,
    pub mode: u8,
    pub stratum: u8,
    pub poll: i8,
    pub precision: i8,
    pub root_delay: f64,
    pub root_dispersion: f64,
    pub ref_id: String,
    pub ref_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtpError {
    pub success: bool,
    pub error: String,
    pub code: String,
}

const NTP_TO_UNIX_EPOCH: u64 = 2208988800;
const NTP_PACKET_SIZE: usize = 48;
const NTP_PORT: u16 = 123;
const NTP_TIMEOUT_SECS: u64 = 5;

fn extract_ntp_timestamp(packet: &[u8], offset: usize) -> (u64, u64) {
    let seconds = u32::from_be_bytes([
        packet[offset],
        packet[offset + 1],
        packet[offset + 2],
        packet[offset + 3],
    ]) as u64;
    let fraction = u32::from_be_bytes([
        packet[offset + 4],
        packet[offset + 5],
        packet[offset + 6],
        packet[offset + 7],
    ]) as u64;
    (seconds, fraction)
}

fn ntp_to_unix_ms(seconds: u64, fraction: u64) -> f64 {
    if seconds == 0 && fraction == 0 {
        return 0.0;
    }
    if seconds < NTP_TO_UNIX_EPOCH {
        return 0.0;
    }
    let unix_seconds = seconds - NTP_TO_UNIX_EPOCH;
    (unix_seconds as f64) * 1000.0 + (fraction as f64) * 1000.0 / 4294967296.0
}

fn duration_to_unix_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn decode_reference_id(ref_id_bytes: [u8; 4], stratum: u8) -> String {
    if stratum == 0 || stratum == 1 {
        ref_id_bytes
            .iter()
            .filter(|&&b| b >= 0x20 && b <= 0x7E)
            .map(|&b| b as char)
            .collect::<String>()
            .trim()
            .to_string()
    } else {
        format!(
            "{}.{}.{}.{}",
            ref_id_bytes[0], ref_id_bytes[1], ref_id_bytes[2], ref_id_bytes[3]
        )
    }
}

pub fn query_ntp(server: &str) -> Result<NtpResult, NtpError> {
    let mut ntp_packet = [0u8; NTP_PACKET_SIZE];
    ntp_packet[0] = 0x1b;

    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| NtpError {
        success: false,
        error: format!("無法綁定 UDP socket: {}", e),
        code: "SOCKET_BIND".to_string(),
    })?;

    socket
        .set_read_timeout(Some(Duration::from_secs(NTP_TIMEOUT_SECS)))
        .map_err(|e| NtpError {
            success: false,
            error: format!("無法設定超時: {}", e),
            code: "SOCKET_TIMEOUT".to_string(),
        })?;

    let server_addr = format!("{}:{}", server, NTP_PORT);

    let t1_system = SystemTime::now();
    socket.send_to(&ntp_packet, &server_addr).map_err(|e| NtpError {
        success: false,
        error: format!("無法發送請求: {}", e),
        code: "SEND_ERROR".to_string(),
    })?;
    let t1 = duration_to_unix_ms(t1_system.duration_since(UNIX_EPOCH).map_err(|e| NtpError {
        success: false,
        error: format!("系統時間錯誤: {}", e),
        code: "TIME_ERROR".to_string(),
    })?);

    let mut response = [0u8; NTP_PACKET_SIZE];
    let (size, peer_addr) = socket.recv_from(&mut response).map_err(|e| NtpError {
        success: false,
        error: format!("無法接收回應: {}", e),
        code: "RECV_ERROR".to_string(),
    })?;

    let t4 = duration_to_unix_ms(SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| NtpError {
        success: false,
        error: format!("系統時間錯誤: {}", e),
        code: "TIME_ERROR".to_string(),
    })?);

    if size < NTP_PACKET_SIZE {
        return Err(NtpError {
            success: false,
            error: format!("回應不完整: {} bytes", size),
            code: "INCOMPLETE".to_string(),
        });
    }

    let li_vn_mode = response[0];
    let leap = (li_vn_mode >> 6) & 0x03;
    let version = (li_vn_mode >> 3) & 0x07;
    let mode = li_vn_mode & 0x07;
    let stratum = response[1];
    let poll = response[2] as i8;
    let precision = response[3] as i8;

    let root_delay_raw = u32::from_be_bytes([response[4], response[5], response[6], response[7]]);
    let root_dispersion_raw =
        u32::from_be_bytes([response[8], response[9], response[10], response[11]]);
    let root_delay = (root_delay_raw as f64 / 65536.0) * 1000.0;
    let root_dispersion = (root_dispersion_raw as f64 / 65536.0) * 1000.0;

    let ref_id_bytes = [response[12], response[13], response[14], response[15]];
    let ref_id = decode_reference_id(ref_id_bytes, stratum);

    let (ref_sec, ref_frac) = extract_ntp_timestamp(&response, 16);
    let ref_time = ntp_to_unix_ms(ref_sec, ref_frac);

    let (t2_sec, t2_frac) = extract_ntp_timestamp(&response, 32);
    let t2 = ntp_to_unix_ms(t2_sec, t2_frac);

    let (t3_sec, t3_frac) = extract_ntp_timestamp(&response, 40);
    let t3 = ntp_to_unix_ms(t3_sec, t3_frac);

    let offset = ((t2 - t1) + (t3 - t4)) / 2.0;
    let delay = (t4 - t1) - (t3 - t2);

    Ok(NtpResult {
        success: true,
        server: server.to_string(),
        server_ip: peer_addr.ip().to_string(),
        t1,
        t2,
        t3,
        t4,
        offset,
        delay,
        leap,
        version,
        mode,
        stratum,
        poll,
        precision,
        root_delay,
        root_dispersion,
        ref_id,
        ref_time,
    })
}

#[tauri::command]
pub async fn query_ntp_udp(server: String) -> Result<String, String> {
    println!("[NTP] 查詢 {}", server);

    match query_ntp(&server) {
        Ok(result) => {
            println!(
                "[NTP] ✓ {} | offset={}ms delay={}ms stratum={}",
                result.server_ip, result.offset, result.delay, result.stratum
            );
            serde_json::to_string(&result).map_err(|e| e.to_string())
        }
        Err(error) => {
            println!("[NTP] ✗ {} ({})", error.error, error.code);
            serde_json::to_string(&error).map_err(|e| e.to_string())
        }
    }
}
