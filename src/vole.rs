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
                "The ROM is too large to fit into memory at the offset {:#X}",
                offset
            )
        }
    }

    /// Set the program counter to this address when starting the program
    pub fn set_start_address(&mut self, address: u8) {
        self.pc = address;
    }

    /// Start the machine
    pub fn start(&mut self, start_mode: StartMode) {
        if start_mode == StartMode::Reset {
            self.reset_cpu();
        }
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

    /// Returns the program counter
    pub fn program_counter(&self) -> u8 {
        self.pc
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
        self.pc += 2;

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
                // Reinterprets XY as an i8 since it's directly loaded into a register
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
