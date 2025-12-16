!macro NSIS_HOOK_PREINSTALL
  nsExec::Exec 'taskkill /F /IM "ntp-client.exe"'
  Sleep 500
!macroend
