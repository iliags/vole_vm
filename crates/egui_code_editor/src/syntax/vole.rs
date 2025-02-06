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
                "adds", // ADD two's compliment
                "addf", // ADD float
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
                "r0", // 0x0
                "r1", // 0x1
                "r2", // 0x2
                "r3", // 0x3
                "r4", // 0x4
                "r5", // 0x5
                "r6", // 0x6
                "r7", // 0x7
                "r8", // 0x8
                "r9", // 0x9
                "ra", // 0xA
                "rb", // 0xB
                "rc", // 0xC
                "rd", // 0xD
                "re", // 0xE
                "rf", // 0xF
            ]),
        }
    }
}
