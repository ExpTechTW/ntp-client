!macro NSIS_HOOK_PREINSTALL
  ; 通知程式自行退出（透過 Windows Event）
  System::Call 'kernel32::OpenEventW(i 0x1F0003, i 0, w "Global\NTPClientExitEvent") p .r0'
  ${If} $0 != 0
    System::Call 'kernel32::SetEvent(p r0)'
    System::Call 'kernel32::CloseHandle(p r0)'
    Sleep 1000
  ${EndIf}
!macroend
