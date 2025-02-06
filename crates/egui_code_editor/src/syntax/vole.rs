use super::Syntax;
use std::collections::BTreeSet;

impl Syntax {
    pub fn vole() -> Self {
        Syntax {
            language: "Vole Assembly",
            case_sensitive: false,
            comment: ";",
            comment_multiline: ["/*", "*/"],
            hyperlinks: BTreeSet::from(["http"]),
            keywords: BTreeSet::from([
                "ld",   // LOAD, registers have letters, memory locations use parentheses
                "mv",   // MOVE
                "add",  // ADD
                "or",   // OR
                "and",  // AND
                "xor",  // XOR
                "rot",  // ROTATE
                "jp",   // JUMP if zero
                "halt", // HALT
            ]),
            types: BTreeSet::from([]),
            special: BTreeSet::from([
                //8-bit registers
                "r0",  // 0x0
                "r1",  // 0x1
                "r2",  // 0x2
                "r3",  // 0x3
                "r4",  // 0x4
                "r5",  // 0x5
                "r6",  // 0x6
                "r7",  // 0x7
                "r8",  // 0x8
                "r9",  // 0x9
                "r10", // 0xA
                "r11", // 0xB
                "r12", // 0xC
                "r13", // 0xD
                "r14", // 0xE
                "r15", // 0xF
            ]),
        }
    }
}
