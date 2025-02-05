use egui::scroll_area::ScrollBarVisibility;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use regex::Regex;
use strum::IntoEnumIterator;

use super::{rom::ROM, NumericDisplay, SourceEditMode};

const DEMO_ROM: &str = "; Load 0x00 into r0
0x20, 0x00,

; Load 0xFF into r5
0x25, 0xFF,

; Load mem 0x44 into r4
0x14, 0x44,

; If r4 == r0, jump to mem 0x0A (skip next line)
0xB4, 0x0A,

; Load 0x01 into r5
0x25, 0x01,

; Store r5 into mem 0x46
0x35, 0x46,

;Quit
0xC0, 0x00,";

const HEX_STR: &str = "^(0x|0X)?[a-fA-F0-9]+$";
const BINARY_STR: &str = "\\b(0b)?[01]+\\b";

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VoleUI {
    // Example stuff:
    label: String,

    #[serde(skip)]
    source_code: String,

    #[serde(skip)]
    source_edit_mode: SourceEditMode,

    #[serde(skip)]
    numeric_display: NumericDisplay,

    #[serde(skip)]
    rom: ROM,

    #[serde(skip)]
    active_cell_index: Option<usize>,

    #[serde(skip)]
    active_cell_string: String,

    #[serde(skip)]
    hex_regex: Regex,

    #[serde(skip)]
    binary_regex: Regex,
}

impl Default for VoleUI {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            source_code: DEMO_ROM.to_owned(),
            source_edit_mode: SourceEditMode::Byte,
            numeric_display: NumericDisplay::Hex,
            rom: ROM::new(),
            active_cell_index: None,
            active_cell_string: "".to_owned(),
            hex_regex: Regex::new(HEX_STR).unwrap(),
            binary_regex: Regex::new(BINARY_STR).unwrap(),
        }
    }
}

impl VoleUI {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // TODO: Add slider for global scaling
        //cc.egui_ctx.set_pixels_per_point(1.0);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, crate::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for VoleUI {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, crate::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // Github icon
                ui.hyperlink_to("\u{E624}", "https://github.com/iliags/vole_vm")
                    .on_hover_text("Link to GitHub");

                ui.separator();

                // TODO: Color theme menu
                egui::widgets::global_theme_preference_buttons(ui);

                ui.separator();

                let numeric = &mut self.numeric_display;
                for numerics in NumericDisplay::iter() {
                    ui.selectable_value(numeric, numerics.clone(), numerics.to_string());
                }
            });
        });

        /*
           Source code editing window
        */
        egui::Window::new("Program Source Code").show(ctx, |ui| {
            // Source edit mode selection
            egui::ComboBox::from_label("Edit mode")
                .selected_text(self.source_edit_mode.to_string())
                .show_ui(ui, |ui| {
                    let edit_mode = &mut self.source_edit_mode;
                    for mode in SourceEditMode::iter() {
                        ui.selectable_value(edit_mode, mode.clone(), mode.to_string());
                    }
                });

            ui.separator();

            // TODO: Add proper modes
            // Source code editor
            match self.source_edit_mode {
                SourceEditMode::Byte => {
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .auto_shrink(false)
                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                        .show(ui, |ui| {
                            egui::Grid::new("byte_grid")
                                .striped(true)
                                .num_columns(2)
                                .show(ui, |ui| {
                                    for (i, byte) in self.rom.bytes_mut().iter_mut().enumerate() {
                                        let byte_index = match self.numeric_display {
                                            NumericDisplay::Hex => format!("{:#X}", i),
                                            // Note: Rust counts the "0b" as part of the display length, hence the "010b",
                                            //  use "08b" if the prefix isn't visible.
                                            NumericDisplay::Binary => format!("{:#010b}", i),
                                        };
                                        ui.label(byte_index);

                                        let mut byte_string = if self
                                            .active_cell_index
                                            .is_some_and(|index| index == i)
                                        {
                                            self.active_cell_string.clone()
                                        } else {
                                            match self.numeric_display {
                                                NumericDisplay::Hex => format!("{:#X}", byte),
                                                // Note: Rust counts the "0b" as part of the display length, hence the "010b",
                                                //  use "08b" if the prefix isn't visible.
                                                NumericDisplay::Binary => format!("{:#010b}", byte),
                                            }
                                        };

                                        let response =
                                            ui.add(egui::TextEdit::singleline(&mut byte_string));

                                        if self.active_cell_index.is_some_and(|index| index == i) {
                                            if response.changed() {
                                                let within_length = match self.numeric_display {
                                                    NumericDisplay::Hex => byte_string.len() < 5,
                                                    NumericDisplay::Binary => {
                                                        byte_string.len() < 11
                                                    }
                                                };

                                                let valid_start = match self.numeric_display {
                                                    NumericDisplay::Hex => {
                                                        byte_string.starts_with("0x")
                                                    }
                                                    NumericDisplay::Binary => {
                                                        byte_string.starts_with("0b")
                                                    }
                                                };

                                                let valid_data = match self.numeric_display {
                                                    NumericDisplay::Hex => {
                                                        self.hex_regex.is_match(&byte_string)
                                                    }
                                                    NumericDisplay::Binary => {
                                                        self.binary_regex.is_match(&byte_string)
                                                    }
                                                };

                                                if within_length {
                                                    if valid_data || valid_start {
                                                        self.active_cell_string = byte_string;
                                                    }
                                                }
                                            } else if response.lost_focus() {
                                                let prefix = match self.numeric_display {
                                                    NumericDisplay::Hex => "0x",
                                                    NumericDisplay::Binary => "0b",
                                                };

                                                let result = i8::from_str_radix(
                                                    &byte_string.trim_start_matches(prefix),
                                                    16,
                                                );

                                                *byte = match result {
                                                    Ok(v) => v,
                                                    Err(_) => 0,
                                                }
                                            }
                                        } else if response.gained_focus() {
                                            self.active_cell_index = Some(i);
                                            self.active_cell_string = byte_string;
                                        }

                                        ui.end_row();
                                    }
                                });
                        });
                }
                SourceEditMode::Instruction => {
                    ui.label("Under Construction");
                }
                SourceEditMode::Assembly => {
                    #[cfg(debug_assertions)]
                    egui::ScrollArea::both().max_height(400.0).show(ui, |ui| {
                        CodeEditor::default()
                            .id_source("code editor")
                            .with_rows(12)
                            .with_fontsize(12.0)
                            .with_theme(ColorTheme::AYU_DARK)
                            .with_syntax(Syntax::vole())
                            .with_numlines(true)
                            .show(ui, &mut self.source_code);
                    });

                    #[cfg(not(debug_assertions))]
                    ui.label("Under Construction");
                }
            }

            ui.separator();

            if ui.button("Run").clicked() {
                // TODO: Run code
            }

            ui.separator();

            ui.collapsing("Output", |ui| {
                // TODO: Save as text file
                ui.label("Under construction");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Vole Virtual Machine");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |_ui| {
                //egui::warn_if_debug_build(ui);
            });
        });
    }
}
