# ⚡ FunType

> **Instant-launch, hardware-accelerated stress-burster typing game for developers & computer workers.**  
> Crafted in Rust for Unix systems (Linux & macOS). Summoned in milliseconds via a global shortcut key.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org)
[![Startup Time](https://img.shields.io/badge/Startup-%3C%2025ms-brightgreen.svg)]()
[![Binary Size](https://img.shields.io/badge/Binary%20Size-1.6%20MB-purple.svg)]()

---

## 🌟 Why FunType?

Working long hours at a computer leads to cognitive overload, screen fatigue, and tension. Traditional typing tests (like Monkeytype or TypeRacer) often induce performance anxiety with aggressive error highlighting and timers.

**FunType is engineered specifically as a therapeutic stress-burster**:
- 🚀 **Sub-25ms Launch**: Native GPU-accelerated window (1.6 MB standalone binary) that pops up immediately when summoned.
- ⌨️ **Global Shortcut**: Press `SUPER + ALT + F` anywhere over your code editor, terminal, or browser to start playing instantly. Press `Esc` anytime to hide.
- 🎧 **Procedural ASMR Mechanical Audio**: Zero external audio assets—synthesizes rich tactile mechanical switch clicks and deep "thocks" in memory at runtime.
- 💥 **Explosive Particle Catharsis**: Shred work-stress triggers into glowing neon particle fireworks with screen micro-shakes.
- 🌿 **Mindful Breathing Guide**: A therapeutic 12-second visual breathing orb (4s Inhale / 2s Hold / 4s Exhale / 2s Rest) paired with philosophical reflections to actively down-regulate cortisol and eye strain.
- 🎨 **State-of-the-Art Developer Palettes**: Catppuccin Mocha, Tokyo Night, Nord Serenity, Rosé Pine, Cyberpunk, and Sunset Zen.

---

## 🎮 Game Modes

### 1. 💥 Stress Shredder (Arcade Mode — Key `1`)
Workplace stressors descend gracefully:
- *Code*: `merge conflict`, `prod incident`, `segfault`, `null pointer`, `git push --force`, `memory leak`, `infinite loop`, `404 not found`
- *Workplace*: `emergency call`, `unresolved blocker`, `endless meeting`, `jira sprint`, `unpaid overtime`, `scope creep`

Target and type them to trigger **explosive particle fireworks**, combo streaks, and screen-shake feedback. Watch your **Workplace Stress Meter** drop from 100% down to **0% Zen Nirvana**. Never punishes you with game-over screens.

### 2. 🌿 Zen Flow (Mindful Breathing Mode — Key `2`)
- Combines smooth typing of philosophical and nature reflections with a gentle glowing **Breathing Orb**.
- Synchronizes typing cadence with deep breathing.
- Smooth interpolated caret with ambient aura. Mistakes are softly highlighted without harsh buzzer tones.

### 3. 🚀 Speed Sprint (High-Velocity Sprint — Key `3`)
- Fast 30-second rhythm flow.
- Real-time WPM, accuracy, and combo streak counter with flame particle trails for streaks above 15.
- Results card calculating **Stress Relieved Points** earned.

---

## 🎧 ASMR Mechanical Switch Audio Profiles

Toggle anytime with `[Tab]`:
1. **Thocky (Holy Panda)**: Resonant low-mid frequency body with crisp tactile snap.
2. **Clicky (Blue)**: Crisp, high-frequency mechanical click.
3. **Creamy Linear**: Smooth, lubricated linear switch pop.
4. **Bubble Pop**: Calming water droplet / bubble harmonic frequency sweep.
5. **Mute (Silent)**: For quiet office environments.

---

## 🎨 Aesthetic Themes

Cycle anytime with `[F2]`:
- **Catppuccin Mocha** (Default)
- **Tokyo Night**
- **Nord Serenity**
- **Rosé Pine**
- **Cyberpunk Matrix**
- **Sunset Zen**

---

## ⌨️ Controls & Shortcuts

| Key | Action |
| --- | --- |
| `SUPER + ALT + F` | **Summon / Launch FunType anywhere** |
| `Esc` | **Instant Exit / Dismiss** |
| `1` | Switch to **Stress Shredder** |
| `2` | Switch to **Zen Flow** |
| `3` | Switch to **Speed Sprint** |
| `Tab` | Cycle **ASMR Switch Sounds** |
| `F2` | Cycle **Themes** |
| `R` | Reset / Restart Active Mode |
| `Backspace` | Erase Character (Zen Flow & Sprint) |

---

## 🚀 Quick Install (Unix / Linux / macOS)

### One-line Installation:
```bash
git clone https://github.com/iamkxyz/funtype.git
cd funtype
./install.sh
```

The script will:
1. Build the release binary via `cargo build --release`.
2. Install the binary to `~/.local/bin/funtype`.
3. Register the `.desktop` launcher in `~/.local/share/applications/`.
4. Automatically configure **Hyprland / Omarchy** floating window rules and shortcut (`SUPER + ALT + F`).

---

## 🖥️ Platform Configuration Guide

### Omarchy / Hyprland (Arch Linux)
The installer automatically adds the following to your config:
- In `~/.config/hypr/bindings.lua`:
  ```lua
  o.bind("SUPER + ALT + F", "FunType", os.getenv("HOME") .. "/.local/bin/funtype")
  ```
- In `~/.config/hypr/hyprland.lua`:
  ```lua
  o.window("funtype", { float = true, center = true, size = { 1040, 660 } })
  ```

### Standard Hyprland (`hyprland.conf`)
```ini
bind = SUPER ALT, F, exec, ~/.local/bin/funtype
windowrulev2 = float, class:^(funtype)$
windowrulev2 = size 1040 660, class:^(funtype)$
windowrulev2 = center, class:^(funtype)$
```

### i3 / Sway
```ini
bindsym $mod+Mod1+f exec ~/.local/bin/funtype
for_window [class="funtype"] floating enable, resize set 1040 660, move position center
```

### macOS
1. Add `~/.local/bin` to your `$PATH`.
2. To trigger via shortcut:
   - Use **Raycast** (Create Quicklink or Script Command `~/.local/bin/funtype`).
   - Or use **skhd** (`~/.config/skhd/skhdrc`):
     ```bash
     cmd + alt - f : ~/.local/bin/funtype
     ```
   - Or create a macOS Shortcut in the Shortcuts app with keybind `Cmd+Option+F`.

---

## 🛠️ Architecture & Tech Stack

```
funtype/
├── bin/
│   └── funtype            # Sub-millisecond launcher & toggle script
├── install.sh             # Universal Unix installer
├── funtype.desktop        # Linux desktop entry
├── Cargo.toml             # Rust manifest
└── src/
    ├── main.rs            # Event loop, window config & dispatcher
    ├── audio.rs           # Procedural 44.1kHz PCM mechanical switch synthesizer
    ├── theme.rs           # 6 modern developer themes
    ├── particles.rs       # 60 FPS particle physics & screen-shake engine
    ├── modes/
    │   ├── mod.rs
    │   ├── stress_shredder.rs  # Work stressor targeting & destruction
    │   ├── zen_flow.rs         # Mindful typing with 12s breathing orb
    │   └── speed_sprint.rs     # 30s WPM sprint & combo multipliers
    └── ui/
        ├── mod.rs
        └── widgets.rs     # Header tabs, sound pill, footer hotkeys
```

---

## 📄 License
Licensed under the [MIT License](LICENSE). Open-source for the developer community.
