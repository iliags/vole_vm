pub mod asm_result;
pub mod assembler;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssemblerError {
    #[error("Malformed memory address '{1}' at line {0}")]
    MalformedAddress(usize, String),

    #[error("Unknown register '{1}' at line {0}")]
    UnknownRegister(usize, String),

    #[error("Malformed number '{1}' at line {0}")]
    MalformedNumber(usize, String),

    #[error("Type mismatch '{1}' at line {0}")]
    TypeMismatch(usize, String),

    #[error("{1} at line {0}")]
    LoadOpFail(usize, String),

    #[error("Error resolving label '{1}' at line {0}")]
    LabelResolution(usize, String),

    #[error("Unknown argument '{1}' at line {0}")]
    UnknownArgument(usize, String),
}

pub const DEMO_SOURCE: &str = ".org 0x02           ; Offset start by 2

ld r0, 0x00         ; Load 0x00 into r0
ld r5, 0xFF         ; Load 0xFF into r5
ld r4, (0x44)       ; Load mem 0x44 into r4

jp r4, continue     ; If r4 == r0, jump to continue
ld r5, 0x01         ; Load 0x01 into r5

continue:
    ld (0x46), r5   ; Store r5 into mem 0x46

    ld r6, 0x01     ; Load 1 into r6
    ld r7, 0x01     ; Load 1 into r7
    adds r8, r6, r7 ; Add r6 and r7 as two's compliment, store in r8
    addf r9, r6, r7 ; Add r6 and r7 as float, store in r9
    or ra, r6, r7   ; OR r6 and r7 as float, store in ra
    and rb, r6, r7  ; AND r6 and r7 as float, store in rb
    xor rc, r6, r7  ; XOR r6 and r7 as float, store in rc
    rot rd, 0x02    ; ROTATE rd to the right 2 times

    halt            ; Quit";

pub const DEMO_ROM: &[u8] = &[
    0x00, 0x00, // Offset by 2
    0x20, 0x00, // Load 0x00 into r0
    0x25, 0xFF, // Load 0xFF into r5
    0x14, 0x44, // Load mem 0x44 into r4
    0xB4, 0x0C, // If r4 == r0, jump to mem 0x0C (skip next line)
    0x25, 0x01, // load 0x01 into r5
    0x35, 0x46, // Store r5 into mem 0x46
    0x26, 0x01, // Load 1 into r6
    0x27, 0x01, // Load 1 into r7
    0x58, 0x67, // Add r6 and r7 as two's compliment, store in r8
    0x69, 0x67, // Add r6 and r7 as float, store in r9
    0x7A, 0x67, // OR r6 and r7 as float, store in ra
    0x8B, 0x67, // AND r6 and r7 as float, store in rb
    0x9C, 0x67, // XOR r6 and r7 as float, store in rc
    0xAD, 0x02, // ROTATE rd to the right 2 times
    0xC0, 0x00, // Quit
];
