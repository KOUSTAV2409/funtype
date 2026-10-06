use macroquad::prelude::*;
use crate::audio::SoundEngine;
use crate::particles::ParticleSystem;
use crate::theme::ThemePalette;

pub struct SpeedSprint {
    pub words: Vec<String>,
    pub current_word_idx: usize,
    pub current_char_idx: usize,
    pub correct_keystrokes: u32,
    pub total_keystrokes: u32,
    pub time_remaining: f32,
    pub duration: f32,
    pub is_started: bool,
    pub is_finished: bool,
    pub combo: u32,
    pub max_combo: u32,
}

impl SpeedSprint {
    pub fn new() -> Self {
        let mut sprint = Self {
            words: Vec::new(),
            current_word_idx: 0,
            current_char_idx: 0,
            correct_keystrokes: 0,
            total_keystrokes: 0,
            time_remaining: 30.0,
            duration: 30.0,
            is_started: false,
            is_finished: false,
            combo: 0,
            max_combo: 0,
        };
        sprint.generate_words();
        sprint
    }

    pub fn reset(&mut self) {
        self.generate_words();
        self.current_word_idx = 0;
        self.current_char_idx = 0;
        self.correct_keystrokes = 0;
        self.total_keystrokes = 0;
        self.time_remaining = self.duration;
        self.is_started = false;
        self.is_finished = false;
        self.combo = 0;
        self.max_combo = 0;
    }

    fn generate_words(&mut self) {
        let pool = vec![
            "focus", "clarity", "rhythm", "serene", "stream", "breath", "flow", "code",
            "compile", "deploy", "rust", "speed", "smooth", "system", "kernel", "linux",
            "zen", "tranquil", "moment", "release", "horizon", "bright", "space", "energy",
            "matrix", "create", "balance", "silent", "pure", "glide", "swift", "crystal",
            "spark", "vivid", "gentle", "wisdom", "motion", "peace", "thock", "click",
            "ambient", "tactile", "spring", "neural", "pulse", "echo", "dynamic", "craft",
        ];

        self.words.clear();
        for _ in 0..70 {
            let idx = rand::gen_range(0, pool.len());
            self.words.push(pool[idx].to_string());
        }
    }

