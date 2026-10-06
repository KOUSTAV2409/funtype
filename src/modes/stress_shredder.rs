use macroquad::prelude::*;
use crate::audio::SoundEngine;
use crate::particles::ParticleSystem;
use crate::theme::ThemePalette;

pub struct StressWord {
    pub text: String,
    pub typed_len: usize,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

pub struct StressShredder {
    pub words: Vec<StressWord>,
    pub targeted_idx: Option<usize>,
    pub spawn_timer: f32,
    pub score: u32,
    pub combo: u32,
    pub stress_level: f32, // 100.0 down to 0.0
    pub words_destroyed: u32,
    pub pool: Vec<&'static str>,
}

impl StressShredder {
    pub fn new() -> Self {
        let pool = vec![
            "merge conflict",
            "prod incident",
            "unresolved blocker",
            "endless meeting",
            "legacy codebase",
            "null pointer",
            "segfault",
            "git push --force",
            "rebase failed",
            "deadline tomorrow",
            "memory leak",
            "sync meeting",
            "jira sprint",
            "spaghetti code",
            "404 not found",
            "emergency call",
            "unpaid overtime",
            "scope creep",
            "out of memory",
            "infinite loop",
            "broken pipeline",
            "nitpick review",
            "dirty tree",
            "deprecated api",
            "circular dependency",
            "context switch",
            "cpu spike",
            "dirty git status",
            "unhandled promise",
            "stack overflow",
            "cache invalidated",
            "flaky test",
        ];

        let mut instance = Self {
            words: Vec::new(),
            targeted_idx: None,
            spawn_timer: 0.2,
            score: 0,
            combo: 0,
            stress_level: 100.0,
            words_destroyed: 0,
            pool,
        };

        // Spawn initial words
        instance.spawn_word(300.0, 160.0);
        instance.spawn_word(600.0, 110.0);
        instance.spawn_word(850.0, 200.0);

        instance
    }

    pub fn reset(&mut self) {
        self.words.clear();
        self.targeted_idx = None;
        self.spawn_timer = 0.5;
        self.score = 0;
        self.combo = 0;
        self.stress_level = 100.0;
        self.words_destroyed = 0;

        self.spawn_word(300.0, 160.0);
        self.spawn_word(600.0, 110.0);
    }

    fn spawn_word(&mut self, x: f32, y: f32) {
        let idx = rand::gen_range(0, self.pool.len());
        let text = self.pool[idx].to_string();
        let speed = rand::gen_range(28.0, 52.0);

        self.words.push(StressWord {
            text,
            typed_len: 0,
            x,
            y,
            speed,
        });
    }

    pub fn handle_char(
        &mut self,
        c: char,
        sound: &SoundEngine,
        particles: &mut ParticleSystem,
        palette: &ThemePalette,
    ) {
        let c_lower = c.to_ascii_lowercase();

        // If we already have a targeted word
        if let Some(target_idx) = self.targeted_idx {
            if target_idx < self.words.len() {
                let target = &mut self.words[target_idx];
                let chars: Vec<char> = target.text.chars().collect();
                if target.typed_len < chars.len() && chars[target.typed_len].to_ascii_lowercase() == c_lower {
                    target.typed_len += 1;
                    sound.play_keystroke();

                    let char_x = target.x + (target.typed_len as f32 * 14.0);
                    particles.spawn_keystroke_spark(char_x, target.y + 12.0, palette.accent);

                    // Check if word completed!
                    if target.typed_len == chars.len() {
                        let word_x = target.x + (target.text.len() as f32 * 7.0);
                        let word_y = target.y;

                        sound.play_shatter();
                        particles.spawn_burst(word_x, word_y, palette.success, 38);

                        self.combo += 1;
                        let combo_bonus = self.combo * 15;
                        self.score += 100 + combo_bonus;
                        self.words_destroyed += 1;

                        // Stress relief drop
                        self.stress_level = (self.stress_level - 6.0).max(0.0);

                        let burst_msg = if self.combo > 1 {
                            format!("-STRESS! {}x COMBO", self.combo)
                        } else {
                            "STRESS PURGED!".to_string()
                        };
                        particles.spawn_floating_text(&burst_msg, word_x, word_y - 20.0, palette.success, 20.0);

                        if self.combo % 5 == 0 {
                            sound.play_combo();
                            particles.spawn_floating_text("SERENITY BURST!", word_x, word_y - 45.0, palette.accent, 24.0);
                        }

                        self.words.remove(target_idx);
                        self.targeted_idx = None;
                    }
                    return;
                }
            }
        }

        // Search for a new matching word
        let mut best_match: Option<usize> = None;
        let mut lowest_y = -1.0;

        for (i, word) in self.words.iter().enumerate() {
            if let Some(first_char) = word.text.chars().next() {
                if first_char.to_ascii_lowercase() == c_lower && word.y > lowest_y {
                    lowest_y = word.y;
                    best_match = Some(i);
                }
            }
        }

        if let Some(idx) = best_match {
            self.targeted_idx = Some(idx);
            self.words[idx].typed_len = 1;
            sound.play_keystroke();

            let target = &self.words[idx];
            particles.spawn_keystroke_spark(target.x + 14.0, target.y + 12.0, palette.primary);

            // In case 1-letter word completed
            if target.typed_len >= target.text.len() {
                sound.play_shatter();
                particles.spawn_burst(target.x, target.y, palette.success, 30);
                self.words.remove(idx);
                self.targeted_idx = None;
            }
        } else {
            // Typing mismatch: subtle reset combo
            self.combo = 0;
        }
    }

