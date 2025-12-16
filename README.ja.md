<div align="center">

<img src="app-icon.png" width="128" height="128" alt="NTP Client">

# NTP Client

高精度ネットワーク時刻同期ツール

[![License](https://img.shields.io/github/license/ExpTechTW/ntp-client?style=flat-square&color=blue)](LICENSE)
[![Release](https://img.shields.io/github/v/release/ExpTechTW/ntp-client?style=flat-square&color=green)](https://github.com/ExpTechTW/ntp-client/releases)
[![Downloads](https://img.shields.io/github/downloads/ExpTechTW/ntp-client/total?style=flat-square)](https://github.com/ExpTechTW/ntp-client/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/ExpTechTW/ntp-client/ci.yml?style=flat-square&label=CI)](https://github.com/ExpTechTW/ntp-client/actions)

[繁體中文](README.md) | [English](README.en.md) | **日本語**

</div>

## スクリーンショット

<div align="center">
<img src="images/image1.png" width="280" alt="メイン画面-小">
<img src="images/image2.png" width="280" alt="メイン画面-大">
<img src="images/image3.png" width="280" alt="分析ページ">
</div>

## 機能

- **超軽量** - macOS はわずか 8MB、Windows はわずか 3MB と、極めて小さなインストールサイズ
- **精密同期** - 5回の測定から中央値を取るアルゴリズムを使用し、時刻オフセット測定の正確性を確保
- **自動同期** - 60秒ごとに自動的に時刻同期を実行
- **複数サーバー** - ExpTech、Apple、Google、Cloudflare などの NTP サーバーをサポート
- **クロスプラットフォーム** - macOS、Windows、Linux をネイティブサポート
- **自動起動** - 起動時の自動起動をサポート
- **ダーク/ライトテーマ** - ダークモードとライトモードを切り替え可能
- **多言語対応** - 繁体字中国語、英語、日本語をサポート

## ダウンロード・インストール

<div align="center">

[![macOS](https://img.shields.io/badge/macOS-black?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/ExpTechTW/ntp-client/releases/latest)
[![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/ExpTechTW/ntp-client/releases/latest)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://github.com/ExpTechTW/ntp-client/releases/latest)

</div>

### サポートシステム

| オペレーティングシステム | バージョン      | アーキテクチャ          | 備考                           |
| ----------------------- | -------------- | ---------------------- | ------------------------------ |
| **macOS**               | 10.13+         | Intel / Apple Silicon  | Sidecar Server が必要          |
| **Windows**             | 10+            | x64                    | WebView2 が必要                |
| **Linux**               | Ubuntu 22.04+  | x64                    | GLib 2.70+、WebKitGTK 4.1 が必要 |

### macOS インストール

1. `.dmg` ファイルをダウンロードしてインストール
2. アプリケーション内の「Sidecar Server をインストール」ボタンをクリックして時刻同期権限を取得

> Sidecar Server は、システムサービスとして実行される補助プログラムで、管理者権限でシステム時刻を設定するために使用されます。

### Windows インストール

1. `.exe` インストーラーをダウンロードして実行
2. 実行時に、アプリケーションは Windows システム時刻を設定するために管理者権限を要求します

### Linux インストール

```bash
chmod +x ntp-client_*.AppImage
./ntp-client_*.AppImage
```

時刻同期には `pkexec` による権限昇格が必要です。

## 使用方法

### 時刻ステータス

| ステータス   | オフセット | 説明               |
| ----------- | --------- | ------------------ |
| **極めて良好** | < 10ms  | 時刻が非常に正確   |
| **良好**    | < 50ms  | 時刻が正確         |
| **正常**    | < 100ms | 時刻が許容範囲内   |
| **ずれ**    | < 500ms | 明らかなオフセット |
| **異常**    | ≥ 500ms | オフセットが大きすぎる |

### 情報タブ

- **タイムスタンプ** - T1/T2/T3/T4 の4つの時刻ポイント
- **計算** - オフセット、遅延、RTT、処理時間
- **パケット** - NTP パケットの詳細情報（Stratum、Poll、Precision）
- **比較** - 同期前後のオフセット比較

## ライセンス

[AGPL-3.0](LICENSE) - 自由に使用、変更、配布できます。変更後はオープンソースにする必要があります。

## 貢献

1. このプロジェクトを Fork する
2. 機能ブランチを作成する (`git checkout -b feature/amazing`)
3. 変更をコミットする (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュする (`git push origin feature/amazing`)
5. Pull Request を開く

## サポート

- [Issues](https://github.com/ExpTechTW/ntp-client/issues) - 問題を報告
- [Discussions](https://github.com/ExpTechTW/ntp-client/discussions) - 機能提案

---

<div align="center">

**[ExpTech](https://github.com/ExpTechTW)**

</div>

