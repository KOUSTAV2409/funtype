mod audio;
mod modes;
mod particles;
pub mod stats;
mod theme;
mod ui;

use macroquad::prelude::*;
use audio::SoundEngine;
use modes::speed_sprint::SpeedSprint;
use modes::stress_shredder::StressShredder;
use modes::zen_flow::ZenFlow;
use modes::GameMode;
use particles::ParticleSystem;
use theme::ThemeMode;
use ui::widgets::UIWidgets;

fn window_conf() -> Conf {
    Conf {
        window_title: "FunType".to_string(),
        window_width: 1040,
        window_height: 660,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

const JETBRAINS_MONO: &[u8] = include_bytes!("../assets/fonts/jetbrains_mono.ttf");

#[macroquad::main(window_conf)]
async fn main() {
    // Smooth anti-aliased font texture filtering
    set_default_filter_mode(FilterMode::Linear);

    // Load and activate high-definition JetBrains Mono typography
    if let Ok(font) = load_ttf_font_from_bytes(JETBRAINS_MONO) {
        set_default_font(font);
    }

    let mut sound = SoundEngine::new().await;
    let mut theme_mode = ThemeMode::CatppuccinMocha;
    let mut current_mode = GameMode::StressShredder;

    let mut particles = ParticleSystem::new();
    let mut stress_shredder = StressShredder::new();
    let mut zen_flow = ZenFlow::new();
    let mut speed_sprint = SpeedSprint::new();

    loop {
        let dt = get_frame_time().min(0.1);
        let screen_w = screen_width();
        let screen_h = screen_height();
        let palette = theme_mode.palette();

        // 1. Process Global Hotkeys
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        // Mode Switching via F-keys
        if is_key_pressed(KeyCode::F1) {
            current_mode = GameMode::StressShredder;
        } else if is_key_pressed(KeyCode::F2) {
            current_mode = GameMode::ZenFlow;
        } else if is_key_pressed(KeyCode::F3) {
            current_mode = GameMode::SpeedSprint;
        }

        // Switch Sound Profile via Tab
        if is_key_pressed(KeyCode::Tab) {
            sound.current_switch = sound.current_switch.next();
            sound.play_keystroke();
            particles.spawn_floating_text(
                sound.current_switch.name(),
                screen_w - 140.0,
                75.0,
                palette.accent,
                16.0,
            );
        }

        // Switch Theme via F4
        if is_key_pressed(KeyCode::F4) {
            theme_mode = theme_mode.next();
            particles.spawn_floating_text(
                theme_mode.name(),
                screen_w - 280.0,
                75.0,
                palette.peach,
                16.0,
            );
        }

        // Safe Reset: Only triggers on Ctrl+R or F5 (NEVER bare 'R'!)
        let is_ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
        if (is_ctrl && is_key_pressed(KeyCode::R)) || is_key_pressed(KeyCode::F5) {
            match current_mode {
                GameMode::StressShredder => stress_shredder.reset(),
                GameMode::ZenFlow => zen_flow.reset(),
                GameMode::SpeedSprint => speed_sprint.reset(),
            }
            particles.spawn_floating_text("RESTARTED", screen_w * 0.5, screen_h * 0.5, palette.warning, 24.0);
        }

        // Cycle Sprint Duration via Ctrl+T
        if is_ctrl && is_key_pressed(KeyCode::T) && current_mode == GameMode::SpeedSprint {
            speed_sprint.cycle_duration();
            particles.spawn_floating_text(
                &format!("{:.0}s SPRINT", speed_sprint.duration),
                screen_w * 0.5,
                180.0,
                palette.primary,
                18.0,
            );
        }

        // Toggle Punctuation & Symbols via Ctrl+P
        if is_ctrl && is_key_pressed(KeyCode::P) && current_mode == GameMode::SpeedSprint {
            speed_sprint.toggle_punctuation();
            let punc_msg = if speed_sprint.punctuation_mode { "SYMBOLS ON" } else { "STANDARD WORDS" };
            particles.spawn_floating_text(
                punc_msg,
                screen_w * 0.5,
                180.0,
                palette.accent,
                18.0,
            );
        }

        // Backspace handling
        if is_key_pressed(KeyCode::Backspace) {
            match current_mode {
                GameMode::StressShredder => {
                    stress_shredder.handle_backspace();
                    sound.play_keystroke();
                }
                GameMode::ZenFlow => {
                    zen_flow.handle_backspace();
                    sound.play_keystroke();
                }
                GameMode::SpeedSprint => {
                    speed_sprint.handle_backspace();
                    sound.play_keystroke();
                }
            }
        }

        // 2. Process Typing Characters
        while let Some(c) = get_char_pressed() {
            // Ignore control characters or when Ctrl is held
            if is_ctrl || (c.is_control() && c != ' ') {
                continue;
            }

            match current_mode {
                GameMode::StressShredder => {
                    stress_shredder.handle_char(c, &mut sound, &mut particles, &palette);
                }
                GameMode::ZenFlow => {
                    zen_flow.handle_char(c, &mut sound, &mut particles, &palette);
                }
                GameMode::SpeedSprint => {
                    speed_sprint.handle_char(c, &mut sound, &mut particles, &palette);
                }
            }
        }

        // 3. Process Mouse Click on Header Tabs & Widgets
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let h_y = 18.0;
            let h_h = 44.0;
            if my >= h_y && my <= h_y + h_h {
                if mx >= 180.0 && mx <= 290.0 {
                    current_mode = GameMode::StressShredder;
                } else if mx >= 295.0 && mx <= 405.0 {
                    current_mode = GameMode::ZenFlow;
                } else if mx >= 410.0 && mx <= 500.0 {
                    current_mode = GameMode::SpeedSprint;
                } else if mx >= screen_w - 200.0 && mx <= screen_w - 30.0 {
                    sound.current_switch = sound.current_switch.next();
                    sound.play_keystroke();
                } else if mx >= screen_w - 360.0 && mx <= screen_w - 210.0 {
                    theme_mode = theme_mode.next();
                }
            } else if current_mode == GameMode::SpeedSprint {
                speed_sprint.handle_click(mx, my, screen_w, screen_h);
            }
        }

        // 4. Update Game State
        particles.update(dt);

        match current_mode {
            GameMode::StressShredder => {
                stress_shredder.update(dt, screen_h, &palette, &mut particles);
            }
            GameMode::ZenFlow => {
                zen_flow.update(dt);
            }
            GameMode::SpeedSprint => {
                speed_sprint.update(dt, screen_w, screen_h, &palette, &mut sound, &mut particles);
            }
        }

        let (shake_x, shake_y) = particles.get_shake_offset();

        // 5. Render Scene
        clear_background(palette.bg);

        match current_mode {
            GameMode::StressShredder => {
                stress_shredder.draw(screen_w, screen_h, &palette, shake_x, shake_y);
            }
            GameMode::ZenFlow => {
                zen_flow.draw(screen_w, screen_h, &palette, shake_x, shake_y);
            }
            GameMode::SpeedSprint => {
                speed_sprint.draw(screen_w, screen_h, &palette, shake_x, shake_y);
            }
        }

        particles.draw(shake_x, shake_y);

        UIWidgets::draw_header(
            screen_w,
            current_mode,
            sound.current_switch,
            theme_mode,
            &palette,
            0.0,
            0.0,
        );
        UIWidgets::draw_footer(screen_w, screen_h, &palette, 0.0, 0.0);
        UIWidgets::draw_window_frame(screen_w, screen_h, &palette, 0.0, 0.0);

        next_frame().await;
    }
}
