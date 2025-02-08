use strum_macros::EnumIter;

#[derive(PartialEq, Clone, Copy, EnumIter, serde::Deserialize, serde::Serialize)]
pub enum NumericDisplay {
    Hex,
    Binary,
}

impl NumericDisplay {
    /// Returns the locale key of the numeric display
    pub const fn locale_key(&self) -> &'static str {
        match self {
            NumericDisplay::Hex => "hex",
            NumericDisplay::Binary => "binary",
        }
    }

    /// Returns the string prefix of the current numeric representation
    pub const fn prefix(&self) -> &'static str {
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
    pub const fn radix(&self) -> u32 {
        match self {
            NumericDisplay::Hex => 16,
            NumericDisplay::Binary => 2,
        }
    }
}