    pub fn handle_char(
        &mut self,
        c: char,
        sound: &SoundEngine,
        particles: &mut ParticleSystem,
        palette: &ThemePalette,
    ) {
        if self.is_finished {
            return;
        }

        if !self.is_started {
            self.is_started = true;
        }

        self.total_keystrokes += 1;

        if self.current_word_idx >= self.words.len() {
            return;
        }

        let current_word = &self.words[self.current_word_idx];
        let word_chars: Vec<char> = current_word.chars().collect();

        if c == ' ' {
            // Space pressed: move to next word
            if self.current_char_idx > 0 {
                self.current_word_idx += 1;
                self.current_char_idx = 0;
                sound.play_keystroke();

                if self.combo > 5 {
                    particles.spawn_burst(400.0, 300.0, palette.success, 16);
                }
            }
            return;
        }

        if self.current_char_idx < word_chars.len() {
            if word_chars[self.current_char_idx] == c {
                self.correct_keystrokes += 1;
                self.current_char_idx += 1;
                self.combo += 1;
                if self.combo > self.max_combo {
                    self.max_combo = self.combo;
                }
                sound.play_keystroke();

                let col = if self.combo > 15 { palette.warning } else { palette.accent };
                particles.spawn_keystroke_spark(450.0, 310.0, col);

                if self.current_char_idx == word_chars.len() {
                    // Completed current word
                    self.current_word_idx += 1;
                    self.current_char_idx = 0;
                    if self.combo % 10 == 0 {
                        sound.play_combo();
                        particles.spawn_floating_text(&format!("{}x COMBO STREAK!", self.combo), 450.0, 240.0, palette.warning, 22.0);
                    }
                }
            } else {
                // Mistake
                self.combo = 0;
                sound.play_keystroke();
                particles.spawn_keystroke_spark(450.0, 310.0, palette.danger);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_started && !self.is_finished {
            self.time_remaining -= dt;
            if self.time_remaining <= 0.0 {
                self.time_remaining = 0.0;
                self.is_finished = true;
            }
        }
    }

    pub fn wpm(&self) -> f32 {
        let elapsed = (self.duration - self.time_remaining).max(0.1);
        let words_typed = (self.correct_keystrokes as f32) / 5.0;
        (words_typed / (elapsed / 60.0)).max(0.0)
    }

    pub fn accuracy(&self) -> f32 {
        if self.total_keystrokes == 0 {
            100.0
        } else {
            (self.correct_keystrokes as f32 / self.total_keystrokes as f32) * 100.0
        }
    }

    pub fn draw(&self, screen_w: f32, screen_h: f32, palette: &ThemePalette, offset_x: f32, offset_y: f32) {
        if self.is_finished {
            // Results Card
            let card_w = 480.0;
            let card_h = 280.0;
            let card_x = (screen_w - card_w) * 0.5 + offset_x;
            let card_y = (screen_h - card_h) * 0.5 + offset_y;

            draw_rectangle(card_x, card_y, card_w, card_h, palette.surface_bright);
            draw_rectangle_lines(card_x, card_y, card_w, card_h, 2.0, palette.accent);

            let title = "SPRINT COMPLETE";
            let t_dims = measure_text(title, None, 28, 1.0);
            draw_text(title, card_x + (card_w - t_dims.width) * 0.5, card_y + 45.0, 28.0, palette.accent);

            let wpm_str = format!("WPM: {:.1}", self.wpm());
            let acc_str = format!("Accuracy: {:.1}%", self.accuracy());
            let combo_str = format!("Max Combo: {}x", self.max_combo);
            let stress_relieved = format!("Stress Points Relieved: +{}", (self.wpm() * 12.0) as u32);

            draw_text(&wpm_str, card_x + 50.0, card_y + 105.0, 24.0, palette.success);
            draw_text(&acc_str, card_x + 50.0, card_y + 145.0, 24.0, palette.primary);
            draw_text(&combo_str, card_x + 50.0, card_y + 185.0, 24.0, palette.peach);
            draw_text(&stress_relieved, card_x + 50.0, card_y + 225.0, 20.0, palette.warning);

            let restart_hint = "Press [R] to Play Again";
            let r_dims = measure_text(restart_hint, None, 16, 1.0);
            draw_text(restart_hint, card_x + (card_w - r_dims.width) * 0.5, card_y + card_h - 18.0, 16.0, palette.subtext);
            return;
        }

        // Live HUD Metrics Header
        let timer_str = format!("Time: {:.1}s", self.time_remaining);
        let wpm_str = format!("WPM: {:.0}", self.wpm());
        let acc_str = format!("Acc: {:.0}%", self.accuracy());
        let combo_str = format!("Combo: {}x", self.combo);

        let metrics = format!("{}   |   {}   |   {}   |   {}", timer_str, wpm_str, acc_str, combo_str);
        let m_dims = measure_text(&metrics, None, 20, 1.0);
        draw_text(&metrics, (screen_w - m_dims.width) * 0.5 + offset_x, 100.0 + offset_y, 20.0, palette.accent);

        // Word Stream Display
        let start_x = 120.0 + offset_x;
        let start_y = 260.0 + offset_y;
        let font_size = 28.0;
        let line_height = 44.0;
        let max_w = screen_w - 240.0;

        let mut cur_x = start_x;
        let mut cur_y = start_y;

        let start_word = self.current_word_idx.saturating_sub(2);
        let end_word = (start_word + 24).min(self.words.len());

        for w_idx in start_word..end_word {
            let word = &self.words[w_idx];
            let w_dims = measure_text(word, None, font_size as u16, 1.0);

            if cur_x + w_dims.width + 16.0 > start_x + max_w {
                cur_x = start_x;
                cur_y += line_height;
            }

            if w_idx == self.current_word_idx {
                // Highlight active word
                let chars: Vec<char> = word.chars().collect();
                for (c_i, &ch) in chars.iter().enumerate() {
                    let ch_s = ch.to_string();
                    let ch_dims = measure_text(&ch_s, None, font_size as u16, 1.0);
                    let col = if c_i < self.current_char_idx {
                        palette.success
                    } else if c_i == self.current_char_idx {
                        palette.text
                    } else {
                        palette.subtext
                    };
                    draw_text(&ch_s, cur_x, cur_y, font_size, col);

                    // Active letter underline
                    if c_i == self.current_char_idx {
                        draw_rectangle(cur_x, cur_y + 4.0, ch_dims.width, 2.5, palette.accent);
                    }
                    cur_x += ch_dims.width;
                }
                cur_x += 16.0;
            } else if w_idx < self.current_word_idx {
                // Already typed
                draw_text(word, cur_x, cur_y, font_size, palette.muted);
                cur_x += w_dims.width + 16.0;
            } else {
                // Upcoming
                draw_text(word, cur_x, cur_y, font_size, palette.subtext);
                cur_x += w_dims.width + 16.0;
            }
        }

        if !self.is_started {
            let start_hint = "Start typing any letter to begin the 30s sprint";
            let s_dims = measure_text(start_hint, None, 16, 1.0);
            draw_text(start_hint, (screen_w - s_dims.width) * 0.5 + offset_x, screen_h - 110.0 + offset_y, 16.0, palette.peach);
        }
    }
}
