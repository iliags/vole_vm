#[derive(Debug, Default)]
pub struct AssemblerResult {
    rom: Vec<u8>,
    program_counter: u8,
}

impl AssemblerResult {
    pub fn new() -> Self {
        AssemblerResult::default()
    }

    pub fn program_counter(&self) -> u8 {
        self.program_counter
    }

    pub fn program_counter_mut(&mut self) -> &mut u8 {
        &mut self.program_counter
    }

    pub fn rom(&self) -> &[u8] {
        &self.rom
    }

    pub fn rom_mut(&mut self) -> &mut Vec<u8> {
        &mut self.rom
    }
}
