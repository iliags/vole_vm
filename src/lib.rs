// Vole virtual machine
pub mod vole;

// UI
mod app;
pub use app::VoleUI;

/// Storage key used for the app
pub const APP_KEY: &str = "vole";
