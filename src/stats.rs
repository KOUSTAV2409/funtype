use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct UserStats {
    pub pb_15: f32,
    pub pb_30: f32,
    pub pb_60: f32,
    pub weak_letters: HashMap<char, u32>,
    pub weak_bigrams: HashMap<String, u32>,
}

impl UserStats {
    pub fn new() -> Self {
        Self::load()
    }

    fn stats_file_path() -> Option<PathBuf> {
        let base = if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
            PathBuf::from(xdg)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".local").join("share")
        } else {
            return None;
        };

        let dir = base.join("funtype");
        let _ = fs::create_dir_all(&dir);
        Some(dir.join("stats.txt"))
    }

    pub fn load() -> Self {
        let mut stats = Self {
            pb_15: 0.0,
            pb_30: 0.0,
            pb_60: 0.0,
            weak_letters: HashMap::new(),
            weak_bigrams: HashMap::new(),
        };

        if let Some(path) = Self::stats_file_path() {
            if let Ok(content) = fs::read_to_string(path) {
                for line in content.lines() {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();
                        let val_str = parts[1].trim();
                        match key {
                            "pb_15" => stats.pb_15 = val_str.parse().unwrap_or(0.0),
                            "pb_30" => stats.pb_30 = val_str.parse().unwrap_or(0.0),
                            "pb_60" => stats.pb_60 = val_str.parse().unwrap_or(0.0),
                            "weak_letters" => {
                                for item in val_str.split(',') {
                                    let kv: Vec<&str> = item.split(':').collect();
                                    if kv.len() == 2 {
                                        if let Some(c) = kv[0].chars().next() {
                                            if let Ok(cnt) = kv[1].parse::<u32>() {
                                                if cnt > 0 {
                                                    stats.weak_letters.insert(c.to_ascii_lowercase(), cnt);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            "weak_bigrams" => {
                                for item in val_str.split(',') {
                                    let kv: Vec<&str> = item.split(':').collect();
                                    if kv.len() == 2 {
                                        let bg = kv[0].to_ascii_lowercase();
                                        if let Ok(cnt) = kv[1].parse::<u32>() {
                                            if cnt > 0 && bg.len() >= 2 {
                                                stats.weak_bigrams.insert(bg, cnt);
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        stats
    }

    pub fn save(&self) {
        if let Some(path) = Self::stats_file_path() {
            let mut letters_str = String::new();
            for (c, cnt) in &self.weak_letters {
                if *cnt > 0 {
                    if !letters_str.is_empty() {
                        letters_str.push(',');
                    }
                    letters_str.push_str(&format!("{}:{}", c, cnt));
                }
            }

            let mut bigrams_str = String::new();
            for (bg, cnt) in &self.weak_bigrams {
                if *cnt > 0 {
                    if !bigrams_str.is_empty() {
                        bigrams_str.push(',');
                    }
                    bigrams_str.push_str(&format!("{}:{}", bg, cnt));
                }
            }

            let content = format!(
                "pb_15={:.1}\npb_30={:.1}\npb_60={:.1}\nweak_letters={}\nweak_bigrams={}\n",
                self.pb_15, self.pb_30, self.pb_60, letters_str, bigrams_str
            );
            let _ = fs::write(path, content);
        }
    }

    pub fn get_pb(&self, duration: f32) -> f32 {
        let dur = duration as u32;
        match dur {
            15 => self.pb_15,
            30 => self.pb_30,
            60 => self.pb_60,
            _ => 0.0,
        }
    }

    /// Records a mistyped character and its bigram transition (increases weakness score)
    pub fn record_typo(&mut self, expected_char: char, prev_char: Option<char>) {
        let ch = expected_char.to_ascii_lowercase();
        if ch.is_alphabetic() {
            let entry = self.weak_letters.entry(ch).or_insert(0);
            *entry = (*entry + 2).min(30);
        }

        if let Some(prev) = prev_char {
            let p = prev.to_ascii_lowercase();
            if p.is_alphabetic() && ch.is_alphabetic() {
                let bigram = format!("{}{}", p, ch);
                let entry = self.weak_bigrams.entry(bigram).or_insert(0);
                *entry = (*entry + 3).min(30);
            }
        }
    }

    /// Records a clean keystroke on a weak key or combination (decay / gaining proficiency)
    pub fn record_clean_keystroke(&mut self, ch: char, prev_char: Option<char>) {
        let c = ch.to_ascii_lowercase();
        if let Some(count) = self.weak_letters.get_mut(&c) {
            if *count > 0 {
                *count -= 1;
            }
        }
        self.weak_letters.retain(|_, count| *count > 0);

        if let Some(prev) = prev_char {
            let p = prev.to_ascii_lowercase();
            let bigram = format!("{}{}", p, c);
            if let Some(count) = self.weak_bigrams.get_mut(&bigram) {
                if *count > 0 {
                    *count -= 1;
                }
            }
            self.weak_bigrams.retain(|_, count| *count > 0);
        }
    }

    /// Returns top weak letters and weak bigrams sorted by mistake severity
    pub fn top_weaknesses(&self) -> (Vec<(char, u32)>, Vec<(String, u32)>) {
        let mut letters: Vec<(char, u32)> = self.weak_letters.iter().map(|(&c, &n)| (c, n)).collect();
        letters.sort_by(|a, b| b.1.cmp(&a.1));
        letters.truncate(4);

        let mut bigrams: Vec<(String, u32)> = self.weak_bigrams.iter().map(|(s, &n)| (s.clone(), n)).collect();
        bigrams.sort_by(|a, b| b.1.cmp(&a.1));
        bigrams.truncate(4);

        (letters, bigrams)
    }

    pub fn has_weaknesses(&self) -> bool {
        !self.weak_letters.is_empty() || !self.weak_bigrams.is_empty()
    }

    /// Records score and returns true if this is a new personal best (saves to disk)
    pub fn record_score(&mut self, duration: f32, wpm: f32) -> bool {
        let dur = duration as u32;
        let is_new = match dur {
            15 => {
                if wpm > self.pb_15 && wpm > 0.0 {
                    self.pb_15 = wpm;
                    true
                } else {
                    false
                }
            }
            30 => {
                if wpm > self.pb_30 && wpm > 0.0 {
                    self.pb_30 = wpm;
                    true
                } else {
                    false
                }
            }
            60 => {
                if wpm > self.pb_60 && wpm > 0.0 {
                    self.pb_60 = wpm;
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        // Always save updated PB and weakness scores to disk
        self.save();

        is_new
    }
}
