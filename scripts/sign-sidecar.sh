#!/bin/bash

# macOS sidecar 簽名腳本
# 在 tauri build 後執行，對 sidecar 進行 ad-hoc 簽名

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# 檢查是否為 macOS
if [[ "$(uname)" != "Darwin" ]]; then
    echo "[sign-sidecar] 跳過：不是 macOS 系統"
    exit 0
fi

echo "[sign-sidecar] 開始簽名 sidecar..."

# 尋找所有可能的 sidecar 位置
SIDECAR_LOCATIONS=(
    # Release 構建目錄
    "$PROJECT_ROOT/src-tauri/target/release/ntp-client-sidecar"
    "$PROJECT_ROOT/target/release/ntp-client-sidecar"
    # Debug 構建目錄
    "$PROJECT_ROOT/src-tauri/target/debug/ntp-client-sidecar"
    "$PROJECT_ROOT/target/debug/ntp-client-sidecar"
    # .app bundle 內
    "$PROJECT_ROOT/src-tauri/target/release/bundle/macos/NTP Client.app/Contents/Resources/ntp-client-sidecar"
    "$PROJECT_ROOT/src-tauri/target/release/bundle/macos/NTP Client.app/Contents/MacOS/ntp-client-sidecar"
)

SIGNED_COUNT=0

for SIDECAR_PATH in "${SIDECAR_LOCATIONS[@]}"; do
    if [[ -f "$SIDECAR_PATH" ]]; then
        echo "[sign-sidecar] 找到 sidecar: $SIDECAR_PATH"

        # 移除擴展屬性
        xattr -cr "$SIDECAR_PATH" 2>/dev/null || true

        # 移除現有簽名（如果有）
        codesign --remove-signature "$SIDECAR_PATH" 2>/dev/null || true

        # 進行 ad-hoc 簽名
        # 優先嘗試帶 hardened runtime 的簽名
        if codesign --force --sign - --timestamp=none --options runtime "$SIDECAR_PATH" 2>/dev/null; then
            echo "[sign-sidecar] 已使用 hardened runtime 簽名: $SIDECAR_PATH"
        elif codesign --force --sign - "$SIDECAR_PATH" 2>/dev/null; then
            echo "[sign-sidecar] 已使用基本簽名: $SIDECAR_PATH"
        else
            echo "[sign-sidecar] 警告：無法簽名 $SIDECAR_PATH"
            continue
        fi

        # 驗證簽名
        if codesign --verify "$SIDECAR_PATH" 2>/dev/null; then
            echo "[sign-sidecar] 簽名驗證成功: $SIDECAR_PATH"
            ((SIGNED_COUNT++))
        else
            echo "[sign-sidecar] 警告：簽名驗證失敗: $SIDECAR_PATH"
        fi
    fi
done

if [[ $SIGNED_COUNT -gt 0 ]]; then
    echo "[sign-sidecar] 完成！已簽名 $SIGNED_COUNT 個 sidecar"
else
    echo "[sign-sidecar] 警告：未找到任何 sidecar 二進制文件"
fi
