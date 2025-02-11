use crate::asm::asm_result::AssemblerResult;

use super::AssemblerError;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Assembler {
    log: String,
    line_number: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ValueType {
    Register(u8),
    Address(u8),
    Literal(u8),
    Label(String),
}

impl Assembler {
    #[must_use]
    pub fn new() -> Self {
        Assembler::default()
    }

    pub fn add_log(&mut self, line: &str) {
        self.log += line;
    }

    pub fn log(&self) -> String {
        self.log.clone()
    }

    /// # Errors
    ///
    /// Will return `AssemblerError` if an error occurs during assembly
    pub fn assemble(&mut self, source_code: String) -> Result<AssemblerResult, AssemblerError> {
        // <Label, Calling Address>
        let mut labels: HashMap<String, u8> = HashMap::new();

        let mut asm_result = AssemblerResult::new();

        let source_lines: Vec<&str> = source_code.split_terminator("\n").collect();

        self.add_log("---------------------------");
        self.add_log(&format!("Line count: {}", source_lines.len()));

        for (line_num, line) in source_lines.iter().enumerate() {
            self.line_number = line_num;
            self.add_log("---------------------------");
            self.add_log(&format!("{:?}: {}", line_num, line));

            // Skip empty lines and comment lines
            if line.starts_with(";") || line.is_empty() {
                self.add_log("Skipping empty or comment");
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
                    let (lhs, rhs) = split_two_args(post);
                    self.add_log(&format!("lhs_str: {}\nrhs_str: {}", lhs, rhs));

                    let lhs = self.resolve_argument(&lhs)?;
                    let rhs = self.resolve_argument(&rhs)?;
                    self.add_log(&format!("lhs: {:?}\nrhs: {:?}", lhs, rhs));

                    match lhs {
                        ValueType::Register(r0) => match rhs {
                            ValueType::Register(r1) => {
                                //0x4RXY
                                let high = (0x4u8 << 4) | r0;
                                let low = r1;

                                self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                                asm_result.rom_mut().push(high);
                                asm_result.rom_mut().push(low);
                            }
                            ValueType::Address(a) => {
                                //0x1RXY
                                let high = (0x1u8 << 4) | r0;
                                let low = a;

                                self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                                asm_result.rom_mut().push(high);
                                asm_result.rom_mut().push(low);
                            }
                            ValueType::Literal(l) => {
                                //0x2RXY
                                let high = (0x2u8 << 4) | r0;
                                let low = l;

                                self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                                asm_result.rom_mut().push(high);
                                asm_result.rom_mut().push(low);
                            }
                            ValueType::Label(l) => {
                                return Err(AssemblerError::TypeMismatch(line_num, l))
                            }
                        },
                        ValueType::Address(a) => {
                            //0x3RXY
                            match rhs {
                                ValueType::Register(r) => {
                                    let high = (0x3u8 << 4) | r;
                                    let low = a;

                                    self.add_log(&format!(
                                        "Pushing: {:#04X?}, {:#04X?}",
                                        high, low
                                    ));
                                    asm_result.rom_mut().push(high);
                                    asm_result.rom_mut().push(low);
                                }
                                _ => {
                                    return Err(AssemblerError::TypeMismatch(
                                        line_num,
                                        "non-register value".to_owned(),
                                    ));
                                }
                            };
                        }
                        _ => {
                            return Err(AssemblerError::LoadOpFail(
                                line_num,
                                "Failed to determine ld type".to_owned(),
                            ));
                        }
                    };
                }
                "adds" => {
                    //0x5RST
                    let (r, s, t) = self.resolve_rst(post)?;

                    let high = (0x5u8 << 4) | r;
                    let low = (s << 4) | t;

                    self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                    asm_result.rom_mut().push(high);
                    asm_result.rom_mut().push(low);
                }
                "addf" => {
                    //0x6RST

                    let (r, s, t) = self.resolve_rst(post)?;

                    let high = (0x6u8 << 4) | r;
                    let low = (s << 4) | t;

                    self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                    asm_result.rom_mut().push(high);
                    asm_result.rom_mut().push(low);
                }
                "or" => {
                    //0x7RST
                    let (r, s, t) = self.resolve_rst(post)?;

                    let high = (0x7u8 << 4) | r;
                    let low = (s << 4) | t;

                    self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                    asm_result.rom_mut().push(high);
                    asm_result.rom_mut().push(low);
                }
                "and" => {
                    //0x8RST
                    let (r, s, t) = self.resolve_rst(post)?;

                    let high = (0x8u8 << 4) | r;
                    let low = (s << 4) | t;

                    self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                    asm_result.rom_mut().push(high);
                    asm_result.rom_mut().push(low);
                }
                "xor" => {
                    //0x9RST
                    let (r, s, t) = self.resolve_rst(post)?;

                    let high = (0x9u8 << 4) | r;
                    let low = (s << 4) | t;

                    self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                    asm_result.rom_mut().push(high);
                    asm_result.rom_mut().push(low);
                }
                "rot" => {
                    //0xAR0X
                    let (lhs, rhs) = split_two_args(post);
                    self.add_log(&format!("lhs_str: {}\nrhs_str: {}", lhs, rhs));

                    let lhs = match self.resolve_argument(&lhs) {
                        Ok(v) => match v {
                            ValueType::Register(r) => r,
                            _ => {
                                return Err(AssemblerError::TypeMismatch(
                                    line_num,
                                    "non-register".to_string(),
                                ));
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    };
                    let rhs = match self.resolve_argument(&rhs) {
                        Ok(v) => match v {
                            ValueType::Literal(l) => l,
                            _ => {
                                return Err(AssemblerError::TypeMismatch(
                                    line_num,
                                    "non-literal".to_string(),
                                ));
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    };
                    self.add_log(&format!("lhs: {:?}\nrhs: {:?}", lhs, rhs));

                    let high = (0xAu8 << 4) | lhs;
                    let low = rhs;

                    self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                    asm_result.rom_mut().push(high);
                    asm_result.rom_mut().push(low);
                }
                "halt" => {
                    asm_result.rom_mut().push(0xC0);
                    asm_result.rom_mut().push(0x00);
                    self.add_log("Pushing 0xC0, 0x00");
                }
                "jp" => {
                    // TODO: Handle labels
                    // TODO: Add support for multiple jump instructions to the same label
                    //0xBRXY
                    let (lhs, rhs) = split_two_args(post);
                    self.add_log(&format!("lhs_str: {}\nrhs_str: {}", lhs, rhs));

                    let lhs = match self.resolve_argument(&lhs) {
                        Ok(v) => match v {
                            ValueType::Register(r) => r,
                            _ => {
                                return Err(AssemblerError::TypeMismatch(
                                    line_num,
                                    "non-register".to_string(),
                                ));
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    let high = (0xBu8 << 4) | lhs;
                    let low = 0xFF;

                    self.add_log(&format!("Pushing: {:#04X?}, {:#04X?}", high, low));
                    asm_result.rom_mut().push(high);
                    asm_result.rom_mut().push(low);

                    let call_address = asm_result.rom().len() - 1;
                    labels.insert(rhs, call_address as u8);
                    self.add_log(&format!("Label call address: {}", call_address));
                }
                ".org" => {
                    *asm_result.program_counter_mut() = match self.resolve_argument(post) {
                        Ok(result) => match result {
                            ValueType::Literal(v) => v,
                            _ => {
                                return Err(AssemblerError::TypeMismatch(
                                    line_num,
                                    "non-literal".to_string(),
                                ));
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    let new_size = asm_result.program_counter() as usize;
                    asm_result.rom_mut().resize(new_size, 0x00);
                }
                unknown => {
                    if unknown.trim_end().ends_with(":") {
                        let label = unknown.trim_end_matches(":");

                        if labels.contains_key(label) {
                            // TODO: Check for unresolved labels
                            match labels.remove_entry(label) {
                                Some((_k, call)) => {
                                    // The target jump address will be the next line
                                    let target = asm_result.rom().len() as u8;
                                    asm_result.rom_mut()[call as usize] = target;

                                    self.add_log(&format!("Storing jump target {:#04X?}", target));
                                }
                                None => {
                                    return Err(AssemblerError::LabelResolution(
                                        line_num,
                                        label.to_string(),
                                    ))
                                }
                            }
                        }
                    } else {
                        eprintln!("Unknown mnemonic: {}", pre);
                    }
                }
            }
        }

        if asm_result.rom().len() % 2 != 0 {
            asm_result.rom_mut().push(0x00);
        }

        self.add_log("---------------------------");
        self.add_log("Assembler completed");

        Ok(asm_result)
    }

    fn resolve_rst(&self, arg: &str) -> Result<(u8, u8, u8), AssemblerError> {
        let (r, s, t) = split_three_args(arg);

        let r = match self.resolve_argument(&r) {
            Ok(v) => match v {
                ValueType::Register(v) => v,
                _ => {
                    return Err(AssemblerError::TypeMismatch(
                        self.line_number,
                        "non-register".to_string(),
                    ));
                }
            },
            Err(e) => {
                return Err(e);
            }
        };

        let s = match self.resolve_argument(&s) {
            Ok(v) => match v {
                ValueType::Register(v) => v,
                _ => {
                    return Err(AssemblerError::TypeMismatch(
                        self.line_number,
                        "non-register".to_string(),
                    ));
                }
            },
            Err(e) => {
                return Err(e);
            }
        };

        let t = match self.resolve_argument(&t) {
            Ok(v) => match v {
                ValueType::Register(v) => v,
                _ => {
                    return Err(AssemblerError::TypeMismatch(
                        self.line_number,
                        "non-register".to_string(),
                    ));
                }
            },
            Err(e) => {
                return Err(e);
            }
        };

        Ok((r, s, t))
    }

    fn register_to_value(&self, reg: &str) -> Result<u8, AssemblerError> {
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
            _ => Err(AssemblerError::UnknownRegister(
                self.line_number,
                reg.to_string(),
            )),
        }
    }

    fn resolve_argument(&self, arg: &str) -> Result<ValueType, AssemblerError> {
        let val = arg.to_lowercase();
        if val.starts_with('r') {
            // Register
            match self.register_to_value(val.as_str()) {
                Ok(v) => {
                    return Ok(ValueType::Register(v));
                }
                Err(e) => return Err(e),
            }
        }

        if val.starts_with('(') && val.ends_with(')') {
            // Memory address
            match numeric_to_value(val.as_str()) {
                Ok(v) => {
                    return Ok(ValueType::Address(v));
                }
                Err(_) => {
                    return Err(AssemblerError::MalformedAddress(self.line_number, val));
                }
            }
        } else if val.starts_with('(') || val.ends_with(')') {
            return Err(AssemblerError::MalformedAddress(self.line_number, val));
        }

        if val.starts_with("0x") || val.starts_with("0b") {
            // Literal
            match numeric_to_value(val.as_str()) {
                Ok(v) => {
                    return Ok(ValueType::Literal(v));
                }
                Err(_) => {
                    return Err(AssemblerError::MalformedNumber(self.line_number, val));
                }
            }
        }

        if val.trim_end().ends_with(':') {
            return Ok(ValueType::Label(val.trim_end_matches(':').to_string()));
        }

        //TODO: labels
        //Ok(ValueType::Literal(0x0))
        Err(AssemblerError::UnknownArgument(self.line_number, val))
    }
}

fn numeric_to_value(num: &str) -> Result<u8, String> {
    let value = num.trim_start_matches('(').trim_end_matches(')');

    let radix = if value.starts_with("0x") {
        16
    } else if value.starts_with("0b") {
        2
    } else {
        return Err(format!("Malformed number: {num}"));
    };

    let prefix = if value.starts_with("0x") {
        "0x"
    } else if value.starts_with("0b") {
        "0b"
    } else {
        return Err(format!("Malformed number: {num}"));
    };

    let value = value.strip_prefix(prefix).unwrap_or(value);
    let value = u8::from_str_radix(value, radix).unwrap_or_default();

    Ok(value)
}

fn split_two_args(args: &str) -> (String, String) {
    let result: Vec<&str> = args.split(',').flat_map(|s| s.split(", ")).collect();
    (result[0].trim().to_string(), result[1].trim().to_string())
}

fn split_three_args(args: &str) -> (String, String, String) {
    let result: Vec<&str> = args.split(',').flat_map(|s| s.split(", ")).collect();
    (
        result[0].trim().to_string(),
        result[1].trim().to_string(),
        result[2].trim().to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::{DEMO_ROM, DEMO_SOURCE};
    use rand::{self, Rng};

    #[test]
    fn ld() {
        let mut asm = Assembler::new();

        let l1 = "ld r0, 0x01".to_owned();
        let result = match asm.assemble(l1) {
            Ok(v) => v,
            Err(e) => {
                println!("{e}");

                let log = asm.log().to_string();
                println!("{log}");
                AssemblerResult::default()
            }
        };
        assert_eq!(result.rom(), [0x20, 0x01]);

        let l2 = "ld r0, 0b00000001".to_owned();
        let result = match asm.assemble(l2) {
            Ok(v) => v,
            Err(e) => {
                println!("{e}");

                let log = asm.log().to_string();
                println!("{log}");
                AssemblerResult::default()
            }
        };
        assert_eq!(result.rom(), [0x20, 0x01]);
    }

    #[test]
    fn ld_0x2() {
        let mut asm = Assembler::new();
        let mut rng = rand::rng();

        let mut program = String::new();
        for i in 0..16 {
            let register = match decimal_to_register_string(i) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Invalid register {e}");
                    "r0".to_owned()
                }
            };

            let value = rng.random::<u8>();
            let value = if rng.random_bool(0.5) {
                format!("{value:#04X?}")
            } else {
                format!("{value:#010b}")
            };
            let inst = format!("ld {register}, {value}\n");
            program.push_str(&inst);
        }

        // TODO: Actually compare values
        let result = match asm.assemble(program) {
            Ok(v) => v,
            Err(e) => {
                println!("{e}");

                let log = asm.log().to_string();
                println!("{log}");
                AssemblerResult::default()
            }
        };
        assert_eq!(result.rom().len(), 32);
    }

    #[test]
    fn split_args_two() {
        let (l, r) = split_two_args("a,b");
        assert_eq!(l, "a");
        assert_eq!(r, "b");

        let (l, r) = split_two_args("a, b");
        assert_eq!(l, "a");
        assert_eq!(r, "b");

        let (l, r) = split_two_args("a , b");
        assert_eq!(l, "a");
        assert_eq!(r, "b");
    }

    #[test]
    fn split_args_three() {
        let (l, m, r) = split_three_args("a,b,c");
        assert_eq!(l, "a");
        assert_eq!(m, "b");
        assert_eq!(r, "c");

        let (l, m, r) = split_three_args("a, b, c");
        assert_eq!(l, "a");
        assert_eq!(m, "b");
        assert_eq!(r, "c");

        let (l, m, r) = split_three_args("a, b,c");
        assert_eq!(l, "a");
        assert_eq!(m, "b");
        assert_eq!(r, "c");

        let (l, m, r) = split_three_args("a,b, c");
        assert_eq!(l, "a");
        assert_eq!(m, "b");
        assert_eq!(r, "c");
    }

    #[test]
    fn demo_program() {
        let mut asm = Assembler::new();
        let result = match asm.assemble(DEMO_SOURCE.to_string()) {
            Ok(v) => v,
            Err(e) => {
                println!("{e}");

                let log = asm.log().to_string();
                println!("{log}");
                AssemblerResult::default()
            }
        };

        assert_eq!(result.rom(), DEMO_ROM);
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
            _ => Err(format!("Invalid register {reg}")),
        }
    }
}
