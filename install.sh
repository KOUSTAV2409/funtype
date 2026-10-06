#!/usr/bin/env bash
# ==============================================================================
# FunType - Universal Unix Installer (Arch / Omarchy / Debian / Fedora / macOS)
# ==============================================================================

set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DEST="${HOME}/.local/bin"
APP_DEST="${HOME}/.local/share/applications"

echo "⚡ Installing FunType..."

# 1. Ensure Cargo is available
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ Error: cargo (Rust) is required to build FunType."
    echo "Please install Rust via https://rustup.rs"
    exit 1
fi

# 2. Build optimized release binary
echo "🔨 Building optimized release binary..."
cd "$ROOT"
cargo build --release

# 3. Create destination directory
mkdir -p "$BIN_DEST"
mkdir -p "$APP_DEST"

# 4. Copy binary and launcher
cp -f "$ROOT/target/release/funtype" "$BIN_DEST/funtype-bin"
cp -f "$ROOT/bin/funtype" "$BIN_DEST/funtype"
chmod +x "$BIN_DEST/funtype" "$BIN_DEST/funtype-bin"

echo "✅ Installed binary to: $BIN_DEST/funtype"

# 5. Install Desktop entry
if [[ -f "$ROOT/funtype.desktop" ]]; then
    sed -e "s|Exec=funtype|Exec=${BIN_DEST}/funtype|g" "$ROOT/funtype.desktop" > "$APP_DEST/funtype.desktop"
    echo "✅ Installed desktop entry to: $APP_DEST/funtype.desktop"
fi

# 6. Omarchy / Hyprland auto-configuration
HYPR_BINDINGS="${HOME}/.config/hypr/bindings.lua"
HYPR_MAIN="${HOME}/.config/hypr/hyprland.lua"

if [[ -f "$HYPR_BINDINGS" ]]; then
    if ! grep -q "FunType" "$HYPR_BINDINGS"; then
        echo "" >> "$HYPR_BINDINGS"
        echo "-- FunType - Instant Stress Burster Typing Game" >> "$HYPR_BINDINGS"
        echo "o.bind(\"SUPER + Y\", \"FunType\", os.getenv(\"HOME\") .. \"/.local/bin/funtype\")" >> "$HYPR_BINDINGS"
        echo "✨ Added shortcut 'SUPER + Y' to ~/.config/hypr/bindings.lua"
    else
        echo "ℹ️  Shortcut binding already present in ~/.config/hypr/bindings.lua"
    fi
fi

if [[ -f "$HYPR_MAIN" ]]; then
    if ! grep -q 'o.window("funtype"' "$HYPR_MAIN"; then
        echo "" >> "$HYPR_MAIN"
        echo '-- Floating window rule for FunType' >> "$HYPR_MAIN"
        echo 'o.window("funtype", { float = true, center = true, size = { 1040, 660 } })' >> "$HYPR_MAIN"
        echo "✨ Added floating window rule to ~/.config/hypr/hyprland.lua"
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
