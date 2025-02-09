use strum_macros::EnumIter;

/// The form the source code is being edited in
#[derive(PartialEq, Clone, Copy, EnumIter, serde::Deserialize, serde::Serialize)]
pub enum SourceEditMode {
    /// Each byte is being edited individually
    Byte,

    /// The instruction is being edited
    Instruction,

    /// Simple assembly is being edited
    Assembly,
}

impl SourceEditMode {
    pub const fn to_string(self) -> &'static str {
        match self {
            SourceEditMode::Byte => "Byte",
            SourceEditMode::Instruction => "Instruction",
            SourceEditMode::Assembly => "Assembly",
        }
    }
}
