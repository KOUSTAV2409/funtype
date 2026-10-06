use macroquad::prelude::*;

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub size: f32,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
}

pub struct FloatingText {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
    pub font_size: f32,
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    floating_texts: Vec<FloatingText>,
    pub trauma: f32,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(300),
            floating_texts: Vec::with_capacity(30),
            trauma: 0.0,
        }
    }

    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }

    pub fn get_shake_offset(&self) -> (f32, f32) {
        if self.trauma <= 0.001 {
            return (0.0, 0.0);
        }
        let shake = self.trauma * self.trauma;
        let max_offset = 12.0;
        let ox = rand::gen_range(-1.0, 1.0) * max_offset * shake;
        let oy = rand::gen_range(-1.0, 1.0) * max_offset * shake;
        (ox, oy)
    }

    pub fn spawn_keystroke_spark(&mut self, x: f32, y: f32, color: Color) {
        for _ in 0..4 {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(20.0, 90.0);
            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed - 15.0,
                size: rand::gen_range(2.0, 4.0),
                color,
                life: 0.35,
                max_life: 0.35,
            });
        }
    }

    pub fn spawn_burst(&mut self, x: f32, y: f32, color: Color, count: usize) {
        self.add_trauma(0.35);
        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(60.0, 320.0);
            let life = rand::gen_range(0.5, 1.1);
            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                size: rand::gen_range(3.0, 7.0),
                color,
                life,
                max_life: life,
            });
        }
    }

    pub fn spawn_floating_text(&mut self, text: &str, x: f32, y: f32, color: Color, font_size: f32) {
        self.floating_texts.push(FloatingText {
            text: text.to_string(),
            x,
            y,
            vy: -40.0,
            color,
            life: 1.2,
            max_life: 1.2,
            font_size,
        });
    }

    pub fn update(&mut self, dt: f32) {
        // Decay screen shake
        if self.trauma > 0.0 {
            self.trauma = (self.trauma - dt * 1.8).max(0.0);
        }

        // Update particles
        let mut i = 0;
        while i < self.particles.len() {
            let p = &mut self.particles[i];
            p.life -= dt;
            if p.life <= 0.0 {
                self.particles.swap_remove(i);
            } else {
                p.x += p.vx * dt;
                p.y += p.vy * dt;
                p.vy += 120.0 * dt; // subtle gravity
                p.vx *= 0.96;       // drag
                p.vy *= 0.96;
                i += 1;
            }
        }

        // Update floating texts
        let mut j = 0;
        while j < self.floating_texts.len() {
            let ft = &mut self.floating_texts[j];
            ft.life -= dt;
            if ft.life <= 0.0 {
                self.floating_texts.swap_remove(j);
            } else {
                ft.y += ft.vy * dt;
                ft.vy *= 0.95;
                j += 1;
            }
        }
    }

    pub fn draw(&self, offset_x: f32, offset_y: f32) {
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut col = p.color;
            col.a = alpha;
            let current_size = p.size * alpha;
            draw_circle(p.x + offset_x, p.y + offset_y, current_size, col);
        }

        for ft in &self.floating_texts {
            let alpha = (ft.life / ft.max_life).clamp(0.0, 1.0);
            let mut col = ft.color;
            col.a = alpha;
            let text_dims = measure_text(&ft.text, None, ft.font_size as u16, 1.0);
            let draw_x = ft.x - text_dims.width * 0.5 + offset_x;
            let draw_y = ft.y + offset_y;
            draw_text(&ft.text, draw_x, draw_y, ft.font_size, col);
        }
    }
}
