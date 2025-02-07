// TODO: Remove
#![allow(warnings)]

#[derive(Debug, Default)]
pub struct Assembler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //#[ignore]
    fn test_compile() {
        const TEST_RESULT: &[u8] = &[
            0x20, 0x00, 0x25, 0xFF, 0x14, 0x44, 0xB4, 0x0A, 0x35, 0x46, 0xC0, 0x00,
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

        //assert_eq!(result, TEST_RESULT);
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

                        let (lhs, rhs) = if parts.len() < 3 {
                            // There is no space between arguments, we can split at the comma
                            let result: Vec<_> = parts[1].split(",").collect();

                            (result[0], result[1])
                        } else {
                            // The left side will have a trailing comma if a space is used
                            let result: Vec<_> = parts[1].split(",").collect();
                            (result[0], parts[2])
                        };
                        eprintln!("lhs_str: {}\nrhs_str: {}", lhs, rhs);

                        let lhs = match self.resolve_argument(lhs) {
                            Ok(v) => v,
                            Err(e) => {
                                // TODO: Fix this
                                println!("Fix this: {:?}", e);
                                ValueType::Literal(0x00)
                            }
                        };
                        let rhs = match self.resolve_argument(rhs) {
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
                                }
                                ValueType::Address(a) => {
                                    //0x1RXY
                                }
                                ValueType::Literal(l) => {
                                    //0x2RXY
                                }
                                ValueType::Label(l) => {
                                    // TODO: Fix this
                                    let msg = format!("Cannot store {} in register", l);
                                    println!("{}", msg);
                                    continue;
                                }
                            },
                            ValueType::Address(_) => {
                                //0x3RXY
                            }
                            _ => {
                                // TODO: Fix this
                                println!("Failed to determine ld type");
                                continue;
                            }
                        };
                    }
                    "halt" => {
                        result.push(0xC0);
                        result.push(0x00);
                        eprintln!("Pushing 0xC000");
                    }
                    _ => {
                        // TODO: Handle labels
                        eprintln!("Unknown part: {}", parts[0]);
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
}
