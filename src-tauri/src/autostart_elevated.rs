use tauri::AppHandle;

#[tauri::command]
pub async fn enable_autostart(app: AppHandle) -> Result<String, String> {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    {
        use tauri_plugin_autostart::ManagerExt;
        
        app.autolaunch()
            .enable()
            .map_err(|e| format!("啟用開機自啟動失敗: {}", e))?;

        Ok(serde_json::json!({
            "success": true,
            "message": "已啟用開機自啟動"
        })
        .to_string())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("不支援的作業系統".to_string())
    }
}

#[tauri::command]
pub async fn disable_autostart(app: AppHandle) -> Result<String, String> {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    {
        use tauri_plugin_autostart::ManagerExt;
        
        app.autolaunch()
            .disable()
            .map_err(|e| format!("停用開機自啟動失敗: {}", e))?;

        Ok(serde_json::json!({
            "success": true,
            "message": "已停用開機自啟動"
        })
        .to_string())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("不支援的作業系統".to_string())
    }
}

#[tauri::command]
pub async fn is_autostart_enabled(app: AppHandle) -> Result<String, String> {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    {
        use tauri_plugin_autostart::ManagerExt;
        
        let enabled = app.autolaunch()
            .is_enabled()
            .map_err(|e| format!("查詢開機自啟動狀態失敗: {}", e))?;

        Ok(serde_json::json!({
            "enabled": enabled,
            "message": if enabled { "已啟用" } else { "未啟用" }
        })
        .to_string())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Ok(serde_json::json!({
            "enabled": false,
            "message": "不支援的作業系統"
        })
        .to_string())
    }
}
