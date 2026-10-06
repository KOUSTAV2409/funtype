use std::fs;
use std::path::PathBuf;

pub struct UserStats {
    pub pb_15: f32,
    pub pb_30: f32,
    pub pb_60: f32,
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
        };

        if let Some(path) = Self::stats_file_path() {
            if let Ok(content) = fs::read_to_string(path) {
                for line in content.lines() {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();
                        let val: f32 = parts[1].trim().parse().unwrap_or(0.0);
                        match key {
                            "pb_15" => stats.pb_15 = val,
                            "pb_30" => stats.pb_30 = val,
                            "pb_60" => stats.pb_60 = val,
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
            let content = format!(
                "pb_15={:.1}\npb_30={:.1}\npb_60={:.1}\n",
                self.pb_15, self.pb_30, self.pb_60
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

        if is_new {
            self.save();
        }

        is_new
    }
}
