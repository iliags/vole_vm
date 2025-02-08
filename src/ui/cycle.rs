use strum_macros::EnumIter;

#[derive(PartialEq, Clone, Copy, EnumIter, serde::Deserialize, serde::Serialize)]
pub enum CycleExecutionMode {
    FullSpeed,
    Timer(f32),
    Manual(bool),
}

impl CycleExecutionMode {
    pub const fn locale_key(&self) -> &'static str {
        match self {
            CycleExecutionMode::FullSpeed => "full_speed",
            CycleExecutionMode::Timer(_) => "timer",
            CycleExecutionMode::Manual(_) => "manual",
        }
    }
}
