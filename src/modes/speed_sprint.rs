use macroquad::prelude::*;
use crate::audio::SoundEngine;
use crate::particles::ParticleSystem;
use crate::stats::UserStats;
use crate::theme::ThemePalette;

pub const AVAILABLE_DURATIONS: [f32; 3] = [15.0, 30.0, 60.0];

pub struct SpeedSprint {
    pub words: Vec<String>,
    pub current_word_idx: usize,
    pub current_input: String,
    pub typed_history: Vec<String>,
    pub correct_chars: u32,
    pub incorrect_chars: u32,
    pub extra_chars: u32,
    pub missed_chars: u32,
    pub time_remaining: f32,
    pub duration: f32,
    pub is_started: bool,
    pub is_finished: bool,
    pub combo: u32,
    pub max_combo: u32,
    pub caret_x: f32,
    pub caret_y: f32,
    pub target_caret_x: f32,
    pub target_caret_y: f32,
    pub stats: UserStats,
    pub is_new_pb: bool,
    pub pb_burst_spawned: bool,
    pub punctuation_mode: bool,
}

impl SpeedSprint {
    pub fn new() -> Self {
        let mut sprint = Self {
            words: Vec::new(),
            current_word_idx: 0,
            current_input: String::new(),
            typed_history: Vec::new(),
            correct_chars: 0,
            incorrect_chars: 0,
            extra_chars: 0,
            missed_chars: 0,
            time_remaining: 15.0,
            duration: 15.0,
            is_started: false,
            is_finished: false,
            combo: 0,
            max_combo: 0,
            caret_x: 120.0,
            caret_y: 250.0,
            target_caret_x: 120.0,
            target_caret_y: 250.0,
            stats: UserStats::new(),
            is_new_pb: false,
            pb_burst_spawned: false,
            punctuation_mode: false,
        };
        sprint.generate_words();
        sprint
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
        self.reset();
    }

    pub fn cycle_duration(&mut self) {
        let next_dur = match self.duration as u32 {
            15 => 30.0,
            30 => 60.0,
            _ => 15.0,
        };
        self.set_duration(next_dur);
    }

    pub fn toggle_punctuation(&mut self) {
        self.punctuation_mode = !self.punctuation_mode;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.generate_words();
        self.current_word_idx = 0;
        self.current_input.clear();
        self.typed_history.clear();
        self.correct_chars = 0;
        self.incorrect_chars = 0;
        self.extra_chars = 0;
        self.missed_chars = 0;
        self.time_remaining = self.duration;
        self.is_started = false;
        self.is_finished = false;
        self.combo = 0;
        self.max_combo = 0;
        self.caret_x = 120.0;
        self.caret_y = 250.0;
        self.target_caret_x = 120.0;
        self.target_caret_y = 250.0;
        self.is_new_pb = false;
        self.pb_burst_spawned = false;
    }

