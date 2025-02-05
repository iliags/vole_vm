// App UI
mod app;
pub use app::VoleUI;

// ROM
mod rom;

use strum_macros::EnumIter;

/// The form the source code is being edited in
#[derive(PartialEq, Clone, Copy, EnumIter)]
pub enum SourceEditMode {
    /// Each byte is being edited individually
    Byte,

    /// The instruction is being edited
    Instruction,

    /// Simple assembly is being edited
    Assembly,
}

impl SourceEditMode {
    pub fn to_string(&self) -> &str {
        match self {
            SourceEditMode::Byte => "Byte",
            SourceEditMode::Instruction => "Instruction",
            SourceEditMode::Assembly => "Assembly",
        }
    }
}
