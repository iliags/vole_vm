#[derive(Debug, Default)]
pub struct Assembler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //#[ignore]
    fn test_compile() {
        #[allow(dead_code)]
        const TEST_RESULT: &[u8] = &[
            0x20, 0x00, 0x25, 0xFF, 0x14, 0x44, 0xB4, 0x0A, 0x35, 0x46, 0xC0, 0x00,
        ];
        const TEST_SOURCE: &str = "
ld r0, 0x00        ; Load 0x00 into r0
ld r5, 0xFF        ; Load 0xFF into r5
ld r4, (0x44)      ; Load mem 0x44 into r4

jp r4, continue    ; If r4 == r0, jump to continue
ld r5, 0x01        ; Load 0x01 into r5

continue:
    ld (0x46), r5  ; Store r5 into mem 0x46
    halt           ; Quit";

        //println!("Input:\n{}\n", TEST_SOURCE);

        let asm = Assembler::new();
        let result = asm.assemble(TEST_SOURCE.to_string());

        eprintln!("---------------------------");
        eprintln!("Output:\n{:?}", result);

        //assert_eq!(result, TEST_RESULT);
    }
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn assemble(&self, source_code: String) -> Vec<u8> {
        #[allow(unused_mut)]
        let mut result = Vec::new();

        let lines: Vec<&str> = source_code.split_terminator("\n").collect();
        eprintln!("Line count: {}\n", lines.len());

        for (line_num, line) in lines.iter().enumerate() {
            eprintln!("---------------------------");
            eprintln!("{:?}: {}", line_num, line);

            // Skip empty lines and comment lines
            if line.starts_with(";") || line.is_empty() {
                eprintln!("Skipping empty or comment");
                continue;
            }

            // Split off end of line comments
            //let items: Vec<_> = line.split(";").collect();
            let line = line.trim();
            // Trim everything after ";"
            let line = line.split_once(';').map_or(line, |(before, _)| before);
            let line = line.trim_end();

            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() > 0 {
                match parts[0] {
                    _ => {
                        eprintln!("Unknown part: {}", parts[0]);
                    }
                }
            }

            /*
            if items.len() > 0 {
                let operand = items[0].trim();
                //eprintln!("operand: {}", operand);
                if operand.starts_with("")
            }
             */
        }

        result
    }
}