    pub fn update(&mut self, dt: f32, screen_w: f32, screen_h: f32, palette: &ThemePalette, particles: &mut ParticleSystem) {
        // Spawn timer
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 && self.words.len() < 7 {
            let x = rand::gen_range(160.0, screen_w - 280.0);
            let y = rand::gen_range(90.0, 130.0);
            self.spawn_word(x, y);
            self.spawn_timer = rand::gen_range(1.6, 2.8);
        }

        // Move words
        let mut i = 0;
        while i < self.words.len() {
            let word = &mut self.words[i];
            word.y += word.speed * dt;

            // If word reached bottom, gracefully dissolve without punishing player
            if word.y > screen_h - 110.0 {
                particles.spawn_floating_text("let it go...", word.x + 40.0, word.y - 10.0, palette.muted, 16.0);
                self.words.remove(i);
                if self.targeted_idx == Some(i) {
                    self.targeted_idx = None;
                } else if let Some(t_idx) = self.targeted_idx {
                    if t_idx > i {
                        self.targeted_idx = Some(t_idx - 1);
                    }
                }
            } else {
                i += 1;
            }
        }
    }

    pub fn draw(&self, screen_w: f32, _screen_h: f32, palette: &ThemePalette, offset_x: f32, offset_y: f32) {
        // Stress Bar at top
        let bar_w = 420.0;
        let bar_h = 14.0;
        let bar_x = (screen_w - bar_w) * 0.5 + offset_x;
        let bar_y = 66.0 + offset_y;

        // Label
        let stress_label = if self.stress_level <= 0.1 {
            "ZEN NIRVANA REACHED (0% STRESS)".to_string()
        } else {
            format!("WORKPLACE STRESS LEVEL: {:.0}%", self.stress_level)
        };
        let label_dims = measure_text(&stress_label, None, 14, 1.0);
        let label_col = if self.stress_level < 30.0 { palette.success } else if self.stress_level < 70.0 { palette.warning } else { palette.danger };
        draw_text(&stress_label, bar_x + (bar_w - label_dims.width) * 0.5, bar_y - 6.0, 14.0, label_col);

        // Bar background
        draw_rectangle(bar_x, bar_y, bar_w, bar_h, palette.surface_bright);
        // Bar fill
        let fill_w = bar_w * (self.stress_level / 100.0);
        draw_rectangle(bar_x, bar_y, fill_w, bar_h, label_col);
        // Bar outline
        draw_rectangle_lines(bar_x, bar_y, bar_w, bar_h, 1.5, palette.border);

        // Stats summary row
        let stat_text = format!("Purged: {}  |  Score: {}  |  Combo: {}x", self.words_destroyed, self.score, self.combo);
        let stat_dims = measure_text(&stat_text, None, 16, 1.0);
        draw_text(&stat_text, (screen_w - stat_dims.width) * 0.5 + offset_x, bar_y + 30.0, 16.0, palette.subtext);

        // Render each falling word
        for (i, word) in self.words.iter().enumerate() {
            let is_targeted = self.targeted_idx == Some(i);
            let font_size = 22.0;
            let text_dims = measure_text(&word.text, None, font_size as u16, 1.0);

            let pad_x = 12.0;
            let pad_y = 6.0;
            let box_x = word.x - pad_x + offset_x;
            let box_y = word.y - text_dims.height - pad_y + offset_y;
            let box_w = text_dims.width + pad_x * 2.0;
            let box_h = text_dims.height + pad_y * 2.0 + 4.0;

            // Word bubble background
            let bg_color = if is_targeted {
                palette.surface_bright
            } else {
                Color::new(palette.surface.r, palette.surface.g, palette.surface.b, 0.85)
            };
            draw_rectangle(box_x, box_y, box_w, box_h, bg_color);

            // Border
            let border_color = if is_targeted {
                palette.accent
            } else {
                palette.border
            };
            draw_rectangle_lines(box_x, box_y, box_w, box_h, if is_targeted { 2.0 } else { 1.0 }, border_color);

            // If targeted, draw targeting laser/indicator dot
            if is_targeted {
                draw_circle(box_x - 10.0, box_y + box_h * 0.5, 4.0, palette.accent);
            }

            // Draw text chars: typed portion highlighted, remaining normal
            let typed_str: String = word.text.chars().take(word.typed_len).collect();
            let remain_str: String = word.text.chars().skip(word.typed_len).collect();

            let typed_dims = measure_text(&typed_str, None, font_size as u16, 1.0);

            // Typed part
            draw_text(&typed_str, word.x + offset_x, word.y + offset_y, font_size, palette.success);

            // Remaining part
            draw_text(&remain_str, word.x + typed_dims.width + offset_x, word.y + offset_y, font_size, if is_targeted { palette.text } else { palette.subtext });
        }
    }
}
