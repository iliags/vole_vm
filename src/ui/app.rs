use egui::{
    containers::Frame, emath, epaint, epaint::PathStroke, hex_color, lerp, pos2, remap,
    scroll_area::ScrollBarVisibility, vec2, Color32, Pos2, Rect,
};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use regex::Regex;
use strum::IntoEnumIterator;

use crate::vole::Vole;

use super::{rom::Rom, NumericDisplay, SourceEditMode};

/*
const DEMO_SOURCE: &str = "; Load 0x00 into r0
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
 */

const DEMO_SOURCE: &str = "
ld r0, 0x00        ; Load 0x00 into r0
ld r5, 0xFF        ; Load 0xFF into r5
ld r4, (0x44)      ; Load mem 0x44 into r4

jp r4, continue    ; If r4 == r0, jump to continue
ld r5, 0x01        ; Load 0x01 into r5

continue:
    ld (0x46), r5  ; Store r5 into mem 0x46
    halt           ; Quit";

const DEMO_ROM: &[u8] = &[
    0x20, 0x00, 0x25, 0xFF, 0x14, 0x44, 0xB4, 0x0A, 0x35, 0x46, 0xC0, 0x00,
];

const HEX_STR: &str = "^(0x|0X)?[a-fA-F0-9]+$";
const BINARY_STR: &str = "\\b(0b)?[01]+\\b";

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VoleUI {
    // TODO: Remove skip
    #[serde(skip)]
    source_code: String,

    // TODO: Remove skip
    #[serde(skip)]
    source_edit_mode: SourceEditMode,

    // TODO: Remove skip
    #[serde(skip)]
    numeric_display: NumericDisplay,

    // TODO: Remove skip
    #[serde(skip)]
    rom: Rom,

    // TODO: Remove skip
    #[serde(skip)]
    execution_speed: usize,

    #[serde(skip)]
    active_cell_index: Option<usize>,

    #[serde(skip)]
    active_cell_string: String,

    #[serde(skip)]
    hex_regex: Regex,

    #[serde(skip)]
    binary_regex: Regex,

    #[serde(skip)]
    vole: Vole,
}

