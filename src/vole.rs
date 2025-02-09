/// Vole virtual machine representation
pub struct Vole {
    memory: Vec<u8>,
    registers: Vec<u8>,

    // Program Counter
    pc: u8,

    // Index register
    ir: u16,

    running: bool,
}

/// Machine start mode
#[derive(Debug, PartialEq)]
pub enum StartMode {
    /// Reset the registers, program counter and instruction counter
    Reset,

    /// Keep the state from the previous execution
    KeepState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CycleError {
    InvalidOpcode(String),
}

impl Default for Vole {
    fn default() -> Self {
        Self {
            registers: vec![0; 16],
            memory: vec![0; 256],
            pc: 0,
            ir: 0,
            running: false,
        }
    }
}

impl Vole {
    /// Create a new machine instance
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Loads the given rom into memory starting at 0
    pub fn load_rom(&mut self, rom: &[u8]) {
        // TODO: Fix this
        if rom.len() <= self.memory.len() {
            self.memory[0..rom.len()].clone_from_slice(rom);
        }
    }

    /// Loads the given rom into memory starting at the given offset
    pub fn load_rom_offset(&mut self, rom: &[u8], offset: usize) {
        // TODO: Fix this
        if rom.len() <= self.memory.len() - offset {
            self.memory[offset..offset + rom.len()].clone_from_slice(rom);
        } else {
            // TODO: UI notification
            println!(
                "The ROM {} is too large to fit into memory {} at the offset {:#X}",
                rom.len(),
                self.memory.len(),
                offset,
            );
        }
    }

    /// Start the machine
    pub fn start(&mut self, start_mode: &StartMode, start_location: Option<u8>) {
        if *start_mode == StartMode::Reset {
            self.reset_cpu();
        }
        self.set_program_counter(start_location.unwrap_or(0x00));
        self.running = true;
    }

    /// Is the machine running
    pub fn running(&self) -> bool {
        self.running
    }

    /// Set the value of a memory address
    pub fn set_memory_value(&mut self, address: u8, value: u8) {
        self.memory[address as usize] = value;
    }

    /// Returns the value of a memory address
    pub fn memory_value(&mut self, address: u8) -> u8 {
        self.memory[address as usize]
    }

    /// Returns the memory cells
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    /// Reset the state of the CPU
    pub fn reset_cpu(&mut self) {
        self.ir = 0;
        self.pc = 0;
        self.registers = vec![0; 16];
    }

    /// Get the registers
    pub fn registers(&self) -> &[u8] {
        &self.registers
    }

    /// Get the registers mutable
    pub fn registers_mut(&mut self) -> &mut [u8] {
        &mut self.registers
    }

    /// Set a register value
    pub fn set_register_value(&mut self, register: u8, value: u8) {
        self.registers[register as usize] = value;
    }

    /// Returns the program counter
    pub fn program_counter(&self) -> u8 {
        self.pc
    }

    /// Set the program counter to an address
    pub fn set_program_counter(&mut self, address: u8) {
        self.pc = address;
    }

    /// Returns the instruction register
    pub fn instruction_register(&self) -> u16 {
        self.ir
    }

    /// Perform a fetch-decode-execute cycle
    pub fn cycle(&mut self) -> Result<(), CycleError> {
        // TODO: Execution trace
        /*
           Fetch
        */
        self.ir = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[(self.pc + 1) as usize]) as u16;

        //println!("{:#x}", self.ir);

        // TODO: Store PC in execution trace prior to incrementing
        // Increment program counter now, the jump instruction will overwrite this during the execute step
        if self.ir != 0xC000 {
            self.pc += 2;
        }

        /*
           Decode
        */
        let r = ((self.ir & 0x0F00) >> 8) as u8;
        let s = ((self.ir & 0x00F0) >> 4) as u8;
        let t = (self.ir & 0x000F) as u8;
        let xy = (self.ir & 0x00FF) as u8;

