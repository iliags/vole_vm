pub struct Vole {
    memory: Vec<i8>,
    registers: Vec<i8>,

    // Program Counter
    pc: u16,

    // Index register
    ir: u16,

    running: bool,
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
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Loads the given rom into the memory range starting at 0
    pub fn load_rom(&mut self, rom: &[i8]) {
        if rom.len() < self.memory.len() {
            self.memory[0..rom.len()].copy_from_slice(rom);
        }
    }

    /// Loads the given rom into memory starting at the given offset
    pub fn load_rom_offset(&mut self, rom: &[i8], offset: usize) {
        if rom.len() < self.memory.len() - offset {
            self.memory[offset..offset + rom.len()].copy_from_slice(rom);
        }
    }

    /// Set the program counter to this address when starting the program
    pub fn set_start_address(&mut self, address: u16) {
        self.pc = address;
    }

    /// Start the machine
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Is the machine running
    pub fn running(&self) -> bool {
        self.running
    }

    /// Set the value of a memory address
    pub fn set_memory_value(&mut self, address: i8, value: i8) {
        self.memory[address as usize] = value;
    }

    /// Get the value of a memory address
    pub fn memory_value(&mut self, address: i8) -> i8 {
        self.memory[address as usize]
    }

    /// Perform a fetch-decode-execute cycle
    pub fn cycle(&mut self) {
        self.ir = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[(self.pc + 1) as usize]) as u16;

        //println!("Opcode: {:#X}", self.ir);

        // Increment program counter
        self.pc += 2;

        // Break the opcode into nibbles
        let r = ((self.ir & 0x0F00) >> 8) as u8;
        let s = ((self.ir & 0x00F0) >> 4) as u8;
        let t = (self.ir & 0x000F) as u8;
        let xy = (self.ir & 0x00FF) as u8;

        // Execute opcode
        match self.ir & 0xF000 {
            0x1000 => {
                // Load register R with memory XY
                self.registers[r as usize] = self.memory[xy as usize];
            }
            0x2000 => {
                // Load register R with XY
                self.registers[r as usize] = xy as i8;
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
                self.registers[r as usize] =
                    self.registers[s as usize].wrapping_add(self.registers[t as usize])
            }
            0x6000 => {
                // Due to the conflicting specification requirements, this machine converts to the lowest precision available and back to u8.
                // TODO: Implement the FP operation as described in the book

                // Add register S and register T as floating point, store result in R
                self.registers[r as usize] =
                    (self.registers[s as usize] as f32 + self.registers[t as usize] as f32) as i8;
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
                    self.pc = xy as u16;
                }
            }
            0xC000 => {
                println!("Exiting");
                self.running = false;
            }
            _ => {
                panic!("Invalid opcode: {:#X}", self.ir);
            }
        }
    }
}
