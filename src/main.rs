mod audio;
mod modes;
mod particles;
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

#[macroquad::main(window_conf)]
async fn main() {
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

        if is_key_pressed(KeyCode::Key1) {
            current_mode = GameMode::StressShredder;
        } else if is_key_pressed(KeyCode::Key2) {
            current_mode = GameMode::ZenFlow;
        } else if is_key_pressed(KeyCode::Key3) {
            current_mode = GameMode::SpeedSprint;
        }

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

        if is_key_pressed(KeyCode::F2) {
            theme_mode = theme_mode.next();
            particles.spawn_floating_text(
                theme_mode.name(),
                screen_w - 280.0,
                75.0,
                palette.peach,
                16.0,
            );
        }

        if is_key_pressed(KeyCode::R) {
            match current_mode {
                GameMode::StressShredder => stress_shredder.reset(),
                GameMode::ZenFlow => zen_flow.reset(),
                GameMode::SpeedSprint => speed_sprint.reset(),
            }
            particles.spawn_floating_text("RESTARTED", screen_w * 0.5, screen_h * 0.5, palette.warning, 24.0);
        }

        // 2. Process Typing Characters
        while let Some(c) = get_char_pressed() {
            // Ignore control characters
            if c.is_control() && c != ' ' {
                continue;
            }

            match current_mode {
                GameMode::StressShredder => {
                    stress_shredder.handle_char(c, &sound, &mut particles, &palette);
                }
                GameMode::ZenFlow => {
                    zen_flow.handle_char(c, &sound, &mut particles, &palette);
                }
                GameMode::SpeedSprint => {
                    speed_sprint.handle_char(c, &sound, &mut particles, &palette);
                }
            }
        }

        // Handle Backspace for text input modes
        if is_key_pressed(KeyCode::Backspace) {
            match current_mode {
                GameMode::ZenFlow => {
                    zen_flow.handle_backspace();
                    sound.play_keystroke();
                }
                GameMode::SpeedSprint => {
                    // In speed sprint, backspace can be used if desired
                }
                _ => {}
            }
        }

        // 3. Process Mouse Click on Header / Tabs
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let h_y = 18.0;
            let h_h = 44.0;
            if my >= h_y && my <= h_y + h_h {
                // Check mode buttons
                if mx >= 200.0 && mx <= 310.0 {
                    current_mode = GameMode::StressShredder;
                } else if mx >= 320.0 && mx <= 430.0 {
                    current_mode = GameMode::ZenFlow;
                } else if mx >= 440.0 && mx <= 530.0 {
                    current_mode = GameMode::SpeedSprint;
                }
                // Check sound pill
                else if mx >= screen_w - 220.0 && mx <= screen_w - 30.0 {
                    sound.current_switch = sound.current_switch.next();
                    sound.play_keystroke();
                }
            }
        }

        // 4. Update Game State
        particles.update(dt);

        match current_mode {
            GameMode::StressShredder => {
                stress_shredder.update(dt, screen_w, screen_h, &palette, &mut particles);
            }
            GameMode::ZenFlow => {
                zen_flow.update(dt);
            }
            GameMode::SpeedSprint => {
                speed_sprint.update(dt);
            }
        }

        let (shake_x, shake_y) = particles.get_shake_offset();

        // 5. Render Scene
        clear_background(palette.bg);

        // Draw active mode
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

        // Draw particles & floating texts
        particles.draw(shake_x, shake_y);

        // Draw HUD Header, Footer & Frame
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