    fn generate_words(&mut self) {
        // Standard high-frequency English typing words (Monkeytype English 200 standard)
        let pool = vec![
            "the", "be", "to", "of", "and", "a", "in", "that", "have", "it", "for", "not",
            "on", "with", "he", "as", "you", "do", "at", "this", "but", "his", "by", "from",
            "they", "we", "say", "her", "she", "or", "an", "will", "my", "one", "all", "would",
            "there", "their", "what", "so", "up", "out", "if", "about", "who", "get", "which",
            "go", "me", "when", "make", "can", "like", "time", "no", "just", "him", "know",
            "take", "people", "into", "year", "your", "good", "some", "could", "them", "see",
            "other", "than", "then", "now", "look", "only", "come", "its", "over", "think",
            "also", "back", "after", "use", "two", "how", "our", "work", "first", "well",
            "way", "even", "new", "want", "because", "any", "these", "give", "day", "most",
            "us", "great", "such", "through", "code", "focus", "flow", "mind", "calm", "speed",
            "system", "rust", "state", "point", "form", "life", "light", "space", "clear",
            "real", "still", "world", "hand", "high", "place", "hold", "turn", "right", "move",
        ];

        let num_pool = vec!["42", "100", "2026", "1", "10", "256", "365", "12", "7", "99"];

        self.words.clear();
        for _ in 0..160 {
            if self.punctuation_mode && rand::gen_range(0, 10) == 0 {
                let n_idx = rand::gen_range(0, num_pool.len());
                self.words.push(num_pool[n_idx].to_string());
                continue;
            }

            let idx = rand::gen_range(0, pool.len());
            let mut word = pool[idx].to_string();

            if self.punctuation_mode {
                let p_choice = rand::gen_range(0, 10);
                match p_choice {
                    1 => word = format!("{},", word),
                    2 => word = format!("{}.", word),
                    3 => word = format!("\"{}\"", word),
                    4 => word = format!("{}:", word),
                    5 => word = format!("{};", word),
                    6 => word = format!("({})", word),
                    7 => word = format!("{}?", word),
                    8 => word = format!("{}!", word),
                    _ => {}
                }
            }

            self.words.push(word);
        }
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

        if self.current_word_idx >= self.words.len() {
            return;
        }

        let target_word = &self.words[self.current_word_idx];
        let target_chars: Vec<char> = target_word.chars().collect();

        // 1. Spacebar pressed: commit current word and advance
        if c == ' ' {
            if !self.current_input.is_empty() {
                sound.play_keystroke();

                // Check missed characters if user pressed space early
                if self.current_input.len() < target_chars.len() {
                    self.missed_chars += (target_chars.len() - self.current_input.len()) as u32;
                    self.combo = 0;
                } else if self.current_input == *target_word {
                    // Space counts as 1 correct character!
                    self.correct_chars += 1;
                    self.combo += 1;
                    if self.combo > self.max_combo {
                        self.max_combo = self.combo;
                    }

                    if self.combo > 5 {
                        particles.spawn_burst(self.caret_x, self.caret_y - 10.0, palette.success, 12);
                    }
                    if self.combo % 10 == 0 {
                        sound.play_combo();
                        particles.spawn_floating_text(
                            &format!("{}x STREAK!", self.combo),
                            self.caret_x,
                            self.caret_y - 30.0,
                            palette.warning,
                            22.0,
                        );
                    }
                } else {
                    self.combo = 0;
                    particles.spawn_keystroke_spark(self.caret_x, self.caret_y - 10.0, palette.danger);
                }

                self.typed_history.push(self.current_input.clone());
                self.current_word_idx += 1;
                self.current_input.clear();
            }
            return;
        }

        // 2. Character typed
        let char_idx = self.current_input.len();
        self.current_input.push(c);
        sound.play_keystroke();

        if char_idx < target_chars.len() {
            if c == target_chars[char_idx] {
                // Correct character!
                self.correct_chars += 1;
                self.combo += 1;
                if self.combo > self.max_combo {
                    self.max_combo = self.combo;
                }
                let spark_col = if self.combo > 15 { palette.warning } else { palette.accent };
                particles.spawn_keystroke_spark(self.caret_x, self.caret_y - 8.0, spark_col);
            } else {
                // Typo!
                self.incorrect_chars += 1;
                self.combo = 0;
                particles.spawn_keystroke_spark(self.caret_x, self.caret_y - 8.0, palette.danger);
            }
        } else {
            // Extra character beyond target length
            self.extra_chars += 1;
            self.combo = 0;
            particles.spawn_keystroke_spark(self.caret_x, self.caret_y - 8.0, palette.danger);
        }
    }

    pub fn handle_backspace(&mut self) {
        if !self.current_input.is_empty() {
            let last_idx = self.current_input.len() - 1;
            if self.current_word_idx < self.words.len() {
                let target_chars: Vec<char> = self.words[self.current_word_idx].chars().collect();
                if let Some(last_ch) = self.current_input.chars().last() {
                    if last_idx < target_chars.len() && last_ch == target_chars[last_idx] {
                        // Undoing a correct character
                        self.correct_chars = self.correct_chars.saturating_sub(1);
                    }
                }
            }
            self.current_input.pop();
        }
    }

