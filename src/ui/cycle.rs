use strum_macros::EnumIter;

#[derive(PartialEq, Clone, Copy, EnumIter)]
pub enum CycleExecutionMode {
    FullSpeed,
    Timer(f32),
    Manual(bool),
}

impl CycleExecutionMode {
    pub const fn as_string(&self) -> &'static str {
        match self {
            CycleExecutionMode::FullSpeed => "Full Speed",
            CycleExecutionMode::Timer(_) => "Timer",
            CycleExecutionMode::Manual(_) => "Manual",
        }
    }
}
