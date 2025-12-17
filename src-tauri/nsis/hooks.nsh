!macro NSIS_HOOK_PREINSTALL
  ; 通知程式自行退出（透過 Windows Event）
  ; EVENT_MODIFY_STATE = 0x0002
  System::Call 'kernel32::OpenEventW(i 0x0002, i 0, w "Global\NTPClientExitEvent") p.r0'
  IntCmp $0 0 done
    System::Call 'kernel32::SetEvent(p r0)'
    System::Call 'kernel32::CloseHandle(p r0)'
    Sleep 1000
  done:
!macroend