    pub fn handle_click(&mut self, mx: f32, my: f32, screen_w: f32, screen_h: f32) -> bool {
        if self.is_finished {
            let card_w = 540.0;
            let card_h = 320.0;
            let card_x = (screen_w - card_w) * 0.5;
            let card_y = (screen_h - card_h) * 0.5;
            let btn_y = card_y + card_h - 70.0;
            let btn_h = 32.0;

            if my >= btn_y && my <= btn_y + btn_h {
                let pill_w = 64.0;
                let spacing = 12.0;
                let total_w = 3.0 * pill_w + 2.0 * spacing;
                let start_x = card_x + (card_w - total_w) * 0.5;

                for (idx, &dur) in AVAILABLE_DURATIONS.iter().enumerate() {
                    let bx = start_x + (idx as f32 * (pill_w + spacing));
                    if mx >= bx && mx <= bx + pill_w {
                        self.set_duration(dur);
                        return true;
                    }
                }
            }
            return false;
        }

        // Check clicks on Main Sprint Duration Bar & Punctuation Toggle
        let bar_y = 138.0;
        let bar_h = 28.0;
        if my >= bar_y && my <= bar_y + bar_h {
            let pill_w = 60.0;
            let spacing = 8.0;
            let total_time_pills_w = 3.0 * pill_w + 2.0 * spacing;
            let punc_pill_w = 114.0;
            let gap = 16.0;
            let label = "Time:";
            let l_dims = measure_text(label, None, 15, 1.0);
            let total_bar_w = l_dims.width + 12.0 + total_time_pills_w + gap + punc_pill_w;
            let start_bar_x = (screen_w - total_bar_w) * 0.5;
            let start_pills_x = start_bar_x + l_dims.width + 12.0;

            for (idx, &dur) in AVAILABLE_DURATIONS.iter().enumerate() {
                let bx = start_pills_x + (idx as f32 * (pill_w + spacing));
                if mx >= bx && mx <= bx + pill_w {
                    self.set_duration(dur);
                    return true;
                }
            }

            let punc_x = start_pills_x + total_time_pills_w + gap;
            if mx >= punc_x && mx <= punc_x + punc_pill_w {
                self.toggle_punctuation();
                return true;
            }
        }

        false
    }

    pub fn update(
        &mut self,
        dt: f32,
        screen_w: f32,
        screen_h: f32,
        palette: &ThemePalette,
        sound: &mut SoundEngine,
        particles: &mut ParticleSystem,
    ) {
        if self.is_started && !self.is_finished {
            self.time_remaining -= dt;
            if self.time_remaining <= 0.0 {
                self.time_remaining = 0.0;
                self.is_finished = true;
                let final_wpm = self.wpm();
                if self.stats.record_score(self.duration, final_wpm) {
                    self.is_new_pb = true;
                }
            }
        }

        if self.is_finished && self.is_new_pb && !self.pb_burst_spawned {
            self.pb_burst_spawned = true;
            sound.play_combo();
            let card_x = screen_w * 0.5;
            let card_y = screen_h * 0.5;
            particles.spawn_burst(card_x, card_y - 60.0, palette.warning, 60);
            particles.spawn_floating_text(
                "★ NEW PERSONAL BEST! ★",
                card_x - 110.0,
                card_y - 120.0,
                palette.warning,
                24.0,
            );
        }

        // Smooth Caret Lerp
        let blend = (dt * 30.0).min(1.0);
        self.caret_x += (self.target_caret_x - self.caret_x) * blend;
        self.caret_y += (self.target_caret_y - self.caret_y) * blend;
    }

    /// Standard Net WPM: (correct_chars / 5) / (elapsed_minutes)
    pub fn wpm(&self) -> f32 {
        let elapsed_sec = (self.duration - self.time_remaining).max(0.1);
        let elapsed_min = elapsed_sec / 60.0;
        let words_typed = self.correct_chars as f32 / 5.0;
        (words_typed / elapsed_min).max(0.0)
    }

    /// Standard Raw WPM: ((correct_chars + incorrect_chars + extra_chars) / 5) / (elapsed_minutes)
    pub fn raw_wpm(&self) -> f32 {
        let elapsed_sec = (self.duration - self.time_remaining).max(0.1);
        let elapsed_min = elapsed_sec / 60.0;
        let total_chars = self.correct_chars + self.incorrect_chars + self.extra_chars;
        ((total_chars as f32 / 5.0) / elapsed_min).max(0.0)
    }

    /// Standard Accuracy: correct_chars / (correct_chars + incorrect_chars + extra_chars) * 100%
    pub fn accuracy(&self) -> f32 {
        let total_attempts = self.correct_chars + self.incorrect_chars + self.extra_chars;
        if total_attempts == 0 {
            100.0
        } else {
            ((self.correct_chars as f32 / total_attempts as f32) * 100.0).clamp(0.0, 100.0)
        }
    }

