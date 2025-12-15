// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autostart_elevated;
mod ntp;
mod offset;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};
use tauri_plugin_updater::UpdaterExt;

/// Windows: 檢查是否以管理員權限執行，如果不是則重新啟動並請求 UAC
#[cfg(target_os = "windows")]
fn ensure_admin() {
    use std::process::Command;

    // 檢查是否已經是管理員
    let is_admin = Command::new("net")
        .args(["session"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !is_admin {
        // 取得當前執行檔路徑
        if let Ok(exe_path) = std::env::current_exe() {
            let exe_str = exe_path.to_string_lossy();

            // 使用 PowerShell 以管理員權限重新啟動
            let _ = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-Command",
                    &format!(
                        "Start-Process '{}' -Verb RunAs",
                        exe_str.replace("'", "''")
                    ),
                ])
                .spawn();

            // 退出當前非管理員實例
            std::process::exit(0);
        }
    }
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}

fn main() {
    // Windows: 確保以管理員權限執行
    #[cfg(target_os = "windows")]
    ensure_admin();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // 當已有實例運行時，顯示已存在的視窗
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // macOS: 隱藏 Dock 圖示，只顯示在系統托盤
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let handle = app.handle().clone();

            // 建立托盤選單
            let show_i = MenuItem::with_id(app, "show", "顯示視窗", true, None::<&str>)?;
            let sync_i = MenuItem::with_id(app, "sync", "立即同步", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &sync_i, &quit_i])?;

            // 建立托盤圖示
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "sync" => {
                        let handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let server = "time.exptech.com.tw".to_string();
                            let _ = offset::sync_ntp_time(server).await;
                            println!("[TRAY] 同步完成");
                            // 通知前端更新
                            let _ = handle.emit("ntp-synced", ());
                        });
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // 啟動更新檢查
            tauri::async_runtime::spawn(async move {
                let _ = update(handle).await;
            });

            // 背景同步任務
            let sync_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    let server = "time.exptech.com.tw".to_string();
                    match offset::sync_ntp_time(server).await {
                        Ok(_) => println!("[BG] 背景同步完成"),
                        Err(e) => println!("[BG] 背景同步失敗: {}", e),
                    }
                    let _ = sync_handle.emit("ntp-synced", ());
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            // 視窗關閉時隱藏到托盤而非退出
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            ntp::query_ntp_udp,
            offset::adjust_time_by_offset,
            offset::set_system_time_ms,
            offset::check_time_permission,
            offset::sync_ntp_time,
            autostart_elevated::enable_autostart,
            autostart_elevated::disable_autostart,
            autostart_elevated::is_autostart_enabled
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
