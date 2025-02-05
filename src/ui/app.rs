use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use strum::IntoEnumIterator;

use super::SourceEditMode;

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

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for VoleUI {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            source_code: DEMO_ROM.to_owned(),
            source_edit_mode: SourceEditMode::Instruction,
            value: 2.7,
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
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // Github icon
                ui.hyperlink_to("\u{E624}", "https://github.com/iliags/vole_vm")
                    .on_hover_text("Link to GitHub");

                ui.separator();

                // TODO: Color theme menu
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        /*
           Source code editing window
        */
        egui::Window::new("Program Source Code").show(ctx, |ui| {
            // Source edit mode selection
            egui::ComboBox::from_label("")
                .selected_text(self.source_edit_mode.to_string())
                .show_ui(ui, |ui| {
                    let edit_mode = &mut self.source_edit_mode;

                    for mode in SourceEditMode::iter() {
                        ui.selectable_value(edit_mode, mode.clone(), mode.to_string());
                    }
                });

            // TODO: Add proper modes
            // Source code editor
            match self.source_edit_mode {
                SourceEditMode::Byte => {
                    ui.label("Under Construction");
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

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |_ui| {
                //egui::warn_if_debug_build(ui);
            });
        });
    }
}
