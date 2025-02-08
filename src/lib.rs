// Allow missing docs for the library file
#![allow(missing_docs)]

/// Vole virtual machine
pub mod vole;

/// Simple assembler
pub mod assembler;

/// UI
mod ui;
pub use ui::VoleUI;

/// Localization
mod localization;

/// Storage key used for the app
pub const APP_KEY: &str = "vole";
