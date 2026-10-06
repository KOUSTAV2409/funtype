use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SwitchSound {
    HolyPanda, // Thocky, deep mechanical resonance
    ClickyBlue, // Crisp, sharp tactile click
    CreamyLinear, // Smooth, lubricated linear pop
    BubblePop, // Resonant zen water bubble
    Mute,
}

impl SwitchSound {
    pub fn name(&self) -> &'static str {
        match self {
            SwitchSound::HolyPanda => "Thocky (Holy Panda)",
            SwitchSound::ClickyBlue => "Clicky (Blue)",
            SwitchSound::CreamyLinear => "Creamy Linear",
            SwitchSound::BubblePop => "Bubble Pop",
            SwitchSound::Mute => "Mute (Silent)",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            SwitchSound::HolyPanda => SwitchSound::ClickyBlue,
            SwitchSound::ClickyBlue => SwitchSound::CreamyLinear,
            SwitchSound::CreamyLinear => SwitchSound::BubblePop,
            SwitchSound::BubblePop => SwitchSound::Mute,
            SwitchSound::Mute => SwitchSound::HolyPanda,
        }
    }
}

pub struct SoundEngine {
    pub current_switch: SwitchSound,
    thocky_variations: Vec<Sound>,
    clicky_variations: Vec<Sound>,
    creamy_variations: Vec<Sound>,
    bubble_variations: Vec<Sound>,
    shatter_sound: Option<Sound>,
    combo_sound: Option<Sound>,
    var_idx: usize,
}

impl SoundEngine {
    pub async fn new() -> Self {
        let mut thocky_variations = Vec::new();
        for &freq_mod in &[1.0, 0.94, 1.06] {
            if let Ok(snd) = load_sound_from_bytes(&generate_thocky_wav(freq_mod)).await {
                thocky_variations.push(snd);
            }
        }

        let mut clicky_variations = Vec::new();
        for &freq_mod in &[1.0, 0.96, 1.04] {
            if let Ok(snd) = load_sound_from_bytes(&generate_clicky_wav(freq_mod)).await {
                clicky_variations.push(snd);
            }
        }

        let mut creamy_variations = Vec::new();
        for &freq_mod in &[1.0, 0.95, 1.05] {
            if let Ok(snd) = load_sound_from_bytes(&generate_creamy_wav(freq_mod)).await {
                creamy_variations.push(snd);
            }
        }

        let mut bubble_variations = Vec::new();
        for &freq_mod in &[1.0, 0.92, 1.08] {
            if let Ok(snd) = load_sound_from_bytes(&generate_bubble_wav(freq_mod)).await {
                bubble_variations.push(snd);
            }
        }

        let shatter_sound = load_sound_from_bytes(&generate_shatter_wav()).await.ok();
        let combo_sound = load_sound_from_bytes(&generate_combo_wav()).await.ok();

        Self {
            current_switch: SwitchSound::HolyPanda,
            thocky_variations,
            clicky_variations,
            creamy_variations,
            bubble_variations,
            shatter_sound,
            combo_sound,
            var_idx: 0,
        }
    }

    pub fn play_keystroke(&mut self) {
        if self.current_switch == SwitchSound::Mute {
            return;
        }

        let variations = match self.current_switch {
            SwitchSound::HolyPanda => &self.thocky_variations,
            SwitchSound::ClickyBlue => &self.clicky_variations,
            SwitchSound::CreamyLinear => &self.creamy_variations,
            SwitchSound::BubblePop => &self.bubble_variations,
            SwitchSound::Mute => return,
        };

        if !variations.is_empty() {
            let snd = &variations[self.var_idx % variations.len()];
            play_sound_once(snd);
            self.var_idx = (self.var_idx + 1) % variations.len();
        }
    }

    pub fn play_shatter(&self) {
        if self.current_switch != SwitchSound::Mute {
            if let Some(snd) = &self.shatter_sound {
                play_sound_once(snd);
            }
        }
    }

    pub fn play_combo(&self) {
        if self.current_switch != SwitchSound::Mute {
            if let Some(snd) = &self.combo_sound {
                play_sound_once(snd);
            }
        }
    }
}

