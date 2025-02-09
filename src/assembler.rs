use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Assembler;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ValueType {
    Register(u8),
    Address(u8),
    Literal(u8),
    Label(String),
}

impl Assembler {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: Add .org
    // TODO: Add line numbers to errors
    // TODO: Error type instead of Vec<string>
    pub fn assemble(&self, source_code: String) -> Result<(Vec<u8>, u8), Vec<String>> {
        let mut result = Vec::new();
        // <Label, Calling Address>
        let mut labels: HashMap<String, u8> = HashMap::new();

        let lines: Vec<&str> = source_code.split_terminator("\n").collect();
        eprintln!("---------------------------");
        eprintln!("Line count: {}", lines.len());

        let mut program_counter = 0u8;

        // TODO: Return errors with line numbers
        for (line_num, line) in lines.iter().enumerate() {
            eprintln!("---------------------------");
            eprintln!("{:?}: {}", line_num, line);

            // Skip empty lines and comment lines
            if line.starts_with(";") || line.is_empty() {
                eprintln!("Skipping empty or comment");
                continue;
            }

            // Trim whitespace and end of line comments
            let line = line.trim();
            let line = line.split_once(';').map_or(line, |(before, _)| before);
            let line = line.trim_end();

            let (pre, post) = match line.split_once(" ") {
                Some((pre, post)) => (pre, post),
                None => (line, ""),
            };

            match pre.to_lowercase().as_str() {
                "ld" => {
                    let (lhs, rhs) = self.split_two_args(post);
                    eprintln!("lhs_str: {}\nrhs_str: {}", lhs, rhs);

                    let lhs = match self.resolve_argument(&lhs) {
                        Ok(v) => v,
                        Err(e) => {
                            // TODO: Fix this
                            println!("Fix this: {:?}", e);
                            ValueType::Literal(0x00)
                        }
                    };
                    let rhs = match self.resolve_argument(&rhs) {
                        Ok(v) => v,
                        Err(e) => {
                            // TODO: Fix this
                            println!("Fix this: {:?}", e);
                            ValueType::Literal(0x00)
                        }
                    };
                    eprintln!("lhs: {:?}\nrhs: {:?}", lhs, rhs);

                    match lhs {
                        ValueType::Register(r0) => match rhs {
                            ValueType::Register(r1) => {
                                //0x4RXY
                                // TODO: Not tested
                                let high = (0x4u8 << 4) | r0;
                                let low = r1;

                                eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                                result.push(high);
                                result.push(low);
                            }
                            ValueType::Address(a) => {
                                //0x1RXY
                                let high = (0x1u8 << 4) | r0;
                                let low = a;

                                eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                                result.push(high);
                                result.push(low);
                            }
                            ValueType::Literal(l) => {
                                //0x2RXY
                                let high = (0x2u8 << 4) | r0;
                                let low = l;

                                eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                                result.push(high);
                                result.push(low);
                            }
                            ValueType::Label(l) => {
                                // TODO: Fix this
                                let msg = format!("Cannot store {} in register", l);
                                println!("{}", msg);
                                continue;
                            }
                        },
                        ValueType::Address(a) => {
                            //0x3RXY
                            match rhs {
                                ValueType::Register(r) => {
                                    let high = (0x3u8 << 4) | r;
                                    let low = a;

                                    eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                                    result.push(high);
                                    result.push(low);
                                }
                                other => {
                                    // TODO: Fix this
                                    let msg = format!("Cannot store {:?} in memory", other);
                                    println!("{}", msg);
                                    continue;
                                }
                            };
                        }
                        _ => {
                            // TODO: Fix this
                            println!("Failed to determine ld type");
                            continue;
                        }
                    };
                }
                "adds" => {
                    // TODO: Not tested
                    //0x5RST
                    let (r, s, t) = self.resolve_rst(post);

                    let high = (0x5u8 << 4) | r;
                    let low = (s << 4) | t;

                    eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                    result.push(high);
                    result.push(low);
                }
                "addf" => {
                    // TODO: Not tested
                    //0x6RST

                    let (r, s, t) = self.resolve_rst(post);

                    let high = (0x6u8 << 4) | r;
                    let low = (s << 4) | t;

                    eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                    result.push(high);
                    result.push(low);
                }
                "or" => {
                    // TODO: Not tested
                    //0x7RST
                    let (r, s, t) = self.resolve_rst(post);

                    let high = (0x7u8 << 4) | r;
                    let low = (s << 4) | t;

                    eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                    result.push(high);
                    result.push(low);
                }
                "and" => {
                    // TODO: Not tested
                    //0x8RST
                    let (r, s, t) = self.resolve_rst(post);

                    let high = (0x8u8 << 4) | r;
                    let low = (s << 4) | t;

                    eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                    result.push(high);
                    result.push(low);
                }
                "xor" => {
                    // TODO: Not tested
                    //0x9RST
                    let (r, s, t) = self.resolve_rst(post);

                    let high = (0x9u8 << 4) | r;
                    let low = (s << 4) | t;

                    eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                    result.push(high);
                    result.push(low);
                }
                "rot" => {
                    // TODO: Not tested
                    //0xAR0X
                    let (lhs, rhs) = self.split_two_args(post);
                    eprintln!("lhs_str: {}\nrhs_str: {}", lhs, rhs);

                    let lhs = match self.resolve_argument(&lhs) {
                        Ok(v) => match v {
                            ValueType::Register(r) => r,
                            other => {
                                // TODO: Fix this
                                println!("Fix this: {:?}", other);
                                0x00
                            }
                        },
                        Err(e) => {
                            // TODO: Fix this
                            println!("Fix this: {:?}", e);
                            0x00
                        }
                    };
                    let rhs = match self.resolve_argument(&rhs) {
                        Ok(v) => match v {
                            ValueType::Literal(l) => l,
                            other => {
                                // TODO: Fix this
                                println!("Fix this: {:?}", other);
                                0x00
                            }
                        },
                        Err(e) => {
                            // TODO: Fix this
                            println!("Fix this: {:?}", e);
                            0x00
                        }
                    };
                    eprintln!("lhs: {:?}\nrhs: {:?}", lhs, rhs);

                    let high = (0xAu8 << 4) | lhs;
                    let low = rhs;

                    eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                    result.push(high);
                    result.push(low);
                }
                "halt" => {
                    result.push(0xC0);
                    result.push(0x00);
                    eprintln!("Pushing 0xC0, 0x00");
                }
                "jp" => {
                    // TODO: Handle labels
                    // TODO: Add support for multiple jump instructions to the same label
                    //0xBRXY
                    let (lhs, rhs) = self.split_two_args(post);
                    eprintln!("lhs_str: {}\nrhs_str: {}", lhs, rhs);

                    let lhs = match self.resolve_argument(&lhs) {
                        Ok(v) => match v {
                            ValueType::Register(r) => r,
                            other => {
                                // TODO: Fix this
                                let msg = format!("Invalid argument for jp {:?}", other);
                                println!("{}", msg);
                                continue;
                            }
                        },
                        Err(e) => {
                            // TODO: Fix this
                            let msg = format!("Error resoloving argument {}", e);
                            println!("{}", msg);
                            continue;
                        }
                    };

                    let high = (0xBu8 << 4) | lhs;
                    let low = 0xFF;

                    eprintln!("Pushing: {:#04X?}, {:#04X?}", high, low);
                    result.push(high);
                    result.push(low);

                    let call_address = result.len() - 1;
                    labels.insert(rhs, call_address as u8);
                    eprintln!("Label call address: {}", call_address);
                }
                ".org" => {
                    program_counter = match self.resolve_argument(&post) {
                        Ok(result) => {
                            match result {
                                ValueType::Literal(v) => v,
                                e => {
                                    // TODO: Fix this
                                    let msg = format!("Error resoloving argument {:?}", e);
                                    println!("{}", msg);
                                    0
                                }
                            }
                        }
                        Err(e) => {
                            // TODO: Fix this
                            let msg = format!("Error resoloving argument {}", e);
                            println!("{}", msg);
                            0
                        }
                    };

                    for _ in 0..program_counter {
                        result.push(0x00);
                    }
                }
                unknown => {
                    if unknown.trim_end().ends_with(":") {
                        let label = unknown.trim_end_matches(":");

                        if labels.contains_key(label) {
                            eprintln!("Labels");

                            // TODO: Check for unresolved labels
                            match labels.remove_entry(label) {
                                Some((_k, call)) => {
                                    // The target jump address will be the next line
                                    let target = result.len() as u8;
                                    result[call as usize] = target;

                                    eprintln!("Storing jump target {:#04X?}", target);
                                }
                                None => {
                                    // TODO: Fix this
                                    let msg = format!("Error resoloving label: {}", label);
                                    println!("{}", msg);
                                    continue;
                                }
                            }
                        }
                    } else {
                        eprintln!("Unknown mnemonic: {}", pre);
                    }
                }
            }
        }

        eprintln!("---------------------------");
        eprintln!("Assembler completed");

        Ok((result, program_counter))
    }

