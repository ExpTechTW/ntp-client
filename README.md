# NTP Client

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://opensource.org/licenses/AGPL-3.0)
[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)](https://github.com/yourusername/ntp-client/releases)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey.svg)](https://github.com/yourusername/ntp-client/releases)

<p align="center">
  <strong>現代化 NTP 時間同步客戶端</strong>
</p>

<p align="center">
  <a href="README.md">繁體中文</a> | <a href="README.en.md">English</a> | <a href="README.ja.md">日本語</a>
</p>

---

NTP Client 是一款功能強大的網路時間協議客戶端工具，幫助您連接到 NTP 伺服器，查詢準確的時間並同步系統時鐘。透過直觀的介面，輕鬆檢查時間偏差並保持系統時間的準確性。

## ✨ 功能特色

- 🕐 **NTP 時間查詢**：連接到 NTP 伺服器獲取準確時間
- ⏱️ **時間偏差顯示**：顯示本地時間與 NTP 時間的偏差
- 🌐 **多伺服器支援**：支援多個常用 NTP 伺服器
- 📊 **即時時間顯示**：顯示本地時間和 UTC 時間
- 🌍 **多語言支援**：支援英文、繁體中文和日文
- 🌓 **深色模式**：美觀的深色/淺色主題切換
- 💻 **跨平台**：可在 macOS、Windows 和 Linux 上運行

## 📥 下載與安裝

### 快速下載

<div align="center">

[![Download for macOS](https://img.shields.io/badge/Download-macOS-black?style=for-the-badge&logo=apple)](https://github.com/yourusername/ntp-client/releases/latest/download/ntp-client_macos.tar.gz)
[![Download for Windows](https://img.shields.io/badge/Download-Windows-blue?style=for-the-badge&logo=windows)](https://github.com/yourusername/ntp-client/releases/latest/download/ntp-client_windows.exe)
[![Download for Linux](https://img.shields.io/badge/Download-Linux-orange?style=for-the-badge&logo=linux)](https://github.com/yourusername/ntp-client/releases/latest/download/ntp-client_linux.AppImage)

</div>

### 安裝步驟

<details>
<summary>🍎 macOS - 查看安裝步驟</summary>

<br>

1. **下載檔案** - 點擊上方按鈕下載 `.tar.gz` 檔案
2. **解壓縮** - 雙擊下載的檔案，macOS 會自動解壓縮
3. **安裝** - 將應用程式拖拽到 Applications 資料夾
4. **啟動** - 在 Applications 中雙擊應用程式圖示啟動

> **💡 提示**
> 如果出現「無法驗證開發者」的警告，請在「系統偏好設定」→「安全性與隱私」中允許執行。

</details>

<details>
<summary>🪟 Windows - 查看安裝步驟</summary>

<br>

1. **下載檔案** - 點擊上方按鈕下載 `.exe` 安裝程式
2. **執行安裝** - 雙擊下載的安裝程式
3. **安裝精靈** - 按照安裝精靈的指示完成安裝
4. **啟動** - 從開始選單或桌面捷徑啟動應用程式

> **💡 提示**
> 如果 Windows Defender 顯示警告，請選擇「更多資訊」→「仍要執行」。

</details>

<details>
<summary>🐧 Linux - 查看安裝步驟</summary>

<br>

1. **下載檔案** - 點擊上方按鈕下載 `.AppImage` 檔案
2. **設定權限** - 開啟終端機執行：
   ```bash
   chmod +x ntp-client_*.AppImage
   ```
3. **執行** - 直接雙擊執行或在終端機執行：
   ```bash
   ./ntp-client_*.AppImage
   ```

> **💡 提示**
> AppImage 是可攜式應用程式，無需安裝即可執行。

</details>

## 🚀 使用方法

1. **啟動應用程式**
2. **輸入 NTP 伺服器地址**（預設為 `pool.ntp.org`）
3. **點擊「查詢」按鈕**獲取 NTP 時間
4. **查看結果**：
   - NTP 伺服器時間
   - 本地時間
   - 時間偏差（毫秒）
   - 網路延遲（毫秒）

### 常用 NTP 伺服器

- `pool.ntp.org` - NTP Pool Project（推薦）
- `time.google.com` - Google Public NTP
- `time.cloudflare.com` - Cloudflare NTP
- `time.windows.com` - Microsoft NTP

## 📋 系統需求

### 🍎 macOS
- macOS 10.15 或更高版本
- Intel (x86_64) / Apple Silicon (ARM64)

### 🪟 Windows
- Windows 10 或更高版本
- x64 (64-bit)
- WebView2 執行環境

### 🐧 Linux
- Ubuntu 20.04+ / Debian 11+ / Fedora 35+
- x64 (64-bit)
- GLib >= 2.70, WebKitGTK 4.1

## 📄 授權條款

本專案為開源專案，採用 [AGPL-3.0](License) 授權條款。

## 🤝 貢獻

歡迎貢獻！如果您想為此專案做出貢獻，請隨時提交 Pull Request 或開啟 Issue。

---

<p align="center">
  如果這個專案對您有幫助，請給我們一個 ⭐️ Star！
</p>
