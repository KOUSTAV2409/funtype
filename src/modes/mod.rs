pub mod stress_shredder;
pub mod zen_flow;
pub mod speed_sprint;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameMode {
    StressShredder,
    ZenFlow,
    SpeedSprint,
}

#[allow(dead_code)]
impl GameMode {
    pub fn name(&self) -> &'static str {
        match self {
            GameMode::StressShredder => "Stress Shredder",
            GameMode::ZenFlow => "Zen Flow",
            GameMode::SpeedSprint => "Speed Sprint",
        }
    }
}
