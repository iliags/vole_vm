// App UI
mod app;
pub use app::VoleUI;

// ROM
mod rom;

// Help text
mod help;

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
    pub fn to_string(self) -> &'static str {
        match self {
            SourceEditMode::Byte => "Byte",
            SourceEditMode::Instruction => "Instruction",
            SourceEditMode::Assembly => "Assembly",
        }
    }
}

#[derive(PartialEq, Clone, Copy, EnumIter)]
pub enum NumericDisplay {
    Hex,
    Binary,
}

impl NumericDisplay {
    /// Returns the string of the numeric display
    pub fn to_string(self) -> &'static str {
        match self {
            NumericDisplay::Hex => "Hex",
            NumericDisplay::Binary => "Binary",
        }
    }

    /// Returns the string prefix of the current numeric representation
    pub fn prefix(&self) -> &'static str {
        match self {
            NumericDisplay::Hex => "0x",
            NumericDisplay::Binary => "0b",
        }
    }

    /// Converts the given byte into a binary or hex string
    pub fn bit_string(&self, byte: u8) -> String {
        match self {
            NumericDisplay::Hex => format!("0x{:01X}", byte),
            // Note: Rust counts the "0b" as part of the display length, hence the "006b",
            //  use "08b" if the prefix isn't visible.
            NumericDisplay::Binary => format!("{:#06b}", byte),
        }
    }

    /// Converts the given byte into a binary or hex string
    pub fn byte_string(&self, byte: u8) -> String {
        match self {
            NumericDisplay::Hex => format!("0x{:02X}", byte),
            NumericDisplay::Binary => format!("{:#010b}", byte),
        }
    }

    /// Converts the given instruction into a binary or hex string
    pub fn instruction_string(&self, byte: u16) -> String {
        match self {
            NumericDisplay::Hex => format!("0x{:04X}", byte),
            NumericDisplay::Binary => format!("{:#020b}", byte),
        }
    }

    /// Returns the radix for the current numeric display.
    ///
    /// Used for converting string numerics into types
    pub fn radix(&self) -> u32 {
        match self {
            NumericDisplay::Hex => 16,
            NumericDisplay::Binary => 2,
        }
    }
}
