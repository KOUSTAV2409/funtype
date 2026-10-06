use macroquad::prelude::*;
use crate::audio::SwitchSound;
use crate::modes::GameMode;
use crate::theme::{ThemeMode, ThemePalette};

pub struct UIWidgets;

impl UIWidgets {
    pub fn draw_header(
        screen_w: f32,
        current_mode: GameMode,
        current_sound: SwitchSound,
        current_theme: ThemeMode,
        palette: &ThemePalette,
        offset_x: f32,
        offset_y: f32,
    ) {
        let h_y = 18.0 + offset_y;
        let h_h = 44.0;

        // Top bar container
        draw_rectangle(16.0 + offset_x, h_y, screen_w - 32.0, h_h, palette.surface);
        draw_rectangle_lines(16.0 + offset_x, h_y, screen_w - 32.0, h_h, 1.0, palette.border);

        // Logo
        let logo_text = "⚡ FunType";
        draw_text(logo_text, 36.0 + offset_x, h_y + 28.0, 22.0, palette.accent);

        // Mode tabs in center
        let modes = [
            (GameMode::StressShredder, "F1: Shredder"),
            (GameMode::ZenFlow, "F2: Zen Flow"),
            (GameMode::SpeedSprint, "F3: Sprint"),
        ];

        let mut tab_x = 180.0 + offset_x;
        for (m, label) in modes {
            let is_active = m == current_mode;
            let dims = measure_text(label, None, 14, 1.0);
            let btn_w = dims.width + 18.0;

            if is_active {
                draw_rectangle(tab_x, h_y + 8.0, btn_w, 28.0, palette.surface_bright);
                draw_rectangle_lines(tab_x, h_y + 8.0, btn_w, 28.0, 1.5, palette.primary);
                draw_text(label, tab_x + 9.0, h_y + 26.0, 14.0, palette.primary);
            } else {
                draw_text(label, tab_x + 9.0, h_y + 26.0, 14.0, palette.subtext);
            }

            tab_x += btn_w + 10.0;
        }

        // Right side pills: Sound & Theme
        let sound_label = format!("[Tab] {}", current_sound.name());
        let s_dims = measure_text(&sound_label, None, 14, 1.0);
        let s_w = s_dims.width + 16.0;
        let s_x = screen_w - 36.0 - s_w + offset_x;

        draw_rectangle(s_x, h_y + 8.0, s_w, 28.0, palette.surface_bright);
        draw_rectangle_lines(s_x, h_y + 8.0, s_w, 28.0, 1.0, palette.border);
        draw_text(&sound_label, s_x + 8.0, h_y + 26.0, 14.0, palette.text);

        let theme_label = format!("[F4] {}", current_theme.name());
        let t_dims = measure_text(&theme_label, None, 14, 1.0);
        let t_w = t_dims.width + 16.0;
        let t_x = s_x - t_w - 12.0;

        draw_rectangle(t_x, h_y + 8.0, t_w, 28.0, palette.surface_bright);
        draw_rectangle_lines(t_x, h_y + 8.0, t_w, 28.0, 1.0, palette.border);
        draw_text(&theme_label, t_x + 8.0, h_y + 26.0, 14.0, palette.peach);
    }

    pub fn draw_footer(
        screen_w: f32,
        screen_h: f32,
        palette: &ThemePalette,
        offset_x: f32,
        offset_y: f32,
    ) {
        let f_y = screen_h - 48.0 + offset_y;
        let f_h = 32.0;

        draw_rectangle(16.0 + offset_x, f_y, screen_w - 32.0, f_h, palette.surface);
        draw_rectangle_lines(16.0 + offset_x, f_y, screen_w - 32.0, f_h, 1.0, palette.border);

        let footer_hint = "[Esc] Exit   |   [F1-F3] Modes   |   [Tab] ASMR Switch   |   [F4] Theme   |   [Ctrl+R / F5] Reset";
        let dims = measure_text(footer_hint, None, 13, 1.0);
        draw_text(footer_hint, (screen_w - dims.width) * 0.5 + offset_x, f_y + 21.0, 13.0, palette.muted);
    }

    pub fn draw_window_frame(
        screen_w: f32,
        screen_h: f32,
        palette: &ThemePalette,
        offset_x: f32,
        offset_y: f32,
    ) {
        draw_rectangle_lines(
            2.0 + offset_x,
            2.0 + offset_y,
            screen_w - 4.0,
            screen_h - 4.0,
            2.0,
            palette.border,
        );
    }
}