    fn resolve_argument(&self, arg: &str) -> Result<ValueType, String> {
        let val = arg.to_lowercase();
        if val.starts_with("r") {
            // Register
            match self.register_to_value(val.as_str()) {
                Ok(v) => {
                    return Ok(ValueType::Register(v));
                }
                Err(e) => {
                    // TODO: Fix this
                    println!("Fix this: {}", e);
                }
            }
        }

        if val.starts_with("(") && val.ends_with(")") {
            // Memory address
            match self.numeric_to_value(val.as_str()) {
                Ok(v) => {
                    return Ok(ValueType::Address(v));
                }
                Err(e) => {
                    // TODO: Fix this
                    println!("Fix this: {}", e);
                }
            }
        } else if val.starts_with("(") || val.ends_with(")") {
            return Err(format!("Malformed memory address: {}", val));
        }

        if val.starts_with("0x") || val.starts_with("0b") {
            // Literal
            match self.numeric_to_value(val.as_str()) {
                Ok(v) => {
                    return Ok(ValueType::Literal(v));
                }
                Err(e) => {
                    // TODO: Fix this
                    println!("Fix this: {}", e);
                }
            }
        }

        if val.trim_end().ends_with(":") {
            return Ok(ValueType::Label(val.trim_end_matches(":").to_string()));
        }

        //TODO: labels
        Ok(ValueType::Literal(0x0))
    }

