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
    thocky_sound: Option<Sound>,
    clicky_sound: Option<Sound>,
    creamy_sound: Option<Sound>,
    bubble_sound: Option<Sound>,
    shatter_sound: Option<Sound>,
    combo_sound: Option<Sound>,
}

impl SoundEngine {
    pub async fn new() -> Self {
        let thocky_sound = load_sound_from_bytes(&generate_thocky_wav()).await.ok();
        let clicky_sound = load_sound_from_bytes(&generate_clicky_wav()).await.ok();
        let creamy_sound = load_sound_from_bytes(&generate_creamy_wav()).await.ok();
        let bubble_sound = load_sound_from_bytes(&generate_bubble_wav()).await.ok();
        let shatter_sound = load_sound_from_bytes(&generate_shatter_wav()).await.ok();
        let combo_sound = load_sound_from_bytes(&generate_combo_wav()).await.ok();

        Self {
            current_switch: SwitchSound::HolyPanda,
            thocky_sound,
            clicky_sound,
            creamy_sound,
            bubble_sound,
            shatter_sound,
            combo_sound,
        }
    }

    pub fn play_keystroke(&self) {
        if self.current_switch == SwitchSound::Mute {
            return;
        }

        let sound = match self.current_switch {
            SwitchSound::HolyPanda => self.thocky_sound.as_ref(),
            SwitchSound::ClickyBlue => self.clicky_sound.as_ref(),
            SwitchSound::CreamyLinear => self.creamy_sound.as_ref(),
            SwitchSound::BubblePop => self.bubble_sound.as_ref(),
            SwitchSound::Mute => None,
        };

        if let Some(snd) = sound {
            play_sound_once(snd);
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

/// Generates a deep, rich "Holy Panda" mechanical thock.
fn generate_thocky_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.075;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        // Fast initial impact transient (damped sine sweep 220Hz down to 140Hz)
        let decay = (-t * 60.0).exp();
        let freq = 220.0 - (t * 800.0).min(90.0);
        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let body = phase.sin();

        // Slight tactile tick at the very start (3kHz spike)
        let tick_decay = (-t * 220.0).exp();
        let tick = (2.0 * std::f32::consts::PI * 2800.0 * t).sin() * 0.3 * tick_decay;

        let sample = (body * decay * 0.85 + tick) * 24000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}

/// Generates a crisp, sharp "Clicky Blue" tactile switch.
fn generate_clicky_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.055;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        // Sharp high-pitch click (3600Hz) with fast exponential cutoff
        let click_decay = (-t * 140.0).exp();
        let click = (2.0 * std::f32::consts::PI * 3600.0 * t).sin() * click_decay * 0.7;

        // Followed immediately by subtle housing resonance at 320Hz
        let housing_decay = (-t * 70.0).exp();
        let housing = (2.0 * std::f32::consts::PI * 320.0 * t).sin() * housing_decay * 0.35;

        let sample = (click + housing) * 26000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}

/// Generates a soft, lubricated linear "Creamy" switch sound.
fn generate_creamy_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.065;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        // Smooth rounded bottom-out at 310Hz with soft attack
        let attack = (t * 800.0).min(1.0);
        let decay = (-t * 55.0).exp();
        let body = (2.0 * std::f32::consts::PI * 310.0 * t).sin() * attack * decay;

        let sample = body * 22000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}

/// Generates a calming zen water "Bubble Pop".
fn generate_bubble_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.080;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        // Frequency sweep up from 420Hz to 780Hz
        let freq = 420.0 + (t / duration_sec) * 360.0;
        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let envelope = (t * 600.0).min(1.0) * (-t * 45.0).exp();

        let sample = phase.sin() * envelope * 24000.0;
        samples.push(sample.clamp(-32767.0, 32767.0) as i16);
    }

    pcm_to_wav(&samples, sample_rate)
}

/// Generates an explosive, cathartic shatter sound when a stressor word is destroyed.
fn generate_shatter_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_sec = 0.16;
    let total_samples = (sample_rate as f32 * duration_sec) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        // Low impact boom + harmonic chord resonance
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
        // Two-stage arpeggio note
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
