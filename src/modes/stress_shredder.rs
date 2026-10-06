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
    pub error_timer: f32,
    pub lane: usize,
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
            spawn_timer: 0.1,
            score: 0,
            combo: 0,
            stress_level: 100.0,
            words_destroyed: 0,
            pool,
        };

        // Spawn initial words in distinct lanes
        instance.spawn_in_lane(0, 140.0);
        instance.spawn_in_lane(1, 95.0);
        instance.spawn_in_lane(2, 170.0);

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

        self.spawn_in_lane(0, 140.0);
        self.spawn_in_lane(1, 95.0);
        self.spawn_in_lane(2, 170.0);
    }

    fn spawn_in_lane(&mut self, lane: usize, initial_y: f32) {
        let total_lanes = 4;
        let screen_w = screen_width().max(800.0);
        let lane_width = (screen_w - 200.0) / total_lanes as f32;
        let x = 100.0 + (lane as f32 * lane_width) + rand::gen_range(10.0, lane_width * 0.35);

        let idx = rand::gen_range(0, self.pool.len());
        let text = self.pool[idx].to_string();
        let speed = rand::gen_range(28.0, 48.0);

        self.words.push(StressWord {
            text,
            typed_len: 0,
            x,
            y: initial_y,
            speed,
            error_timer: 0.0,
            lane,
        });
    }

    pub fn handle_char(
        &mut self,
        c: char,
        sound: &mut SoundEngine,
        particles: &mut ParticleSystem,
        palette: &ThemePalette,
    ) {
        let c_lower = c.to_ascii_lowercase();

        // 1. If currently targeting a word, process next char of that word
        if let Some(target_idx) = self.targeted_idx {
            if target_idx < self.words.len() {
                let target = &mut self.words[target_idx];
                let chars: Vec<char> = target.text.chars().collect();

                if target.typed_len < chars.len() {
                    let expected_char = chars[target.typed_len].to_ascii_lowercase();

                    if expected_char == c_lower {
                        // Correct keystroke!
                        target.typed_len += 1;
                        target.error_timer = 0.0;
                        sound.play_keystroke();

                        let font_size = 22.0;
                        let typed_sub: String = chars[..target.typed_len].iter().collect();
                        let typed_dims = measure_text(&typed_sub, None, font_size as u16, 1.0);
                        let spark_x = target.x + typed_dims.width;
                        let spark_y = target.y + 4.0;
                        particles.spawn_keystroke_spark(spark_x, spark_y, palette.accent);

                        // Check word completion!
                        if target.typed_len == chars.len() {
                            let total_dims = measure_text(&target.text, None, font_size as u16, 1.0);
                            let word_center_x = target.x + total_dims.width * 0.5;
                            let word_center_y = target.y;

                            sound.play_shatter();
                            particles.spawn_burst(word_center_x, word_center_y, palette.success, 38);

                            self.combo += 1;
                            let combo_bonus = self.combo * 15;
                            self.score += 100 + combo_bonus;
                            self.words_destroyed += 1;
                            self.stress_level = (self.stress_level - 6.0).max(0.0);

                            let burst_msg = if self.combo > 1 {
                                format!("-STRESS! {}x COMBO", self.combo)
                            } else {
                                "STRESS PURGED!".to_string()
                            };
                            particles.spawn_floating_text(&burst_msg, word_center_x, word_center_y - 20.0, palette.success, 20.0);

                            if self.combo % 5 == 0 {
                                sound.play_combo();
                                particles.spawn_floating_text("SERENITY BURST!", word_center_x, word_center_y - 45.0, palette.accent, 24.0);
                            }

                            self.words.remove(target_idx);
                            self.targeted_idx = None;
                        }
                        return;
                    } else {
                        // Mistake on current target: flash red, reset combo, but DO NOT drop target
                        target.error_timer = 0.25;
                        self.combo = 0;
                        sound.play_keystroke();
                        particles.spawn_keystroke_spark(target.x, target.y, palette.danger);
                        return;
                    }
                }
            }
        }

        // 2. No target active: search for a word starting with this character
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
            let target = &mut self.words[idx];
            target.typed_len = 1;
            sound.play_keystroke();

            let font_size = 22.0;
            let typed_dims = measure_text(&target.text[..1], None, font_size as u16, 1.0);
            particles.spawn_keystroke_spark(target.x + typed_dims.width, target.y + 4.0, palette.primary);

            if target.typed_len >= target.text.len() {
                sound.play_shatter();
                particles.spawn_burst(target.x, target.y, palette.success, 30);
                self.words.remove(idx);
                self.targeted_idx = None;
            }
        } else {
            // No matching word on screen
            self.combo = 0;
        }
    }

    pub fn handle_backspace(&mut self) {
        if let Some(target_idx) = self.targeted_idx {
            if target_idx < self.words.len() {
                let target = &mut self.words[target_idx];
                if target.typed_len > 0 {
                    target.typed_len -= 1;
                    if target.typed_len == 0 {
                        // Deselect target if all characters removed
                        self.targeted_idx = None;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32, screen_h: f32, palette: &ThemePalette, particles: &mut ParticleSystem) {
        // Spawn timer
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 && self.words.len() < 6 {
            // Find lanes with no words near the top
            let mut lane_occupied = [false; 4];
            for w in &self.words {
                if w.y < 220.0 && w.lane < 4 {
                    lane_occupied[w.lane] = true;
                }
            }

            let available_lanes: Vec<usize> = (0..4).filter(|&l| !lane_occupied[l]).collect();
            if !available_lanes.is_empty() {
                let selected_lane = available_lanes[rand::gen_range(0, available_lanes.len())];
                self.spawn_in_lane(selected_lane, 95.0);
            }
            self.spawn_timer = rand::gen_range(1.6, 2.6);
        }

        // Move words and decay errors
        let mut i = 0;
        while i < self.words.len() {
            let word = &mut self.words[i];
            word.y += word.speed * dt;
            if word.error_timer > 0.0 {
                word.error_timer = (word.error_timer - dt).max(0.0);
            }

            // Word reached bottom
            if word.y > screen_h - 110.0 {
                particles.spawn_floating_text("let it go...", word.x + 30.0, word.y - 10.0, palette.muted, 16.0);
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

        let stress_label = if self.stress_level <= 0.1 {
            "ZEN NIRVANA REACHED (0% STRESS)".to_string()
        } else {
            format!("WORKPLACE STRESS LEVEL: {:.0}%", self.stress_level)
        };
        let label_dims = measure_text(&stress_label, None, 14, 1.0);
        let label_col = if self.stress_level < 30.0 {
            palette.success
        } else if self.stress_level < 70.0 {
            palette.warning
        } else {
            palette.danger
        };
        draw_text(&stress_label, bar_x + (bar_w - label_dims.width) * 0.5, bar_y - 6.0, 14.0, label_col);

        draw_rectangle(bar_x, bar_y, bar_w, bar_h, palette.surface_bright);
        let fill_w = bar_w * (self.stress_level / 100.0);
        draw_rectangle(bar_x, bar_y, fill_w, bar_h, label_col);
        draw_rectangle_lines(bar_x, bar_y, bar_w, bar_h, 1.5, palette.border);

        let stat_text = format!("Purged: {}  |  Score: {}  |  Combo: {}x", self.words_destroyed, self.score, self.combo);
        let stat_dims = measure_text(&stat_text, None, 16, 1.0);
        draw_text(&stat_text, (screen_w - stat_dims.width) * 0.5 + offset_x, bar_y + 30.0, 16.0, palette.subtext);

        // Render each falling word
        for (i, word) in self.words.iter().enumerate() {
            let is_targeted = self.targeted_idx == Some(i);
            let font_size = 22.0;
            let text_dims = measure_text(&word.text, None, font_size as u16, 1.0);

            let pad_x = 14.0;
            let pad_y = 7.0;
            let box_x = word.x - pad_x + offset_x;
            let box_y = word.y - text_dims.offset_y - pad_y + offset_y;
            let box_w = text_dims.width + pad_x * 2.0;
            let box_h = text_dims.height + pad_y * 2.0;

            let bg_color = if word.error_timer > 0.0 {
                Color::new(palette.danger.r * 0.35, 0.1, 0.1, 0.95)
            } else if is_targeted {
                palette.surface_bright
            } else {
                Color::new(palette.surface.r, palette.surface.g, palette.surface.b, 0.9)
            };
            draw_rectangle(box_x, box_y, box_w, box_h, bg_color);

            let border_color = if word.error_timer > 0.0 {
                palette.danger
            } else if is_targeted {
                palette.accent
            } else {
                palette.border
            };
            draw_rectangle_lines(box_x, box_y, box_w, box_h, if is_targeted { 2.0 } else { 1.0 }, border_color);

            if is_targeted {
                draw_circle(box_x - 10.0, box_y + box_h * 0.5, 4.0, palette.accent);
            }

            // Draw letters cleanly
            let chars: Vec<char> = word.text.chars().collect();
            let mut char_x = word.x + offset_x;

            for (c_idx, &ch) in chars.iter().enumerate() {
                let display_str = if ch == ' ' && is_targeted && c_idx == word.typed_len {
                    "␣".to_string() // Show spacebar symbol for active space
                } else {
                    ch.to_string()
                };

                let ch_dims = measure_text(&ch.to_string(), None, font_size as u16, 1.0);
                let col = if c_idx < word.typed_len {
                    palette.success
                } else if is_targeted && c_idx == word.typed_len {
                    palette.accent
                } else if is_targeted {
                    palette.text
                } else {
                    palette.subtext
                };

                draw_text(&display_str, char_x, word.y + offset_y, font_size, col);
                char_x += ch_dims.width;
            }
        }
    }
}