/// Creates a standard 44.1kHz 16-bit Mono PCM WAV byte vector from raw audio samples.
fn pcm_to_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let num_samples = samples.len() as u32;
    let data_bytes = num_samples * 2;
    let file_size = 36 + data_bytes;

    let mut wav = Vec::with_capacity(44 + data_bytes as usize);
    // RIFF chunk
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // "fmt " sub-chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size = 16 for PCM
    wav.extend_from_slice(&1u16.to_le_bytes());  // AudioFormat = 1 (PCM)
    wav.extend_from_slice(&1u16.to_le_bytes());  // NumChannels = 1 (Mono)
    wav.extend_from_slice(&sample_rate.to_le_bytes()); // SampleRate
    let byte_rate = sample_rate * 2;
    wav.extend_from_slice(&byte_rate.to_le_bytes());   // ByteRate = SampleRate * NumChannels * BitsPerSample/8
    wav.extend_from_slice(&2u16.to_le_bytes());  // BlockAlign = 2
    wav.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample = 16

    // "data" sub-chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_bytes.to_le_bytes());

    // PCM Data
    for &sample in samples {
        wav.extend_from_slice(&sample.to_le_bytes());
    }

    wav
}

/// Generates a deep, rich "Holy Panda" mechanical thock with frequency modulation.
fn generate_thocky_wav(freq_mod: f32) -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.075;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let decay = (-t * 60.0).exp();
        let freq = (220.0 - (t * 800.0).min(90.0)) * freq_mod;
        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let body = phase.sin();

        let tick_decay = (-t * 220.0).exp();
        let tick = (2.0 * std::f32::consts::PI * (2800.0 * freq_mod) * t).sin() * 0.3 * tick_decay;

        let sample = (body * decay * 0.85 + tick) * 24000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}

/// Generates a crisp, sharp "Clicky Blue" tactile switch with frequency modulation.
fn generate_clicky_wav(freq_mod: f32) -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.055;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let click_decay = (-t * 140.0).exp();
        let click = (2.0 * std::f32::consts::PI * (3600.0 * freq_mod) * t).sin() * click_decay * 0.7;

        let housing_decay = (-t * 70.0).exp();
        let housing = (2.0 * std::f32::consts::PI * (320.0 * freq_mod) * t).sin() * housing_decay * 0.35;

        let sample = (click + housing) * 26000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}

/// Generates a soft, lubricated linear "Creamy" switch sound with frequency modulation.
fn generate_creamy_wav(freq_mod: f32) -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.065;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let attack = (t * 800.0).min(1.0);
        let decay = (-t * 55.0).exp();
        let body = (2.0 * std::f32::consts::PI * (310.0 * freq_mod) * t).sin() * attack * decay;

        let sample = body * 22000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}

/// Generates a calming zen water "Bubble Pop" with frequency modulation.
fn generate_bubble_wav(freq_mod: f32) -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.080;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = (420.0 + (t / duration_sec) * 360.0) * freq_mod;
        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let envelope = (t * 600.0).min(1.0) * (-t * 45.0).exp();

        let sample = phase.sin() * envelope * 24000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}

/// Generates an explosive shatter sound when a stressor word is destroyed.
fn generate_shatter_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.16;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let bass = (2.0 * std::f32::consts::PI * 110.0 * t).sin() * (-t * 22.0).exp();
        let chord1 = (2.0 * std::f32::consts::PI * 520.0 * t).sin() * (-t * 30.0).exp();
        let chord2 = (2.0 * std::f32::consts::PI * 780.0 * t).sin() * (-t * 35.0).exp();
        let shimmer = (2.0 * std::f32::consts::PI * 1320.0 * t).sin() * (-t * 40.0).exp();

        let sample = (bass * 0.6 + chord1 * 0.25 + chord2 * 0.2 + shimmer * 0.15) * 27000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}

/// Generates an ascending combo chime.
fn generate_combo_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.22;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = if t < 0.08 { 587.33 } else { 880.0 }; // D5 -> A5
        let env = if t < 0.08 {
            (-t * 20.0).exp()
        } else {
            (-(t - 0.08) * 16.0).exp()
        };

        let sample = (2.0 * std::f32::consts::PI * freq * t).sin() * env * 22000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}