    fn register_to_value(&self, reg: &str) -> Result<u8, String> {
        match reg {
            "r0" => Ok(0x0),
            "r2" => Ok(0x2),
            "r1" => Ok(0x1),
            "r3" => Ok(0x3),
            "r4" => Ok(0x4),
            "r5" => Ok(0x5),
            "r6" => Ok(0x6),
            "r7" => Ok(0x7),
            "r8" => Ok(0x8),
            "r9" => Ok(0x9),
            "ra" => Ok(0xA),
            "rb" => Ok(0xB),
            "rc" => Ok(0xC),
            "rd" => Ok(0xD),
            "re" => Ok(0xE),
            "rf" => Ok(0xF),
            _ => Err(format!("Invalid register {}", reg)),
        }
    }

    fn numeric_to_value(&self, num: &str) -> Result<u8, String> {
        let value = num.trim_start_matches("(").trim_end_matches(")");

        let radix = if value.starts_with("0x") {
            16
        } else if value.starts_with("0b") {
            2
        } else {
            return Err(format!("Malformed number: {}", num));
        };

        let prefix = if value.starts_with("0x") {
            "0x"
        } else if value.starts_with("0b") {
            "0b"
        } else {
            return Err(format!("Malformed number: {}", num));
        };

        let value = value.strip_prefix(prefix).unwrap_or(value);
        let value = u8::from_str_radix(value, radix).unwrap_or_default();

        Ok(value)
    }

    fn split_two_args(&self, args: &str) -> (String, String) {
        let result: Vec<&str> = args.split(",").flat_map(|s| s.split(", ")).collect();
        (result[0].trim().to_string(), result[1].trim().to_string())
    }

    fn split_three_args(&self, args: &str) -> (String, String, String) {
        let result: Vec<&str> = args.split(",").flat_map(|s| s.split(", ")).collect();
        (
            result[0].trim().to_string(),
            result[1].trim().to_string(),
            result[2].trim().to_string(),
        )
    }

    fn resolve_rst(&self, arg: &str) -> (u8, u8, u8) {
        let (r, s, t) = self.split_three_args(arg);

        let r = match self.resolve_argument(&r) {
            Ok(v) => match v {
                ValueType::Register(v) => v,
                e => {
                    // TODO: Fix this
                    println!("Fix this: {:?}", e);
                    0x00
                }
            },
            Err(e) => {
                // TODO: Fix this
                println!("Fix this: {:?}", e);
                0x00
            }
        };

        let s = match self.resolve_argument(&s) {
            Ok(v) => match v {
                ValueType::Register(v) => v,
                e => {
                    // TODO: Fix this
                    println!("Fix this: {:?}", e);
                    0x00
                }
            },
            Err(e) => {
                // TODO: Fix this
                println!("Fix this: {:?}", e);
                0x00
            }
        };

        let t = match self.resolve_argument(&t) {
            Ok(v) => match v {
                ValueType::Register(v) => v,
                e => {
                    // TODO: Fix this
                    println!("Fix this: {:?}", e);
                    0x00
                }
            },
            Err(e) => {
                // TODO: Fix this
                println!("Fix this: {:?}", e);
                0x00
            }
        };

        (r, s, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, Rng};

    #[test]
    fn ld() {
        let asm = Assembler::new();

        let l1 = "ld r0, 0x01".to_owned();
        let result = asm.assemble(l1).expect("Invalid assembler output");
        assert_eq!(result.0, [0x20, 0x01]);

        let l2 = "ld r0, 0b00000001".to_owned();
        let result = asm.assemble(l2).expect("Invalid assembler output");
        assert_eq!(result.0, [0x20, 0x01]);
    }

    #[test]
    fn ld_0x2() {
        let asm = Assembler::new();
        let mut rng = rand::rng();

        let mut program = String::new();
        for i in 0..16 {
            let reg = match decimal_to_register_string(i) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Invalid register {}", e);
                    "r0".to_owned()
                }
            };

            let value = rng.random::<u8>();
            let value = if rng.random_bool(0.5) {
                format!("{:#04X?}", value)
            } else {
                format!("{:#010b}", value)
            };
            let inst = format!("ld {}, {}\n", reg, value);
            program.push_str(&inst);
        }