    pub fn draw(&mut self, screen_w: f32, screen_h: f32, palette: &ThemePalette, offset_x: f32, offset_y: f32) {
        if self.is_finished {
            // Results Card (Styled after Monkeytype's celebrated clean layout)
            let card_w = 540.0;
            let card_h = 320.0;
            let card_x = (screen_w - card_w) * 0.5 + offset_x;
            let card_y = (screen_h - card_h) * 0.5 + offset_y;

            draw_rectangle(card_x, card_y, card_w, card_h, palette.surface_bright);
            draw_rectangle_lines(card_x, card_y, card_w, card_h, 2.0, palette.accent);

            // Header Row: Big WPM, Accuracy, and Personal Best
            let wpm_val = format!("{:.0}", self.wpm());
            let acc_val = format!("{:.0}%", self.accuracy());
            let pb_val = format!("{:.0}", self.stats.get_pb(self.duration));

            // Left: WPM
            draw_text("wpm", card_x + 50.0, card_y + 40.0, 16.0, palette.subtext);
            draw_text(&wpm_val, card_x + 50.0, card_y + 92.0, 52.0, palette.warning);

            // Center: Accuracy
            draw_text("acc", card_x + 210.0, card_y + 40.0, 16.0, palette.subtext);
            draw_text(&acc_val, card_x + 210.0, card_y + 92.0, 52.0, palette.success);

            // Right: Personal Best
            draw_text("best", card_x + 370.0, card_y + 40.0, 16.0, palette.subtext);
            draw_text(&pb_val, card_x + 370.0, card_y + 92.0, 52.0, palette.peach);

            if self.is_new_pb {
                let badge = "★ NEW PERSONAL BEST! ★";
                let b_dims = measure_text(badge, None, 15, 1.0);
                draw_text(badge, card_x + (card_w - b_dims.width) * 0.5, card_y + 24.0, 15.0, palette.warning);
            }

            // Detailed stats row below
            let raw_str = format!("raw: {:.0}", self.raw_wpm());
            let time_str = format!("time: {:.0}s", self.duration);
            let combo_str = format!("streak: {}x", self.max_combo);
            let chars_str = format!(
                "characters: {}/{}/{}/{}",
                self.correct_chars, self.incorrect_chars, self.extra_chars, self.missed_chars
            );
            let mode_str = if self.punctuation_mode { "mode: symbols" } else { "mode: standard" };

            draw_text(&raw_str, card_x + 50.0, card_y + 140.0, 18.0, palette.primary);
            draw_text(&time_str, card_x + 210.0, card_y + 140.0, 18.0, palette.text);
            draw_text(&combo_str, card_x + 370.0, card_y + 140.0, 18.0, palette.peach);
            draw_text(&chars_str, card_x + 50.0, card_y + 175.0, 18.0, palette.subtext);
            draw_text(mode_str, card_x + 370.0, card_y + 175.0, 18.0, palette.muted);

            // Rematch pills
            let pill_w = 64.0;
            let spacing = 12.0;
            let total_w = 3.0 * pill_w + 2.0 * spacing;
            let start_x = card_x + (card_w - total_w) * 0.5;
            let btn_y = card_y + card_h - 70.0;

            for (idx, &dur) in AVAILABLE_DURATIONS.iter().enumerate() {
                let bx = start_x + (idx as f32 * (pill_w + spacing));
                let label = format!("{:.0}s", dur);
                let is_selected = (self.duration - dur).abs() < 0.1;

                if is_selected {
                    draw_rectangle(bx, btn_y, pill_w, 28.0, palette.accent);
                    let l_dims = measure_text(&label, None, 14, 1.0);
                    draw_text(&label, bx + (pill_w - l_dims.width) * 0.5, btn_y + 19.0, 14.0, palette.bg);
                } else {
                    draw_rectangle(bx, btn_y, pill_w, 28.0, palette.surface);
                    draw_rectangle_lines(bx, btn_y, pill_w, 28.0, 1.0, palette.border);
                    let l_dims = measure_text(&label, None, 14, 1.0);
                    draw_text(&label, bx + (pill_w - l_dims.width) * 0.5, btn_y + 19.0, 14.0, palette.subtext);
                }
            }

            let restart_hint = "Click a duration above or press [Ctrl+R] to play again";
            let r_dims = measure_text(restart_hint, None, 13, 1.0);
            draw_text(restart_hint, card_x + (card_w - r_dims.width) * 0.5, card_y + card_h - 16.0, 13.0, palette.muted);
            return;
        }

        // 1. Live HUD Metrics Header
        let timer_str = format!("Time: {:.1}s", self.time_remaining);
        let wpm_str = format!("WPM: {:.0}", self.wpm());
        let acc_str = format!("Acc: {:.0}%", self.accuracy());
        let combo_str = format!("Streak: {}x", self.combo);

        let metrics = format!("{}   |   {}   |   {}   |   {}", timer_str, wpm_str, acc_str, combo_str);
        let m_dims = measure_text(&metrics, None, 20, 1.0);
        draw_text(&metrics, (screen_w - m_dims.width) * 0.5 + offset_x, 96.0 + offset_y, 20.0, palette.accent);

        // 2. Interactive Time Selector Bar (15s | 30s | 60s) + Punctuation Toggle + PB Badge
        let pill_w = 60.0;
        let spacing = 8.0;
        let total_time_pills_w = 3.0 * pill_w + 2.0 * spacing;
        let punc_pill_w = 114.0;
        let gap = 16.0;
        let label = "Time:";
        let l_dims = measure_text(label, None, 15, 1.0);
        let bar_total_w = l_dims.width + 12.0 + total_time_pills_w + gap + punc_pill_w;
        let start_bar_x = (screen_w - bar_total_w) * 0.5 + offset_x;
        let bar_y = 138.0 + offset_y;

        draw_text(label, start_bar_x, bar_y + 18.0, 15.0, palette.subtext);
        let start_pills_x = start_bar_x + l_dims.width + 12.0;

        for (idx, &dur) in AVAILABLE_DURATIONS.iter().enumerate() {
            let bx = start_pills_x + (idx as f32 * (pill_w + spacing));
            let dur_str = format!("{:.0}s", dur);
            let is_selected = (self.duration - dur).abs() < 0.1;

            if is_selected {
                draw_rectangle(bx, bar_y, pill_w, 26.0, palette.surface_bright);
                draw_rectangle_lines(bx, bar_y, pill_w, 26.0, 1.5, palette.primary);
                let d_dims = measure_text(&dur_str, None, 14, 1.0);
                draw_text(&dur_str, bx + (pill_w - d_dims.width) * 0.5, bar_y + 18.0, 14.0, palette.primary);
            } else {
                draw_rectangle(bx, bar_y, pill_w, 26.0, palette.surface);
                draw_rectangle_lines(bx, bar_y, pill_w, 26.0, 1.0, palette.border);
                let d_dims = measure_text(&dur_str, None, 14, 1.0);
                draw_text(&dur_str, bx + (pill_w - d_dims.width) * 0.5, bar_y + 18.0, 14.0, palette.muted);
            }
        }

        // Punctuation toggle pill
        let punc_x = start_pills_x + total_time_pills_w + gap;
        let punc_label = if self.punctuation_mode { "@ symbols ON" } else { "@ symbols" };
        if self.punctuation_mode {
            draw_rectangle(punc_x, bar_y, punc_pill_w, 26.0, palette.surface_bright);
            draw_rectangle_lines(punc_x, bar_y, punc_pill_w, 26.0, 1.5, palette.accent);
            let p_dims = measure_text(punc_label, None, 13, 1.0);
            draw_text(punc_label, punc_x + (punc_pill_w - p_dims.width) * 0.5, bar_y + 18.0, 13.0, palette.accent);
        } else {
            draw_rectangle(punc_x, bar_y, punc_pill_w, 26.0, palette.surface);
            draw_rectangle_lines(punc_x, bar_y, punc_pill_w, 26.0, 1.0, palette.border);
            let p_dims = measure_text(punc_label, None, 13, 1.0);
            draw_text(punc_label, punc_x + (punc_pill_w - p_dims.width) * 0.5, bar_y + 18.0, 13.0, palette.muted);
        }

        // Best Record Pill (if PB exists)
        let cur_pb = self.stats.get_pb(self.duration);
        if cur_pb > 0.0 {
            let pb_badge = format!("PB: {:.0} WPM", cur_pb);
            let pb_x = punc_x + punc_pill_w + 14.0;
            draw_text(&pb_badge, pb_x, bar_y + 18.0, 13.0, palette.peach);
        }

        // 3. Word Stream Display with stable 3-row layout and character-by-character color
        let start_x = 120.0 + offset_x;
        let start_y = 250.0 + offset_y;
        let font_size = 28.0;
        let line_height = 48.0;
        let max_w = screen_w - 240.0;
        let space_w = 16.0;

        let mut lines: Vec<Vec<usize>> = Vec::new();
        let mut current_line: Vec<usize> = Vec::new();
        let mut current_line_w = 0.0;

        for w_idx in 0..self.words.len() {
            let w_dims = measure_text(&self.words[w_idx], None, font_size as u16, 1.0);
            if current_line_w + w_dims.width + space_w > max_w && !current_line.is_empty() {
                lines.push(current_line);
                current_line = Vec::new();
                current_line_w = 0.0;
            }
            current_line.push(w_idx);
            current_line_w += w_dims.width + space_w;
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        let mut active_line_idx = 0;
        for (l_idx, line) in lines.iter().enumerate() {
            if line.contains(&self.current_word_idx) {
                active_line_idx = l_idx;
                break;
            }
        }

        let visible_start_line = active_line_idx.saturating_sub(1);
        let visible_end_line = (visible_start_line + 3).min(lines.len());

        for (display_row, l_idx) in (visible_start_line..visible_end_line).enumerate() {
            let line = &lines[l_idx];
            let row_y = start_y + (display_row as f32 * line_height);
            let mut cur_x = start_x;

            for &w_idx in line {
                let target_word = &self.words[w_idx];
                let w_dims = measure_text(target_word, None, font_size as u16, 1.0);

                if w_idx == self.current_word_idx {
                    // Active word: render characters with real-time feedback
                    let target_chars: Vec<char> = target_word.chars().collect();
                    let input_chars: Vec<char> = self.current_input.chars().collect();

                    let mut char_x = cur_x;
                    let max_len = target_chars.len().max(input_chars.len());

                    for i in 0..max_len {
                        let ch_to_draw = if i < input_chars.len() {
                            input_chars[i]
                        } else {
                            target_chars[i]
                        };

                        let ch_s = ch_to_draw.to_string();
                        let ch_w = measure_text(&ch_s, None, font_size as u16, 1.0).width;

                        let col = if i < input_chars.len() {
                            if i < target_chars.len() && input_chars[i] == target_chars[i] {
                                palette.text // Correct character typed
                            } else {
                                palette.danger // Typo
                            }
                        } else {
                            palette.muted // Untyped remaining character
                        };

                        draw_text(&ch_s, char_x, row_y, font_size, col);

                        // Target Caret position
                        if i == input_chars.len() {
                            self.target_caret_x = char_x;
                            self.target_caret_y = row_y;
                        }

                        char_x += ch_w;
                    }

                    if input_chars.len() >= target_chars.len() {
                        self.target_caret_x = char_x;
                        self.target_caret_y = row_y;
                    }

                    cur_x += w_dims.width.max(char_x - cur_x) + space_w;
                } else if w_idx < self.current_word_idx {
                    // Previous completed word: draw with actual correctness history!
                    if w_idx < self.typed_history.len() {
                        let typed_history_word = &self.typed_history[w_idx];
                        let is_perfect = typed_history_word == target_word;
                        let col = if is_perfect { palette.subtext } else { palette.danger };
                        draw_text(target_word, cur_x, row_y, font_size, col);
                    } else {
                        draw_text(target_word, cur_x, row_y, font_size, palette.subtext);
                    }
                    cur_x += w_dims.width + space_w;
                } else {
                    // Upcoming word
                    draw_text(target_word, cur_x, row_y, font_size, palette.muted);
                    cur_x += w_dims.width + space_w;
                }
            }
        }

        // Draw Smooth Caret Bar
        if !self.is_finished {
            let caret_h = font_size * 0.85;
            let caret_alpha = if !self.is_started {
                (macroquad::time::get_time() as f32 * 3.5).sin().abs() * 0.6 + 0.4
            } else {
                0.95
            };
            let mut caret_col = palette.accent;
            caret_col.a = caret_alpha;
            draw_rectangle(self.caret_x, self.caret_y - caret_h + 4.0, 2.8, caret_h, caret_col);
        }

        if !self.is_started {
            let start_hint = "Type the first word and press Space to start | Click or [Ctrl+T] for time";
            let s_dims = measure_text(start_hint, None, 14, 1.0);
            draw_text(start_hint, (screen_w - s_dims.width) * 0.5 + offset_x, screen_h - 110.0 + offset_y, 14.0, palette.peach);
        }
    }
}
