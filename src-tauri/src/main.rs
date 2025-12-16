// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autostart_elevated;
mod core;
mod sidecar;
mod user;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};
use tauri_plugin_updater::UpdaterExt;

#[cfg(target_os = "windows")]
fn ensure_admin() {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};
    use windows_sys::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
    use windows_sys::Win32::UI::Shell::{ShellExecuteExW, SHELLEXECUTEINFOW, SEE_MASK_FLAG_NO_UI, SEE_MASK_NOCLOSEPROCESS};

    let is_admin = unsafe {
        let mut token: HANDLE = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            false
        } else {
            let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
            let mut size: u32 = 0;
            let result = GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            );
            CloseHandle(token);
            result != 0 && elevation.TokenIsElevated != 0
        }
    };

    if !is_admin {
        if let Ok(exe_path) = std::env::current_exe() {
            let exe_path_wide: Vec<u16> = OsStr::new(exe_path.as_os_str())
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let runas_verb: Vec<u16> = "runas\0".encode_utf16().collect();

            let mut sei: SHELLEXECUTEINFOW = unsafe { std::mem::zeroed() };
            sei.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
            sei.fMask = SEE_MASK_FLAG_NO_UI | SEE_MASK_NOCLOSEPROCESS;
            sei.hwnd = std::ptr::null_mut();
            sei.lpVerb = runas_verb.as_ptr();
            sei.lpFile = exe_path_wide.as_ptr();
            sei.lpParameters = std::ptr::null();
            sei.lpDirectory = std::ptr::null();
            sei.nShow = 0;

            let result = unsafe { ShellExecuteExW(&mut sei) };

            if result != 0 {
                if !sei.hProcess.is_null() {
                    unsafe {
                        CloseHandle(sei.hProcess);
                    }
                }
                std::process::exit(0);
            } else {
                let error = unsafe { GetLastError() };
                eprintln!("無法以管理員權限啟動: 錯誤碼 {}", error);
            }
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
    #[cfg(target_os = "windows")]
    ensure_admin();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
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
            // 初始化資料庫
            if let Err(e) = core::db::init_db() {
                eprintln!("[DB] 資料庫初始化失敗: {}", e);
            } else {
                println!("[DB] 資料庫初始化成功");
            }

            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }

            let handle = app.handle().clone();

            let show_i = MenuItem::with_id(app, "show", "顯示視窗", true, None::<&str>)?;
            let sync_i = MenuItem::with_id(app, "sync", "立即同步", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &sync_i, &quit_i])?;

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
                            let _ = core::offset::sync_ntp_time(server).await;
                            println!("[TRAY] 同步完成");
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

            let update_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let _ = update(update_handle.clone()).await;
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(15 * 60)).await;
                    let _ = update(update_handle.clone()).await;
                }
            });

            let sync_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    let server = "time.exptech.com.tw".to_string();
                    match core::offset::sync_ntp_time(server).await {
                        Ok(_) => println!("[BG] 背景同步完成"),
                        Err(e) => println!("[BG] 背景同步失敗: {}", e),
                    }
                    let _ = sync_handle.emit("ntp-synced", ());
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Core - NTP
            core::ntp::query_ntp_udp,
            // Core - Offset
            core::offset::adjust_time_by_offset,
            core::offset::set_system_time_ms,
            core::offset::check_time_permission,
            core::offset::sync_ntp_time,
            // Core - Database
            core::db::db_init,
            core::db::db_insert_record,
            core::db::db_insert_batch,
            core::db::db_query_recent,
            core::db::db_query_range,
            core::db::db_query_by_server,
            core::db::db_query_outliers,
            core::db::db_query,
            core::db::db_get_stats,
            core::db::db_archive,
            core::db::db_query_archived,
            core::db::db_delete_before,
            core::db::db_clear,
            core::db::db_optimize,
            core::db::db_aggregate_hourly,
            core::db::db_aggregate_daily,
            // Autostart
            autostart_elevated::enable_autostart,
            autostart_elevated::disable_autostart,
            autostart_elevated::is_autostart_enabled,
            // Sidecar
            sidecar::check_sidecar_status,
            sidecar::install_sidecar,
            sidecar::uninstall_sidecar,
            // User - Stats
            user::stats::calculate_history_stats,
            user::stats::calculate_autocorr_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
