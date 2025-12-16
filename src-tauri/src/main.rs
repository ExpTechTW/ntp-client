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
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
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
