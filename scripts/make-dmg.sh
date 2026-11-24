#!/usr/bin/env bash
# 构建 macOS 安装包（.app + .dmg）
#
# 为什么需要这个脚本：
# macOS Tahoe (26) 会把 hdiutil 挂载到 /Volumes 的读写镜像变为只读，
# 导致 tauri 默认的 dmg 打包在 `ln -s /Applications` 时报 Permission denied。
# 本脚本改用挂载到 /tmp 的 create-dmg 脚本（scripts/dmg/bundle_dmg.sh）绕过该问题。
set -euo pipefail

# 切到项目根目录
cd "$(dirname "$0")/.."
ROOT="$(pwd)"

# 环境：node + cargo + 代理（如已设置则沿用）
export PATH="$HOME/.nvm/versions/node/v20.20.2/bin:$HOME/.cargo/bin:$PATH"
export https_proxy="${https_proxy:-http://127.0.0.1:7897}"
export http_proxy="${http_proxy:-http://127.0.0.1:7897}"

# 1. 构建 .app（跳过 dmg，避免 /Volumes 问题）
echo "==> 构建 .app"
npm run tauri build -- --bundles app

# 2. 读取产品名、版本号，按机器架构拼出 dmg 文件名
PRODUCT="$(node -p "require('./src-tauri/tauri.conf.json').productName")"
VERSION="$(node -p "require('./src-tauri/tauri.conf.json').version")"
case "$(uname -m)" in
  arm64)  ARCH="aarch64" ;;
  x86_64) ARCH="x64" ;;
  *)      ARCH="$(uname -m)" ;;
esac
DMG_NAME="${PRODUCT}_${VERSION}_${ARCH}.dmg"

BUNDLE_DIR="$ROOT/src-tauri/target/release/bundle/macos"
DMG_SCRIPT="$ROOT/scripts/dmg/bundle_dmg.sh"
ICNS="$ROOT/src-tauri/icons/icon.icns"

# 清理上次的 dmg 产物与残留的读写临时镜像，避免 hdiutil convert 因文件已存在而失败
rm -f "$BUNDLE_DIR/$DMG_NAME" "$BUNDLE_DIR"/rw.*.dmg

# 3. 用打补丁后的 create-dmg 脚本生成 .dmg
echo "==> 构建 .dmg ($DMG_NAME)"
(
  cd "$BUNDLE_DIR"
  bash "$DMG_SCRIPT" \
    --volname "$PRODUCT" \
    --icon "${PRODUCT}.app" 180 170 \
    --app-drop-link 480 170 \
    --window-size 660 400 \
    --hide-extension "${PRODUCT}.app" \
    --volicon "$ICNS" \
    --skip-jenkins \
    "$DMG_NAME" "${PRODUCT}.app"
)

echo ""
echo "==> 完成"
echo "    .app : $BUNDLE_DIR/${PRODUCT}.app"
echo "    .dmg : $BUNDLE_DIR/$DMG_NAME"
