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
                "jr",   // JUMP if zero
                "halt", // HALT
            ]),
            types: BTreeSet::from([]),
            special: BTreeSet::from([
                //8-bit registers
                "A", // 0x0
                "B", // 0x1
                "C", // 0x2
                "D", // 0x3
                "E", // 0x4
                "F", // 0x5
                "G", // 0x6
                "H", // 0x7
                "I", // 0x8
                "J", // 0x9
                "K", // 0xA
                "L", // 0xB
                "M", // 0xC
                "N", // 0xD
                "O", // 0xE
                "P", // 0xF
            ]),
        }
    }
}