impl Default for VoleUI {
    fn default() -> Self {
        // TODO: Remove this
        let mut new_rom = Rom::new();
        new_rom.bytes_mut()[0..DEMO_ROM.len()].copy_from_slice(DEMO_ROM);

        Self {
            source_code: DEMO_SOURCE.to_owned(),
            source_edit_mode: SourceEditMode::Byte,
            numeric_display: NumericDisplay::Hex,
            // TODO: Default
            //rom: Rom::new(),
            rom: new_rom,
            execution_speed: 10,
            active_cell_index: None,
            active_cell_string: "".to_owned(),
            hex_regex: Regex::new(HEX_STR).unwrap(),
            binary_regex: Regex::new(BINARY_STR).unwrap(),
            vole: Vole::new(),
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
        // TODO: Add cycle speed
        if self.vole.running() {
            self.vole.cycle();
            // TODO: Spin up background thread instead of relying on egui update
            ctx.request_repaint();
        }

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
                    ui.selectable_value(numeric, numerics, numerics.to_string());
                }
            });
        });

        /*
           Source code editing panel
        */
        egui::SidePanel::left("Source Panel").show(ctx, |ui| {
            ui.heading("Program Source Code");
            // Source edit mode selection
            egui::ComboBox::from_label("Edit mode")
                .selected_text(self.source_edit_mode.to_string())
                .show_ui(ui, |ui| {
                    let edit_mode = &mut self.source_edit_mode;
                    for mode in SourceEditMode::iter() {
                        ui.selectable_value(edit_mode, mode, mode.to_string());
                    }
                });

            if ui.button("Load Demo").clicked() {
                self.rom.bytes_mut()[0..DEMO_ROM.len()].copy_from_slice(DEMO_ROM);
            }

            ui.separator();

            // TODO: Add proper modes
            // Source code editor
            match self.source_edit_mode {
                SourceEditMode::Byte => {
                    egui::ScrollArea::vertical()
                        .max_height(400.0)
                        .auto_shrink(false)
                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                        .show(ui, |ui| {
                            egui::Grid::new("byte_grid")
                                .striped(true)
                                .num_columns(2)
                                .show(ui, |ui| {
                                    for (i, byte) in self.rom.bytes_mut().iter_mut().enumerate() {
                                        if i == 0 {
                                            ui.label("Address");
                                            ui.label("Contents");
                                            ui.end_row();
                                        }

                                        let byte_index = self.numeric_display.byte_string(i as u8);
                                        ui.label(byte_index);

                                        let mut byte_string = if self
                                            .active_cell_index
                                            .is_some_and(|index| index == i)
                                        {
                                            self.active_cell_string.clone()
                                        } else {
                                            self.numeric_display.byte_string(*byte)
                                        };

                                        let response =
                                            ui.add(egui::TextEdit::singleline(&mut byte_string));

                                        if self.active_cell_index.is_some_and(|index| index == i) {
                                            let prefix = self.numeric_display.prefix();

                                            if response.changed() {
                                                let within_length = match self.numeric_display {
                                                    NumericDisplay::Hex => byte_string.len() < 5,
                                                    NumericDisplay::Binary => {
                                                        byte_string.len() < 11
                                                    }
                                                };

                                                let valid_start = byte_string.starts_with(prefix);

                                                let valid_data = match self.numeric_display {
                                                    NumericDisplay::Hex => {
                                                        self.hex_regex.is_match(&byte_string)
                                                    }
                                                    NumericDisplay::Binary => {
                                                        self.binary_regex.is_match(&byte_string)
                                                    }
                                                };

                                                if within_length && (valid_data || valid_start) {
                                                    self.active_cell_string = byte_string;
                                                }
                                            } else if response.lost_focus() {
                                                let radix = match self.numeric_display {
                                                    NumericDisplay::Hex => 16,
                                                    NumericDisplay::Binary => 2,
                                                };

                                                let result = u8::from_str_radix(
                                                    byte_string.trim_start_matches(prefix),
                                                    radix,
                                                );

                                                *byte = result.unwrap_or(0);
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
                    {
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

                        if ui.button("Compile").clicked() {
                            // TODO: Compile source code into bytes
                        }
                    }

                    #[cfg(not(debug_assertions))]
                    ui.label("Under Construction");
                }
            }
            ui.separator();

            /*
            ui.collapsing("Export", |ui| {
                // TODO: Save as text file
                ui.label("Under construction");
            });
            ui.separator();
             */

            ui.heading("Execution");
            // TODO Add grid to fill horizontal space
            ui.horizontal(|ui| {
                if ui.button("Run").clicked() {
                    self.vole.load_rom(self.rom.bytes());
                    self.vole.start();
                }

                /*
                if ui.button("Run Stepped").clicked() {
                    // TODO: Pause after each step of the cycle
                    self.vole.load_rom(self.rom.bytes());
                    self.vole.start();
                }
                 */
            });

            ui.separator();

            ui.add(egui::Slider::new(&mut self.execution_speed, 0..=100).text("Execution Speed"))
                .on_hover_text("The number of cycles to execute per second.");

            ui.separator();
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            /*
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Vole Virtual Machine");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |_ui| {
                //egui::warn_if_debug_build(ui);
            });
             */

            Frame::canvas(ui.style()).show(ui, |ui| {
                let color = if ui.visuals().dark_mode {
                    Color32::from_additive_luminance(196)
                } else {
                    Color32::from_black_alpha(240)
                };

                ui.ctx().request_repaint();
                let time = ui.input(|i| i.time);

                let desired_size = ui.available_width() * vec2(1.0, 0.5);
                let (_id, rect) = ui.allocate_space(desired_size);

                let to_screen = emath::RectTransform::from_to(
                    Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0),
                    rect,
                );

                let mut shapes = vec![];

                for &mode in &[2, 3, 5] {
                    let mode = mode as f64;
                    let n = 120;
                    let speed = 1.5;

                    let points: Vec<Pos2> = (0..=n)
                        .map(|i| {
                            let t = i as f64 / (n as f64);
                            let amp = (time * speed * mode).sin() / mode;
                            let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                            to_screen * pos2(t as f32, y as f32)
                        })
                        .collect();

                    let thickness = 10.0 / mode as f32;
                    shapes.push(epaint::Shape::line(
                        points,
                        if true {
                            PathStroke::new_uv(thickness, move |rect, p| {
                                let t = remap(p.x, rect.x_range(), -1.0..=1.0).abs();
                                let center_color = hex_color!("#5BCEFA");
                                let outer_color = hex_color!("#F5A9B8");

                                Color32::from_rgb(
                                    lerp(center_color.r() as f32..=outer_color.r() as f32, t) as u8,
                                    lerp(center_color.g() as f32..=outer_color.g() as f32, t) as u8,
                                    lerp(center_color.b() as f32..=outer_color.b() as f32, t) as u8,
                                )
                            })
                        } else {
                            PathStroke::new(thickness, color)
                        },
                    ));
                }

                ui.painter().extend(shapes);
            });
        });
    }
}
