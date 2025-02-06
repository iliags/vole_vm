use super::{rom::Rom, NumericDisplay, SourceEditMode};
use crate::vole::{StartMode, Vole};
use egui::{scroll_area::ScrollBarVisibility, Vec2};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use regex::Regex;
use strum::IntoEnumIterator;

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

const DEMO_SOURCE: &str = "ld r0, 0x00        ; Load 0x00 into r0
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

// TODO: Separate execution options from UI
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

    #[serde(skip)]
    show_export: bool,

    #[serde(skip)]
    cycle_timer: f32,

    #[serde(skip)]
    cycle_speed: usize,
}

impl Default for VoleUI {
    fn default() -> Self {
        // TODO: Remove this
        let mut new_rom = Rom::new();
        new_rom.bytes_mut()[0..DEMO_ROM.len()].copy_from_slice(DEMO_ROM);

        Self {
            source_code: DEMO_SOURCE.to_owned(),
            source_edit_mode: SourceEditMode::Instruction,
            numeric_display: NumericDisplay::Hex,
            // TODO: Default
            //rom: Rom::new(),
            rom: new_rom,
            execution_speed: 1,
            active_cell_index: None,
            active_cell_string: "".to_owned(),
            hex_regex: Regex::new(HEX_STR).unwrap(),
            binary_regex: Regex::new(BINARY_STR).unwrap(),
            vole: Vole::new(),
            show_export: false,
            cycle_timer: 0.0,
            cycle_speed: 0,
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
            // TODO: Use actual delta time
            self.cycle_timer += 1.0 / 60.0;
            //let limit = self.cycle_speed * 1000;

            if self.cycle_timer >= self.cycle_speed as f32 {
                println!("Cycle");
                self.cycle_timer = 0.0;
                self.vole.cycle();
            }

            // TODO: Spin up background thread instead of relying on egui update
            ctx.request_repaint();
        }

        /*
            Menu bar at the top
        */
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
                    let response = ui.selectable_value(numeric, numerics, numerics.to_string());

                    if response.clicked() {
                        response.request_focus();
                    }
                }
            });
        });

        /*
           Source code editing panel
        */
        egui::SidePanel::left("Source Panel").show(ctx, |ui| {
            egui::ScrollArea::vertical()
                //.max_height(400.0)
                //.auto_shrink(false)
                //.scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
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
                        let rom: Vec<u8> = vec![0; 256];
                        self.rom.bytes_mut().copy_from_slice(&rom);
                        self.rom.bytes_mut()[0..DEMO_ROM.len()].copy_from_slice(DEMO_ROM);
                    }

                    ui.separator();

                    // TODO: Add proper modes
                    // Source code editor
                    match self.source_edit_mode {
                        // TODO: Merge with instruction mode
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
                                            for (i, byte) in
                                                self.rom.bytes_mut().iter_mut().enumerate()
                                            {
                                                if i == 0 {
                                                    ui.label("Address");
                                                    ui.label("Contents");
                                                    ui.end_row();
                                                }

                                                let byte_index =
                                                    self.numeric_display.byte_string(i as u8);
                                                ui.label(byte_index);

                                                let mut byte_string = if self
                                                    .active_cell_index
                                                    .is_some_and(|index| index == i)
                                                {
                                                    self.active_cell_string.clone()
                                                } else {
                                                    self.numeric_display.byte_string(*byte)
                                                };

                                                let response = ui.add(egui::TextEdit::singleline(
                                                    &mut byte_string,
                                                ));

                                                if self
                                                    .active_cell_index
                                                    .is_some_and(|index| index == i)
                                                {
                                                    let prefix = self.numeric_display.prefix();

                                                    if response.changed() {
                                                        let within_length =
                                                            match self.numeric_display {
                                                                NumericDisplay::Hex => {
                                                                    byte_string.len() < 5
                                                                }
                                                                NumericDisplay::Binary => {
                                                                    byte_string.len() < 11
                                                                }
                                                            };

                                                        let valid_start =
                                                            byte_string.starts_with(prefix);

                                                        let valid_data = match self.numeric_display
                                                        {
                                                            NumericDisplay::Hex => self
                                                                .hex_regex
                                                                .is_match(&byte_string),
                                                            NumericDisplay::Binary => self
                                                                .binary_regex
                                                                .is_match(&byte_string),
                                                        };

                                                        if within_length
                                                            && (valid_data || valid_start)
                                                        {
                                                            self.active_cell_string = byte_string;
                                                        }
                                                    } else if response.lost_focus() {
                                                        *byte = u8::from_str_radix(
                                                            byte_string.trim_start_matches(prefix),
                                                            self.numeric_display.radix(),
                                                        )
                                                        .unwrap_or(0);
                                                        self.active_cell_index = None;
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
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .auto_shrink(false)
                                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                                .show(ui, |ui| {
                                    egui::Grid::new("instruction_grid")
                                        .striped(true)
                                        .num_columns(2)
                                        .show(ui, |ui| {
                                            for (i, chunk) in
                                                self.rom.bytes_mut().chunks_mut(2).enumerate()
                                            {
                                                if i == 0 {
                                                    ui.label("Address");
                                                    ui.label("Contents");
                                                    ui.end_row();
                                                }
                                                let start_byte =
                                                    self.numeric_display.byte_string((i * 2) as u8);
                                                let end_byte = self
                                                    .numeric_display
                                                    .byte_string((i * 2 + 1) as u8);

                                                ui.label(format!("{}-{}", start_byte, end_byte));

                                                let mut byte_string = if self
                                                    .active_cell_index
                                                    .is_some_and(|index| index == i)
                                                {
                                                    self.active_cell_string.clone()
                                                } else {
                                                    self.numeric_display.instruction_string(
                                                        ((chunk[0] as u16) << 8) | chunk[1] as u16,
                                                    )
                                                };

                                                let response = ui.add(egui::TextEdit::singleline(
                                                    &mut byte_string,
                                                ));

                                                if self
                                                    .active_cell_index
                                                    .is_some_and(|index| index == i)
                                                {
                                                    let prefix = self.numeric_display.prefix();

                                                    if response.changed() {
                                                        let within_length =
                                                            match self.numeric_display {
                                                                NumericDisplay::Hex => {
                                                                    byte_string.len() < 7
                                                                }
                                                                NumericDisplay::Binary => {
                                                                    byte_string.len() < 21
                                                                }
                                                            };

                                                        let valid_start =
                                                            byte_string.starts_with(prefix);

                                                        let valid_data = match self.numeric_display
                                                        {
                                                            NumericDisplay::Hex => self
                                                                .hex_regex
                                                                .is_match(&byte_string),
                                                            NumericDisplay::Binary => self
                                                                .binary_regex
                                                                .is_match(&byte_string),
                                                        };

                                                        if within_length
                                                            && (valid_data || valid_start)
                                                        {
                                                            self.active_cell_string = byte_string;
                                                        }
                                                    } else if response.lost_focus() {
                                                        let radix = self.numeric_display.radix();

                                                        let opcode = byte_string
                                                            .strip_prefix("0x")
                                                            .unwrap_or_default();

                                                        if let Some((lhs, rhs)) =
                                                            opcode.split_at_checked(2)
                                                        {
                                                            chunk[0] =
                                                                u8::from_str_radix(lhs, radix)
                                                                    .unwrap_or(0);

                                                            chunk[1] =
                                                                u8::from_str_radix(rhs, radix)
                                                                    .unwrap_or(0);
                                                        }

                                                        self.active_cell_index = None;
                                                        //println!("0 {:#X}, 1 {:#X}", chunk[0], chunk[1]);
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

                    if ui.button("Export").clicked() {
                        self.show_export = true;
                    }

                    ui.separator();

                    ui.heading("Execution");

                    ui.add(
                        egui::Slider::new(&mut self.execution_speed, 1..=10)
                            .text("Execution Speed"),
                    )
                    .on_hover_text("The number of seconds it takes to execute one cycle.");

                    #[cfg(debug_assertions)]
                    {
                        let mut text = "0x00";
                        let label = ui.label("Program Counter Start");
                        ui.text_edit_singleline(&mut text).labelled_by(label.id);
                    }

                    ui.separator();

                    // TODO Add grid to fill horizontal space
                    ui.horizontal(|ui| {
                        if ui
                            .button("Run at full speed")
                            .on_hover_text("The CPU cycles around 60 times per second.")
                            .clicked()
                        {
                            self.vole.load_rom(self.rom.bytes());
                            self.vole.start(StartMode::Reset);
                            self.cycle_speed = 0;
                            self.cycle_timer = 0.0;
                        }

                        if ui
                            .button("Run at execution speed")
                            .on_hover_text("Executes the program at the execution speed.")
                            .clicked()
                        {
                            self.vole.load_rom(self.rom.bytes());
                            self.vole.start(StartMode::Reset);
                            self.cycle_speed = self.execution_speed.max(1);
                            self.cycle_timer = self.execution_speed as f32;
                        }

                        #[cfg(debug_assertions)]
                        if ui
                            .button("Run in steps")
                            .on_hover_text("Each cycle needs to be manually advanced.")
                            .clicked()
                        {
                            // TODO: Pause after each step of the cycle
                            self.vole.load_rom(self.rom.bytes());
                            self.vole.start(StartMode::Reset);
                        }
                    });
                });
        });

        /*
            Export program window
        */
        egui::Window::new("Export")
            .open(&mut self.show_export)
            .show(ctx, |ui| {
                // Constructing the output string here for copying to the clipboard feature
                let output_string = match self.source_edit_mode {
                    SourceEditMode::Byte | SourceEditMode::Instruction => {
                        // TODO: Output types
                        let mut output: String = "".to_owned();
                        for (i, chunk) in self.rom.bytes_mut().chunks_mut(2).enumerate() {
                            if i == 0 {
                                output.push_str("Address         Contents\n");
                            }

                            if chunk[0] == 0 && chunk[1] == 0 {
                                continue;
                            }

                            let a0 = self.numeric_display.byte_string((i * 2) as u8);
                            let a1 = self.numeric_display.byte_string((i * 2 + 1) as u8);
                            let row_string = if self.source_edit_mode == SourceEditMode::Byte {
                                let b0 = self.numeric_display.byte_string(chunk[0]);
                                let b1 = self.numeric_display.byte_string(chunk[1]);
                                let spacing = "            ";
                                format!("{}{}{}\n{}{}{}\n", a0, spacing, b0, a1, spacing, b1)
                            } else {
                                let a = format!("{}-{}", a0, a1);
                                let b = self
                                    .numeric_display
                                    .instruction_string(((chunk[0] as u16) << 8) | chunk[1] as u16);
                                format!("{}       {}\n", a, b)
                            };
                            output.push_str(&row_string);
                        }
                        output
                    }
                    SourceEditMode::Assembly => self.source_code.clone(),
                };

                // TODO: Output types
                ui.label("Under construction");
                if ui.button("Copy to Clipboard").clicked() {
                    ctx.copy_text(output_string.clone());
                }
                ui.separator();

                ui.label(output_string);
            });

        /*
           Visualizer panel
        */
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading("Registers");
                egui::Grid::new("registers")
                    .num_columns(4)
                    //.max_col_width(10.0)
                    .spacing(Vec2::new(10.0, 3.0))
                    .min_col_width(4.0)
                    .show(ui, |ui| {
                        for (i, chunks) in self.vole.registers().chunks(4).enumerate() {
                            for (r, chunk) in chunks.iter().enumerate() {
                                ui.group(|ui| {
                                    let register_text = format!(
                                        "{}",
                                        self.numeric_display.bit_string((r + (i * 4)) as u8)
                                    );
                                    let label = ui.label(register_text.clone());

                                    let mut register = *chunk;

                                    if self.numeric_display == NumericDisplay::Binary {
                                        ui.add(
                                            egui::DragValue::new(&mut register)
                                                .binary(8, false)
                                                .prefix("0b"),
                                        )
                                        //.on_hover_text(register_text)
                                        .labelled_by(label.id);
                                    } else {
                                        ui.add(
                                            egui::DragValue::new(&mut register)
                                                .hexadecimal(2, false, true)
                                                .prefix("0x"),
                                        )
                                        //.on_hover_text(register_text)
                                        .labelled_by(label.id);
                                    }
                                });
                            }

                            ui.end_row();
                        }
                    });
            });
        });
    }
}
