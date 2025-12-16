<div align="center">

<img src="app-icon.png" width="128" height="128" alt="NTP Client">

# NTP Client

高精度網路時間同步工具

[![License](https://img.shields.io/github/license/ExpTechTW/ntp-client?style=flat-square&color=blue)](LICENSE)
[![Release](https://img.shields.io/github/v/release/ExpTechTW/ntp-client?style=flat-square&color=green)](https://github.com/ExpTechTW/ntp-client/releases)
[![Downloads](https://img.shields.io/github/downloads/ExpTechTW/ntp-client/total?style=flat-square)](https://github.com/ExpTechTW/ntp-client/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/ExpTechTW/ntp-client/ci.yml?style=flat-square&label=CI)](https://github.com/ExpTechTW/ntp-client/actions)

**繁體中文** | [English](README.en.md) | [日本語](README.ja.md)

</div>

## 功能特色

- **精準同步** - 採用 5 次測量取中位數演算法，確保時間偏移量測量的準確性
- **自動同步** - 每 60 秒自動進行一次時間同步
- **多伺服器** - 支援 ExpTech、Apple、Google、Cloudflare 等 NTP 伺服器
- **跨平台** - 原生支援 macOS、Windows、Linux
- **深淺主題** - 可切換深色/淺色模式
- **多語言** - 支援繁體中文、English、日本語

## 下載安裝

<div align="center">

[![macOS](https://img.shields.io/badge/macOS-black?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/ExpTechTW/ntp-client/releases/latest)
[![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/ExpTechTW/ntp-client/releases/latest)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://github.com/ExpTechTW/ntp-client/releases/latest)

</div>

### 支援系統

| 作業系統 | 版本 | 架構 | 備註 |
|---------|------|------|------|
| **macOS** | 10.13+ | Intel / Apple Silicon | 需安裝 Sidecar Server |
| **Windows** | 10+ | x64 | 需要 WebView2 |
| **Linux** | Ubuntu 22.04+ | x64 | 需要 GLib 2.70+, WebKitGTK 4.1 |

### macOS 安裝

1. 下載 `.dmg` 檔案並安裝
2. 首次開啟時，前往「系統設定」→「隱私權與安全性」→「仍要打開」
3. 點擊應用程式內的「安裝 Sidecar Server」按鈕以取得時間同步權限

> Sidecar Server 是一個以系統服務形式運行的輔助程式，用於以管理員權限設定系統時間。

### Windows 安裝

1. 下載並執行 `.exe` 安裝程式
2. 以**系統管理員身分**執行應用程式以使用時間同步功能

### Linux 安裝

```bash
chmod +x ntp-client_*.AppImage
./ntp-client_*.AppImage
```

時間同步需要 `pkexec` 提升權限。

## 使用說明

### 時間狀態

| 狀態 | 偏移量 | 說明 |
|------|--------|------|
| **極佳** | < 10ms | 時間高度準確 |
| **良好** | < 50ms | 時間準確 |
| **正常** | < 100ms | 時間可接受 |
| **偏差** | < 500ms | 有明顯偏移 |
| **異常** | ≥ 500ms | 偏移量過大 |

### 資訊分頁

- **時間戳** - T1/T2/T3/T4 四個時間點
- **計算** - Offset、Delay、RTT、處理時間
- **封包** - NTP 封包詳細資訊（Stratum、Poll、Precision）
- **對比** - 同步前後偏移量比較

## 開發

### 環境需求

- [Bun](https://bun.sh/) 或 Node.js 18+
- [Rust](https://www.rust-lang.org/) 1.70+

### 快速開始

```bash
# 安裝依賴
bun install

# 開發模式
bun tauri dev

# 建置
bun tauri build
```

### 專案結構

```
ntp-client/
├── src/                    # 前端 (Next.js + React)
├── src-tauri/              # 後端 (Rust + Tauri)
│   ├── src/
│   │   ├── main.rs         # 應用程式進入點
│   │   ├── ntp.rs          # NTP 協議實作
│   │   ├── offset.rs       # 時間偏移計算
│   │   └── sidecar.rs      # macOS Sidecar 服務
│   └── sidecar/            # Sidecar 執行檔 (macOS)
└── public/locales/         # 多語言翻譯檔
```

### 技術棧

| 層級 | 技術 |
|------|------|
| 前端 | Next.js 15, React 19, Tailwind CSS 4 |
| 後端 | Tauri 2, Rust |
| 協議 | NTPv4 (原生 UDP) |
| 國際化 | i18next |

## 授權條款

[AGPL-3.0](LICENSE) - 可自由使用、修改和發布，修改後必須開源。

## 貢獻

1. Fork 本專案
2. 建立功能分支 (`git checkout -b feature/amazing`)
3. 提交變更 (`git commit -m 'Add amazing feature'`)
4. 推送分支 (`git push origin feature/amazing`)
5. 開啟 Pull Request

## 支援

- [Issues](https://github.com/ExpTechTW/ntp-client/issues) - 問題回報
- [Discussions](https://github.com/ExpTechTW/ntp-client/discussions) - 功能建議

---

<div align="center">

**[ExpTech](https://github.com/ExpTechTW)**

</div>