        // TODO: Actually compare values
        let result = asm.assemble(program).expect("Invalid assembler output");
        assert_eq!(result.0.len(), 32);
    }

    #[test]
    fn split_args_two() {
        let asm = Assembler::new();

        let (l, r) = asm.split_two_args(&"a,b");
        assert_eq!(l, "a");
        assert_eq!(r, "b");

        let (l, r) = asm.split_two_args("a, b");
        assert_eq!(l, "a");
        assert_eq!(r, "b");

        let (l, r) = asm.split_two_args("a , b");
        assert_eq!(l, "a");
        assert_eq!(r, "b");
    }

    #[test]
    fn split_args_three() {
        let asm = Assembler::new();

        let (l, m, r) = asm.split_three_args(&"a,b,c");
        assert_eq!(l, "a");
        assert_eq!(m, "b");
        assert_eq!(r, "c");

        let (l, m, r) = asm.split_three_args(&"a, b, c");
        assert_eq!(l, "a");
        assert_eq!(m, "b");
        assert_eq!(r, "c");

        let (l, m, r) = asm.split_three_args(&"a, b,c");
        assert_eq!(l, "a");
        assert_eq!(m, "b");
        assert_eq!(r, "c");

        let (l, m, r) = asm.split_three_args(&"a,b, c");
        assert_eq!(l, "a");
        assert_eq!(m, "b");
        assert_eq!(r, "c");
    }

    #[test]
    fn demo_program() {
        const TEST_RESULT: &[u8] = &[
            0x20, 0x00, // Load 0x00 into r0
            0x25, 0xFF, // Load 0xFF into r5
            0x14, 0x44, // Load mem 0x44 into r4
            0xB4, 0x0A, // If r4 == r0, jump to mem 0x0A (skip next line)
            0x25, 0x01, // load 0x01 into r5
            0x35, 0x46, // Store r5 into mem 0x46
            0xC0, 0x00, // Quit
        ];

        const TEST_SOURCE: &str = "
ld r0,0x00        ; Load 0x00 into r0
LD R5, 0xFF        ; Load 0xFF into r5
Ld r4, (0x44)      ; Load mem 0x44 into r4

jp r4, continue    ; If r4 == r0, jump to continue
lD r5, 0x01        ; Load 0x01 into r5

continue:
    ld (0x46), r5  ; Store r5 into mem 0x46
    halt           ; Quit";

        let asm = Assembler::new();
        let result = asm
            .assemble(TEST_SOURCE.to_string())
            .expect("Invalid assembler output");

        assert_eq!(result.0, TEST_RESULT);
    }

    #[test]
    fn mnemonics() {
        const MNEMONIC_SOURCE: &str = "
.org 0x02           ; Offset start by 2

ld r0,0x00          ; Load 0x00 into r0
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

        const MNEMONIC_RESULT: &[u8] = &[
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

        let asm = Assembler::new();
        let result = asm
            .assemble(MNEMONIC_SOURCE.to_string())
            .expect("Invalid assembler output");

        assert_eq!(result.0, MNEMONIC_RESULT);
    }

    fn decimal_to_register_string(reg: usize) -> Result<String, String> {
        match reg {
            0x0 => Ok("r0".to_owned()),
            0x2 => Ok("r2".to_owned()),
            0x1 => Ok("r1".to_owned()),
            0x3 => Ok("r3".to_owned()),
            0x4 => Ok("r4".to_owned()),
            0x5 => Ok("r5".to_owned()),
            0x6 => Ok("r6".to_owned()),
            0x7 => Ok("r7".to_owned()),
            0x8 => Ok("r8".to_owned()),
            0x9 => Ok("r9".to_owned()),
            0xA => Ok("ra".to_owned()),
            0xB => Ok("rb".to_owned()),
            0xC => Ok("rc".to_owned()),
            0xD => Ok("rd".to_owned()),
            0xE => Ok("re".to_owned()),
            0xF => Ok("rf".to_owned()),
            _ => Err(format!("Invalid register {}", reg)),
        }
    }
}
