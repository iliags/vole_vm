// TODO: Remove
#![allow(warnings)]

use std::{collections::HashMap, result};

#[derive(Debug, Default)]
pub struct Assembler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //#[ignore]
    fn test_compile() {
        const TEST_RESULT: &[u8] = &[
            0x20, 0x00, // Load 0x00 into r0
            0x25, 0xFF, // Load 0xFF into r5
            0x14, 0x44, // Load mem 0x44 into r4
            0xB4, 0x0A, // If r4 == r0, jump to mem 0x0A (skip next line)
            0x25, 0x01, // load 0x01 into r5
            0x35, 0x46, // Store r5 into mem 0x46
            0xC0, 0x00, // Quit
        ];

        // TODO: Add test code for three argument operands
        const TEST_SOURCE: &str = "
ld r0,0x00        ; Load 0x00 into r0
LD R5, 0xFF        ; Load 0xFF into r5
Ld r4, (0x44)      ; Load mem 0x44 into r4

jp r4, continue    ; If r4 == r0, jump to continue
lD r5, 0x01        ; Load 0x01 into r5

continue:
    ld (0x46), r5  ; Store r5 into mem 0x46
    halt           ; Quit";

        //println!("Input:\n{}\n", TEST_SOURCE);

        let asm = Assembler::new();
        let result = asm.assemble(TEST_SOURCE.to_string());

        eprintln!("---------------------------");
        eprintln!("Output:\n{:#04X?}", result);

        assert_eq!(result, TEST_RESULT);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ValueType {
    Register(u8),
    Address(u8),
    Literal(u8),
    Label(String),
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn assemble(&self, source_code: String) -> Vec<u8> {
        let mut result = Vec::new();
        // <Label, Calling Address>
        let mut labels: HashMap<String, u8> = HashMap::new();

        let lines: Vec<&str> = source_code.split_terminator("\n").collect();
        eprintln!("Line count: {}\n", lines.len());

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

            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() > 0 {
                match parts[0].to_lowercase().as_str() {
                    "ld" => {
                        if parts.len() < 2 {
                            // TODO: Fix this
                            println!("Not enough arguments: {:?}", parts);
                            continue;
                        }

                        let (lhs, rhs) = self.split_two_args(&parts);
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

                        // TODO: Figure out which opcode to use based on arguments provided
                        let op = match lhs {
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
                                let r = match rhs {
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
                    }
                    "addf" => {
                        // TODO: Not tested
                        //0x6RST
                    }
                    "or" => {
                        // TODO: Not tested
                        //0x7RST
                    }
                    "and" => {
                        // TODO: Not tested
                        //0x8RST
                    }
                    "xor" => {
                        // TODO: Not tested
                        //0x9RST
                    }
                    "rot" => {
                        // TODO: Not tested
                        //0xAR0X
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
                        let (lhs, rhs) = self.split_two_args(&parts);
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
                    unknown => {
                        // TODO: Handle labels
                        if unknown.trim_end().ends_with(":") {
                            let label = unknown.trim_end_matches(":");

                            if labels.contains_key(label) {
                                eprintln!("Labels");

                                match labels.get_key_value(label) {
                                    Some((k, call)) => {
                                        // The target jump address will be the next line
                                        let target = result.len() as u8;
                                        result[*call as usize] = target;

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
                            eprintln!("Unknown part: {}", parts[0]);
                        }
                    }
                }
            }
        }

        result
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
        let value = u8::from_str_radix(&value, radix).unwrap_or_default();

        Ok(value)
    }

    fn split_two_args(&self, args: &[&str]) -> (String, String) {
        if args.len() < 3 {
            // There is no space between arguments, we can split at the comma
            let result: Vec<_> = args[1].split(",").collect();

            return (result[0].to_string(), result[1].to_string());
        } else {
            // The left side will have a trailing comma if a space is used
            let result: Vec<_> = args[1].split(",").collect();
            return (result[0].to_owned(), args[2].to_string());
        };
    }

    fn split_three_args(&self, args: &[&str]) -> (String, String, String) {
        // TODO
        ("a".to_string(), "b".to_string(), "c".to_string())
    }
}
