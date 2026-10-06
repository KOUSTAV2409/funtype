use macroquad::prelude::*;
use crate::audio::SoundEngine;
use crate::particles::ParticleSystem;
use crate::theme::ThemePalette;

pub struct ZenFlow {
    pub quotes: Vec<&'static str>,
    pub current_quote_idx: usize,
    pub typed_chars: Vec<char>,
    pub target_chars: Vec<char>,
    pub breath_timer: f32,
    pub quotes_completed: u32,
    pub caret_x: f32,
    pub caret_target_x: f32,
    pub caret_y: f32,
    pub caret_target_y: f32,
}

impl ZenFlow {
    pub fn new() -> Self {
        let quotes = vec![
            "Breathe in calm, breathe out the noise of the day.",
            "You have power over your mind, not outside events. Realize this, and you will find peace.",
            "Nature does not hurry, yet everything is accomplished.",
            "One keystroke at a time. The code will wait, your serenity comes first.",
            "Simplicity is the ultimate sophistication.",
            "Between stimulus and response there is a space. In that space lies your freedom.",
            "Do not anticipate trouble, or worry about what may never happen. Keep in the light.",
            "Water does not resist. Water flows. Be soft and adaptable.",
            "Quiet the mind, and the soul will speak.",
            "Tension is who you think you should be. Relaxation is who you are.",
            "Rest when you are weary. Refresh and renew yourself, your body, your mind.",
            "The mind is like water. When it is calm, everything becomes crystal clear.",
        ];

        let target_chars = quotes[0].chars().collect();

        Self {
            quotes,
            current_quote_idx: 0,
            typed_chars: Vec::new(),
            target_chars,
            breath_timer: 0.0,
            quotes_completed: 0,
            caret_x: 180.0,
            caret_target_x: 180.0,
            caret_y: 280.0,
            caret_target_y: 280.0,
        }
    }

    pub fn reset(&mut self) {
        self.current_quote_idx = (self.current_quote_idx + 1) % self.quotes.len();
        self.typed_chars.clear();
        self.target_chars = self.quotes[self.current_quote_idx].chars().collect();
    }

    pub fn handle_char(
        &mut self,
        c: char,
        sound: &SoundEngine,
        particles: &mut ParticleSystem,
        palette: &ThemePalette,
    ) {
        if self.typed_chars.len() < self.target_chars.len() {
            let expected = self.target_chars[self.typed_chars.len()];
            let is_correct = c == expected;
            self.typed_chars.push(c);
            sound.play_keystroke();

            let spark_col = if is_correct { palette.accent } else { palette.peach };
            particles.spawn_keystroke_spark(self.caret_x, self.caret_y - 8.0, spark_col);

            // Check if finished quote
            if self.typed_chars.len() == self.target_chars.len() {
                self.quotes_completed += 1;
                sound.play_combo();
                particles.spawn_burst(self.caret_x, self.caret_y, palette.success, 45);
                particles.spawn_floating_text("DEEP BREATH COMPLETED", self.caret_x, self.caret_y - 30.0, palette.success, 22.0);

                self.current_quote_idx = (self.current_quote_idx + 1) % self.quotes.len();
                self.typed_chars.clear();
                self.target_chars = self.quotes[self.current_quote_idx].chars().collect();
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        if !self.typed_chars.is_empty() {
            self.typed_chars.pop();
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.breath_timer += dt;
        if self.breath_timer >= 12.0 {
            self.breath_timer -= 12.0;
        }

        // Smooth caret lerp
        self.caret_x += (self.caret_target_x - self.caret_x) * (dt * 24.0).min(1.0);
        self.caret_y += (self.caret_target_y - self.caret_y) * (dt * 24.0).min(1.0);
    }

    pub fn draw(&mut self, screen_w: f32, screen_h: f32, palette: &ThemePalette, offset_x: f32, offset_y: f32) {
        // --- Breathing Guide Orb ---
        let breath_cycle = self.breath_timer;
        let (phase_name, target_scale) = if breath_cycle < 4.0 {
            let t = breath_cycle / 4.0;
            ("Inhale slowly...", 0.6 + t * 0.4)
        } else if breath_cycle < 6.0 {
            ("Hold gently...", 1.0)
        } else if breath_cycle < 10.0 {
            let t = (breath_cycle - 6.0) / 4.0;
            ("Exhale release...", 1.0 - t * 0.4)
        } else {
            ("Rest...", 0.6)
        };

        let orb_x = screen_w * 0.5 + offset_x;
        let orb_y = 120.0 + offset_y;
        let base_radius = 28.0;
        let current_radius = base_radius * target_scale;

        // Outer glow circle
        let mut glow_col = palette.accent;
        glow_col.a = 0.22;
        draw_circle(orb_x, orb_y, current_radius * 1.5, glow_col);

        // Core orb
        let mut core_col = palette.primary;
        core_col.a = 0.85;
        draw_circle(orb_x, orb_y, current_radius, core_col);

        // Breath text prompt
        let prompt_dims = measure_text(phase_name, None, 16, 1.0);
        draw_text(phase_name, orb_x - prompt_dims.width * 0.5, orb_y + current_radius + 24.0, 16.0, palette.subtext);

        // --- Typing Quote Area ---
        let start_x = 100.0 + offset_x;
        let start_y = 260.0 + offset_y;
        let font_size = 28.0;
        let line_height = 42.0;
        let max_w = screen_w - 200.0;

        let mut cur_x = start_x;
        let mut cur_y = start_y;

        for (i, &ch) in self.target_chars.iter().enumerate() {
            let ch_str = ch.to_string();
            let dims = measure_text(&ch_str, None, font_size as u16, 1.0);

            // Wrap line if needed
            if cur_x + dims.width > start_x + max_w && ch == ' ' {
                cur_x = start_x;
                cur_y += line_height;
                continue;
            }

            // Determine character color
            let color = if i < self.typed_chars.len() {
                if self.typed_chars[i] == ch {
                    palette.success
                } else {
                    palette.peach
                }
            } else {
                palette.muted
            };

            // If this is the current caret position
            if i == self.typed_chars.len() {
                self.caret_target_x = cur_x;
                self.caret_target_y = cur_y;
            }

            draw_text(&ch_str, cur_x, cur_y, font_size, color);
            cur_x += dims.width;
        }

        // If at the end of quote
        if self.typed_chars.len() == self.target_chars.len() {
            self.caret_target_x = cur_x;
            self.caret_target_y = cur_y;
        }

        // Draw smooth caret
        let caret_h = font_size * 0.85;
        let caret_alpha = (macroquad::time::get_time() as f32 * 3.5).sin().abs() * 0.7 + 0.3;
        let mut caret_col = palette.accent;
        caret_col.a = caret_alpha;
        draw_rectangle(self.caret_x, self.caret_y - caret_h + 4.0, 3.0, caret_h, caret_col);

        // Footer Zen info
        let progress_text = format!("Wisdom Reflections Completed: {}  |  Press Space / Backspace freely", self.quotes_completed);
        let prog_dims = measure_text(&progress_text, None, 15, 1.0);
        draw_text(&progress_text, (screen_w - prog_dims.width) * 0.5 + offset_x, screen_h - 110.0 + offset_y, 15.0, palette.muted);
    }
}