        /*
           Execute
        */
        match self.ir & 0xF000 {
            0x1000 => {
                // Load register R with memory XY
                self.registers[r as usize] = self.memory[xy as usize];
            }
            0x2000 => {
                // Load register R with XY
                self.registers[r as usize] = xy;
            }
            0x3000 => {
                // Store register R into memory XY
                self.memory[xy as usize] = self.registers[r as usize];
            }
            0x4000 => {
                // Move register S into register R
                self.registers[r as usize] = self.registers[s as usize];
            }
            0x5000 => {
                // Add register S and register T as twos compliment, store result in R
                let reg_s = self.registers[s as usize] as i8;
                let reg_t = self.registers[t as usize] as i8;
                self.registers[r as usize] = reg_s.wrapping_add(reg_t) as u8;
            }
            0x6000 => {
                // Due to the specification requirements, this machine converts to the lowest precision available and back to u8.
                // TODO: Implement the FP operation as described in the book

                // Add register S and register T as floating point, store result in R
                self.registers[r as usize] =
                    (self.registers[s as usize] as f32 + self.registers[t as usize] as f32) as u8;
            }
            0x7000 => {
                // OR register S and register T, store result in R
                self.registers[r as usize] =
                    self.registers[s as usize] | self.registers[t as usize];
            }
            0x8000 => {
                // AND register S and register T, store result in R
                self.registers[r as usize] =
                    self.registers[s as usize] & self.registers[t as usize];
            }
            0x9000 => {
                // XOR register S and register T, store result in R
                self.registers[r as usize] =
                    self.registers[s as usize] ^ self.registers[t as usize];
            }
            0xA000 => {
                // Rotate bit pattern in register R to the right X (t) times
                self.registers[r as usize] = self.registers[r as usize].rotate_right(t as u32);
            }
            0xB000 => {
                // Jump to the instruction at memory XY if register R equals register 0
                if self.registers[r as usize] == self.registers[0] {
                    self.pc = xy;
                }
            }
            0xC000 => {
                self.running = false;
            }
            _ => {
                self.running = false;
                return Err(CycleError::InvalidOpcode(format!("0x{:04X}", self.ir)));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, Rng};

    // Helper function
    fn generate_random_rom(length: usize) -> Vec<u8> {
        assert!(length > 0);

        let mut rng = rand::rng();
        let mut rom = Vec::new();
        for _ in 0..length {
            rom.push(rng.random::<u8>());
        }

        rom
    }

    #[test]
    fn load_rom() {
        let rom_length = rand::random::<u8>().max(1) as usize;
        let rom = generate_random_rom(rom_length);

        // Load the rom into the device
        let mut device = Vole::new();
        device.load_rom(&rom);

        assert!(rom_length > 0);
        assert_eq!(rom, device.memory()[0..rom_length]);
    }

    #[test]
    fn load_rom_offset() {
        let mut rng = rand::rng();

        // Select a random start offset (exclusive)
        let start_offset = rng.random_range(0..255) as usize;

        // Randomly generate a rom
        let rom_length = rng.random_range(0..=(255 - start_offset)).max(1);
        let rom = generate_random_rom(rom_length);

        // Load the rom into the device
        let mut device = Vole::new();
        device.load_rom_offset(&rom, start_offset);

        assert!(rom_length > 0);
        assert_eq!(
            rom,
            device.memory()[start_offset..(start_offset + rom_length)]
        );
    }

    #[test]
    fn device_start() {
        let mut rng = rand::rng();

        // Randomly generate a rom
        let rom_length = rand::random::<u8>().max(1) as usize;
        let rom = generate_random_rom(rom_length);

        let mut device = Vole::new();
        device.load_rom(&rom);

        // Set random data on the device
        let random_pc = rng.random::<u8>().max(1);
        device.set_program_counter(random_pc);

        let mut random_registers = Vec::new();
        for r in device.registers_mut().iter_mut() {
            let val = rng.random::<u8>();
            *r = val;
            random_registers.push(val);
        }
        let random_registers = random_registers;

        device.start(&StartMode::KeepState, Some(device.program_counter()));
        assert!(rom_length > 0);
        assert_eq!(rom, device.memory()[0..rom_length]);
        assert_eq!(device.registers(), random_registers);
        assert_eq!(device.program_counter(), random_pc);

        device.start(&StartMode::Reset, None);
        assert!(rom_length > 0);
        assert_eq!(rom, device.memory()[0..rom_length]);
        assert_ne!(device.registers(), random_registers);
        assert_ne!(device.program_counter(), random_pc);
    }

    // TODO: Fuzzed variation
    #[test]
    fn invalid_opcode() {
        let mut device = Vole::new();

        let empty_rom = [0x0000];
        device.load_rom(&empty_rom);
        device.start(&StartMode::Reset, None);

        let result = device.cycle().unwrap_err();
        assert_eq!(result, CycleError::InvalidOpcode("0x0000".to_string()));
    }
}
