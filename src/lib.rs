// Allow missing docs for the library file
#![allow(missing_docs)]

/// Vole virtual machine
pub mod vole;

/// UI
mod ui;
pub use ui::VoleUI;

/// Storage key used for the app
pub const APP_KEY: &str = "vole";
