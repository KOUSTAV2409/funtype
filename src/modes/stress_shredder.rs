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
    pub max_combo: u32,
    pub stress_level: f32, // 100.0 down to 0.0
    pub words_destroyed: u32,
    pub pool: Vec<&'static str>,
    pub duration_idx: usize, // 0: 15s, 1: 30s, 2: 60s, 3: Free Fall
    pub time_remaining: f32,
    pub is_started: bool,
    pub is_finished: bool,
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
            max_combo: 0,
            stress_level: 100.0,
            words_destroyed: 0,
            pool,
            duration_idx: 3, // Free Fall default
            time_remaining: 0.0,
            is_started: false,
            is_finished: false,
        };

        // Spawn initial words in distinct lanes
        instance.spawn_in_lane(0, 160.0);
        instance.spawn_in_lane(1, 140.0);
        instance.spawn_in_lane(2, 180.0);

        instance
    }

    pub fn current_duration(&self) -> Option<f32> {
        match self.duration_idx {
            0 => Some(15.0),
            1 => Some(30.0),
            2 => Some(60.0),
            _ => None,
        }
    }

    pub fn set_duration_idx(&mut self, idx: usize) {
        self.duration_idx = idx.min(3);
        self.reset();
    }

    pub fn cycle_duration(&mut self) {
        let next_idx = (self.duration_idx + 1) % 4;
        self.set_duration_idx(next_idx);
    }

    pub fn reset(&mut self) {
        self.words.clear();
        self.targeted_idx = None;
        self.spawn_timer = 0.5;
        self.score = 0;
        self.combo = 0;
        self.max_combo = 0;
        self.stress_level = 100.0;
        self.words_destroyed = 0;
        self.is_started = false;
        self.is_finished = false;
        self.time_remaining = self.current_duration().unwrap_or(0.0);

        self.spawn_in_lane(0, 160.0);
        self.spawn_in_lane(1, 140.0);
        self.spawn_in_lane(2, 180.0);
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
        if self.is_finished {
            return;
        }

        if !self.is_started {
            self.is_started = true;
        }

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
                            if self.combo > self.max_combo {
                                self.max_combo = self.combo;
                            }
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
                self.combo += 1;
                if self.combo > self.max_combo {
                    self.max_combo = self.combo;
                }
                self.score += 100 + self.combo * 15;
                self.words_destroyed += 1;
                self.stress_level = (self.stress_level - 6.0).max(0.0);
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

    pub fn handle_click(&mut self, mx: f32, my: f32, screen_w: f32, screen_h: f32) -> bool {
        let pill_widths = [56.0, 56.0, 56.0, 84.0];
        let spacing = 8.0;
        let total_w: f32 = pill_widths.iter().sum::<f32>() + (3.0 * spacing);

        if self.is_finished {
            let card_w = 540.0;
            let card_h = 320.0;
            let card_x = (screen_w - card_w) * 0.5;
            let card_y = (screen_h - card_h) * 0.5;
            let btn_y = card_y + card_h - 75.0;
            let btn_h = 28.0;

            if my >= btn_y && my <= btn_y + btn_h {
                let start_x = card_x + (card_w - total_w) * 0.5;
                let mut cur_x = start_x;
                for idx in 0..4 {
                    let pw = pill_widths[idx];
                    if mx >= cur_x && mx <= cur_x + pw {
                        self.set_duration_idx(idx);
                        return true;
                    }
                    cur_x += pw + spacing;
                }
            }
            return false;
        }

        // Live HUD pills
        let bar_y = 114.0;
        let bar_h = 26.0;
        if my >= bar_y && my <= bar_y + bar_h {
            let start_x = (screen_w - total_w) * 0.5;
            let mut cur_x = start_x;
            for idx in 0..4 {
                let pw = pill_widths[idx];
                if mx >= cur_x && mx <= cur_x + pw {
                    self.set_duration_idx(idx);
                    return true;
                }
                cur_x += pw + spacing;
            }
        }

        false
    }

    pub fn update(
        &mut self,
        dt: f32,
        screen_h: f32,
        palette: &ThemePalette,
        sound: &mut SoundEngine,
        particles: &mut ParticleSystem,
    ) {
        if self.is_started && !self.is_finished {
            if let Some(_) = self.current_duration() {
                self.time_remaining -= dt;
                if self.time_remaining <= 0.0 {
                    self.time_remaining = 0.0;
                    self.is_finished = true;
                    sound.play_combo();
                    let s_w = screen_width();
                    particles.spawn_burst(s_w * 0.5, screen_h * 0.5 - 60.0, palette.warning, 60);
                    particles.spawn_floating_text(
                        "TIME'S UP!",
                        s_w * 0.5 - 60.0,
                        screen_h * 0.5 - 110.0,
                        palette.warning,
                        26.0,
                    );
                    return;
                }
            }
        }

        if self.is_finished {
            return;
        }

        // Spawn timer
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 && self.words.len() < 6 {
            // Find lanes with no words near the top
            let mut lane_occupied = [false; 4];
            for w in &self.words {
                if w.y < 240.0 && w.lane < 4 {
                    lane_occupied[w.lane] = true;
                }
            }

            let available_lanes: Vec<usize> = (0..4).filter(|&l| !lane_occupied[l]).collect();
            if !available_lanes.is_empty() {
                let selected_lane = available_lanes[rand::gen_range(0, available_lanes.len())];
                self.spawn_in_lane(selected_lane, 145.0);
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

    pub fn draw(&self, screen_w: f32, screen_h: f32, palette: &ThemePalette, offset_x: f32, offset_y: f32) {
        let pill_widths = [56.0, 56.0, 56.0, 84.0];
        let labels = ["15s", "30s", "60s", "Free Fall"];
        let spacing = 8.0;
        let total_w: f32 = pill_widths.iter().sum::<f32>() + (3.0 * spacing);

        if self.is_finished {
            let card_w = 540.0;
            let card_h = 320.0;
            let card_x = (screen_w - card_w) * 0.5 + offset_x;
            let card_y = (screen_h - card_h) * 0.5 + offset_y;

            draw_rectangle(card_x, card_y, card_w, card_h, palette.surface_bright);
            draw_rectangle_lines(card_x, card_y, card_w, card_h, 2.0, palette.accent);

            let title = "TIME'S UP! STRESS SHREDDED";
            let t_dims = measure_text(title, None, 22, 1.0);
            draw_text(title, card_x + (card_w - t_dims.width) * 0.5, card_y + 38.0, 22.0, palette.warning);

            // Left: Score
            let score_val = format!("{}", self.score);
            draw_text("score", card_x + 50.0, card_y + 75.0, 15.0, palette.subtext);
            draw_text(&score_val, card_x + 50.0, card_y + 118.0, 42.0, palette.accent);

            // Middle: Purged
            let purged_val = format!("{}", self.words_destroyed);
            draw_text("purged", card_x + 220.0, card_y + 75.0, 15.0, palette.subtext);
            draw_text(&purged_val, card_x + 220.0, card_y + 118.0, 42.0, palette.success);

            // Right: Relief %
            let relief_val = format!("{:.0}%", (100.0 - self.stress_level).max(0.0));
            draw_text("relief", card_x + 380.0, card_y + 75.0, 15.0, palette.subtext);
            draw_text(&relief_val, card_x + 380.0, card_y + 118.0, 42.0, palette.peach);

            // Detailed row
            let streak_str = format!("max streak: {}x", self.max_combo);
            let mode_str = match self.current_duration() {
                Some(s) => format!("mode: {:.0}s shred", s),
                None => "mode: free fall".to_string(),
            };
            draw_text(&streak_str, card_x + 50.0, card_y + 165.0, 17.0, palette.primary);
            draw_text(&mode_str, card_x + 220.0, card_y + 165.0, 17.0, palette.subtext);

            // Rematch pills
            let btn_y = card_y + card_h - 75.0;
            let start_x = card_x + (card_w - total_w) * 0.5;
            let mut cur_x = start_x;

            for (idx, &label) in labels.iter().enumerate() {
                let pw = pill_widths[idx];
                let is_sel = self.duration_idx == idx;

                if is_sel {
                    draw_rectangle(cur_x, btn_y, pw, 26.0, palette.accent);
                    let l_dims = measure_text(label, None, 13, 1.0);
                    draw_text(label, cur_x + (pw - l_dims.width) * 0.5, btn_y + 18.0, 13.0, palette.bg);
                } else {
                    draw_rectangle(cur_x, btn_y, pw, 26.0, palette.surface);
                    draw_rectangle_lines(cur_x, btn_y, pw, 26.0, 1.0, palette.border);
                    let l_dims = measure_text(label, None, 13, 1.0);
                    draw_text(label, cur_x + (pw - l_dims.width) * 0.5, btn_y + 18.0, 13.0, palette.subtext);
                }
                cur_x += pw + spacing;
            }

            let hint = "Click a mode above or press [Ctrl+R] to shred again";
            let h_dims = measure_text(hint, None, 13, 1.0);
            draw_text(hint, card_x + (card_w - h_dims.width) * 0.5, card_y + card_h - 18.0, 13.0, palette.muted);
            return;
        }

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

        // Stats text
        let timer_part = match self.current_duration() {
            Some(_) => format!("Time: {:.1}s", self.time_remaining),
            None => "Free Fall".to_string(),
        };
        let stat_text = format!("{}  |  Purged: {}  |  Score: {}  |  Streak: {}x", timer_part, self.words_destroyed, self.score, self.combo);
        let stat_dims = measure_text(&stat_text, None, 15, 1.0);
        draw_text(&stat_text, (screen_w - stat_dims.width) * 0.5 + offset_x, bar_y + 28.0, 15.0, palette.subtext);

        // Mode Duration selector pills
        let start_pills_x = (screen_w - total_w) * 0.5 + offset_x;
        let pill_y = 108.0 + offset_y;
        let mut cur_x = start_pills_x;

        for (idx, &label) in labels.iter().enumerate() {
            let pw = pill_widths[idx];
            let is_sel = self.duration_idx == idx;

            if is_sel {
                draw_rectangle(cur_x, pill_y, pw, 24.0, palette.surface_bright);
                draw_rectangle_lines(cur_x, pill_y, pw, 24.0, 1.5, palette.primary);
                let l_dims = measure_text(label, None, 13, 1.0);
                draw_text(label, cur_x + (pw - l_dims.width) * 0.5, pill_y + 16.0, 13.0, palette.primary);
            } else {
                draw_rectangle(cur_x, pill_y, pw, 24.0, palette.surface);
                draw_rectangle_lines(cur_x, pill_y, pw, 24.0, 1.0, palette.border);
                let l_dims = measure_text(label, None, 13, 1.0);
                draw_text(label, cur_x + (pw - l_dims.width) * 0.5, pill_y + 16.0, 13.0, palette.muted);
            }

            cur_x += pw + spacing;
        }

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
