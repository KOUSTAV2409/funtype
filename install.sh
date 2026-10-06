#!/usr/bin/env bash
# ==============================================================================
# FunType - Universal Unix Installer (Arch / Omarchy / Debian / Fedora / macOS)
# ==============================================================================

set -euo pipefail

echo "⚡ Installing FunType..."

# 1. Ensure Cargo is available
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ Error: cargo (Rust) is required to build FunType."
    echo "Please install Rust via https://rustup.rs"
    exit 1
fi

BIN_DEST="${HOME}/.local/bin"
APP_DEST="${HOME}/.local/share/applications"

# 2. Determine source directory (support both local execution and curl | bash)
SCRIPT_SOURCE="${BASH_SOURCE[0]:-}"
CLEANUP_TMP=false

if [[ -n "$SCRIPT_SOURCE" && -f "$(dirname "$SCRIPT_SOURCE")/Cargo.toml" ]]; then
    ROOT="$(cd "$(dirname "$SCRIPT_SOURCE")" && pwd)"
else
    if ! command -v git >/dev/null 2>&1; then
        echo "❌ Error: git is required to download source when piping from curl."
        exit 1
    fi
    echo "📦 Fetching latest FunType source..."
    ROOT="$(mktemp -d -t funtype-install.XXXXXX)"
    CLEANUP_TMP=true
    trap 'if [[ "$CLEANUP_TMP" == true && -d "$ROOT" ]]; then rm -rf "$ROOT"; fi' EXIT INT TERM
    git clone --depth 1 https://github.com/KOUSTAV2409/funtype.git "$ROOT" >/dev/null 2>&1
fi

# 3. Build optimized release binary
echo "🔨 Building optimized release binary..."
(cd "$ROOT" && cargo build --release)

# 4. Create destination directory
mkdir -p "$BIN_DEST"
mkdir -p "$APP_DEST"

# 5. Copy binary and launcher
cp -f "$ROOT/target/release/funtype" "$BIN_DEST/funtype-bin"
cp -f "$ROOT/bin/funtype" "$BIN_DEST/funtype"
chmod +x "$BIN_DEST/funtype" "$BIN_DEST/funtype-bin"

echo "✅ Installed binary to: $BIN_DEST/funtype"

# 6. Install Desktop entry
if [[ -f "$ROOT/funtype.desktop" ]]; then
    sed -e "s|Exec=funtype|Exec=${BIN_DEST}/funtype|g" "$ROOT/funtype.desktop" > "$APP_DEST/funtype.desktop"
    echo "✅ Installed desktop entry to: $APP_DEST/funtype.desktop"
fi

# 7. Omarchy / Hyprland auto-configuration with safe backups
HYPR_BINDINGS="${HOME}/.config/hypr/bindings.lua"
HYPR_MAIN="${HOME}/.config/hypr/hyprland.lua"

if [[ -f "$HYPR_BINDINGS" ]]; then
    if ! grep -q "FunType" "$HYPR_BINDINGS"; then
        cp -n "$HYPR_BINDINGS" "${HYPR_BINDINGS}.bak" 2>/dev/null || true
        echo "" >> "$HYPR_BINDINGS"
        echo "-- FunType - Instant Stress Burster Typing Game" >> "$HYPR_BINDINGS"
        echo "o.bind(\"SUPER + Y\", \"FunType\", os.getenv(\"HOME\") .. \"/.local/bin/funtype\")" >> "$HYPR_BINDINGS"
        echo "✨ Added shortcut 'SUPER + Y' to ~/.config/hypr/bindings.lua (backup created)"
    else
        echo "ℹ️  Shortcut binding already present in ~/.config/hypr/bindings.lua"
    fi
fi

if [[ -f "$HYPR_MAIN" ]]; then
    if ! grep -q 'o.window("funtype"' "$HYPR_MAIN"; then
        cp -n "$HYPR_MAIN" "${HYPR_MAIN}.bak" 2>/dev/null || true
        echo "" >> "$HYPR_MAIN"
        echo '-- Floating window rule for FunType' >> "$HYPR_MAIN"
        echo 'o.window("funtype", { float = true, center = true, size = { 1040, 660 } })' >> "$HYPR_MAIN"
        echo "✨ Added floating window rule to ~/.config/hypr/hyprland.lua (backup created)"
    else
        echo "ℹ️  Window rule already present in ~/.config/hypr/hyprland.lua"
    fi
fi

echo ""
echo "🎉 FunType successfully installed!"
echo "👉 Launch command: funtype"
if [[ -f "$HYPR_BINDINGS" ]]; then
    echo "👉 Shortcut: Press SUPER + Y anywhere to summon FunType in milliseconds!"
fi
echo "👉 Controls:"
echo "   - [Esc] Instant Exit / Hide"
echo "   - [1] Stress Shredder (Arcade Mode)"
echo "   - [2] Zen Flow (Mindful Breathing Mode)"
echo "   - [3] Speed Sprint (30s Sprint Mode)"
echo "   - [Tab] Switch ASMR Mechanical Switch Audio"
echo "   - [F2] Cycle Themes (Catppuccin, Tokyo Night, Nord, Rosé Pine, Cyberpunk, Sunset)"
