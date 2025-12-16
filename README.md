# NTP Client

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://opensource.org/licenses/AGPL-3.0)
[![Version](https://img.shields.io/badge/version-1.0.0--beta.1-green.svg)](https://github.com/ExpTechTW/ntp-client/releases)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey.svg)](https://github.com/ExpTechTW/ntp-client/releases)

<p align="center">
  <strong>現代化 NTP 時間同步客戶端</strong>
</p>

<p align="center">
  <a href="README.md">繁體中文</a> | <a href="README.en.md">English</a> | <a href="README.ja.md">日本語</a>
</p>

---

NTP Client 是一款功能強大的網路時間協議客戶端工具，幫助您連接到 NTP 伺服器，查詢準確的時間並同步系統時鐘。透過直觀的介面，輕鬆檢查時間偏差並保持系統時間的準確性。

## 功能特色

- **NTP 時間同步**：自動連接 NTP 伺服器並同步系統時間
- **5 次測量中位數**：使用 5 次 NTP 測量取中位數，提高準確性
- **校正前後對比**：顯示同步前誤差與同步後驗證結果
- **60 秒自動同步**：每 60 秒自動進行一次時間同步
- **多伺服器支援**：支援 ExpTech、Apple、Google、Cloudflare 等 NTP 伺服器
- **詳細資訊顯示**：時間戳、計算公式、NTP 封包資訊、校正對比四個分頁
- **多語言支援**：支援繁體中文、英文和日文
- **深淺色主題**：可切換深色/淺色模式
- **跨平台**：支援 macOS、Windows 和 Linux

## 下載與安裝

### 快速下載

<div align="center">

[![Download for macOS](https://img.shields.io/badge/Download-macOS-black?style=for-the-badge&logo=apple)](https://github.com/ExpTechTW/ntp-client/releases/latest)
[![Download for Windows](https://img.shields.io/badge/Download-Windows-blue?style=for-the-badge&logo=windows)](https://github.com/ExpTechTW/ntp-client/releases/latest)
[![Download for Linux](https://img.shields.io/badge/Download-Linux-orange?style=for-the-badge&logo=linux)](https://github.com/ExpTechTW/ntp-client/releases/latest)

</div>

### 安裝步驟

<details>
<summary>macOS</summary>

1. 下載 `.app` 檔案
2. 將應用程式拖拽到 Applications 資料夾
3. 首次執行需要在「系統設定」→「隱私權與安全性」中允許執行
4. 同步時間需要管理員權限

> **注意**：macOS 10.13 或更高版本

</details>

<details>
<summary>Windows</summary>

1. 下載 `.exe` 安裝程式
2. 執行安裝程式，按照指示完成安裝
3. 同步時間需要以管理員身份執行

> **注意**：Windows 10 或更高版本，需要 WebView2 執行環境

</details>

<details>
<summary>Linux</summary>

1. 下載 `.AppImage` 檔案
2. 設定執行權限：
   ```bash
   chmod +x ntp-client_*.AppImage
   ```
3. 執行應用程式
4. 同步時間需要 `pkexec` 權限

> **注意**：需要 GLib >= 2.70, WebKitGTK 4.1

</details>

## 使用方法

1. 啟動應用程式
2. 從下拉選單選擇 NTP 伺服器（預設為 `time.exptech.com.tw`）
3. 應用程式會自動進行時間同步
4. 查看結果：
   - **時間戳**：T1/T2/T3/T4 時間點
   - **計算**：Offset、Delay、RTT、處理時間
   - **封包**：LI/VN/Mode、Stratum、Poll、Precision 等 NTP 封包資訊
   - **對比**：校正前誤差、校正後誤差、校正量、網路延遲

### 支援的 NTP 伺服器

| 伺服器 | 地址 |
|--------|------|
| ExpTech | `time.exptech.com.tw` |
| Apple | `time.apple.com` |
| Google | `time.google.com` |
| Cloudflare | `time.cloudflare.com` |

## 技術架構

- **前端**：Next.js 15 + React 19 + Tailwind CSS 4
- **後端**：Tauri 2 + Rust
- **NTP 協議**：原生 UDP 實作，支援 NTPv4
- **國際化**：i18next

## 開發

### 環境需求

- [Bun](https://bun.sh/) (推薦) 或 Node.js
- [Rust](https://www.rust-lang.org/)
- [Tauri CLI](https://tauri.app/)

### 安裝依賴

```bash
bun install
```

### 開發模式

```bash
bun run tauri:dev
```

### 建置

```bash
bun run tauri:build
```

## 授權條款

本專案採用 [AGPL-3.0](License) 授權條款。

## 貢獻

歡迎貢獻！如果您想為此專案做出貢獻，請提交 Pull Request 或開啟 Issue。

---

<p align="center">
  由 <a href="https://github.com/ExpTechTW">ExpTech</a> 開發維護
</p>
