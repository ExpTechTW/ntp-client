!macro NSIS_HOOK_PREINSTALL
  nsis_tauri_utils::KillProcessCurrentUser "ntp-client.exe"
  Pop $R0
!macroend
