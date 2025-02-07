#[derive(Debug, Default)]
pub struct Assembler;

impl Assembler {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn assemble(&self, source_code: String) -> Vec<u8> {
        let mut result = Vec::new();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_compile() {
        const TEST_SOURCE: &str = "
ld r0, 0x00        ; Load 0x00 into r0
ld r5, 0xFF        ; Load 0xFF into r5
ld r4, (0x44)      ; Load mem 0x44 into r4

jp r4, continue    ; If r4 == r0, jump to continue
ld r5, 0x01        ; Load 0x01 into r5

continue:
    ld (0x46), r5  ; Store r5 into mem 0x46
    halt           ; Quit";

        println!("Input:\n{}\n", TEST_SOURCE);

        let asm = Assembler::new();
        let result = asm.assemble(TEST_SOURCE.to_string());

        println!("Output:\n{:?}", result);
    }
}
