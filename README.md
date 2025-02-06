# Vole Virtual Machine

[Live version here](https://iliags.github.io/vole_vm/)

An implementation of the virtual machine as specified in *Computer Science: An Overview (13th edition)* by Glenn Brookshear and Dennis Brylow.

## Building

### Desktop

>[!NOTE]
> Linux users may need to run `sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

- Install [rust](https://www.rust-lang.org/tools/install)
- Open a terminal in the repository location
  - Most operating systems have an option to open a terminal in the current folder when right clicking in the file browser.
- Enter ```cargo build --release``` to build but not execute, ```cargo run --release``` to build and execute
  - The ```--release``` flag can be removed to build the development version
- The binary will be located under `target/release/` for release builds and `target/debug/` for development builds

### WASM

- Install [rust](https://www.rust-lang.org/tools/install)
- Install trunk using ```cargo binstall trunk``` (recommended on Windows) or ```cargo install trunk```
  - This may take a while
- Open a terminal in the repository location:
  - Build only ```trunk build --release```
  - Build and start a local web server ```trunk serve --release```
- Trunk will output the files in the `dist` folder which is created if it doesn't exist.

## Specification

> Appendix C
> Let us say that our Vole computer has 16 general-purpose registers numbered 0x0 through 0xF. Each register is one byte (eight bits) long. For identifying registers within instructions, each register is assigned the unique four-bit pattern that represents its register number. Thus, register 0x0 is identified by binary 0000, and register 0x4 is identified by binary 0100. There are 256 cells in the Vole's main memory. Each cell is assigned a unique address consisting of an integer in the range of 0 to 255. An address can therefore be represented by a pattern of eight bits ranging from 00000000 to 11111111 (or a hexadecimal value in the range of 0x00 to 0xFF).
>
> Floating-point values are assumed to be stored in an eight-bit format discussed
in Section 1.7 and summarized in Figure 1.24.
>
> Each Vole machine instruction is two bytes long. The first 4 bits provide the op-code; the last 12 bits make up the operand field. The table that follows lists the instructions in hexadecimal notation together with a short description of each. The letters R, S, and T are used in place of hexadecimal digits in those fields representing a register identifier that varies depending on the particular application of the instruction. The letters X and Y are used in lieu of hexadecimal digits in variable fields not representing a register.

| Op-code | Operand | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|---------|---------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0x1     | RXY     | LOAD the register R with the bit pattern found in the memory cell whose address is XY. Example: 0x14A3 would cause the contents of the memory cell located at address 0xA3 to be placed in register 0x4.                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 0x2     | RXY     | LOAD the register R with the bit pattern XY. Example: 0x20A3 would cause the value 0xA3 to be placed in register 0.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| 0x3     | RXY     | STORE the bit pattern found in register R in the memory cell whose address is XY. Example: 0x35B1 would cause the contents of register 0x5 to be placed in the memory cell whose address is 0xB1.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| 0x4     | 0RS     | MOVE the bit pattern found in register R to register S. Example: 0x40A4 would cause the contents of register 0xA to be copied into register 0x4.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 0x5     | RST     | ADD the bit patterns in registers S and T as though they were two’s complement representations and leave the result in register R. Example: 0x5726 would cause the binary values in registers 0x2 and 0x6 to be added and the sum placed in register 0x7.                                                                                                                                                                                                                                                                                                                                                                                                                         |
| 0x6     | RST     | ADD the bit patterns in registers S and T as though they represented values in floating-point notation and leave the floating-point result in register R. Example: 0x634E would cause the values in registers 0x4 and 0xE to be added as floating-point values and the result to be placed in register 0x3.                                                                                                                                                                                                                                                                                                                                                                       |
| 0x7     | RST     | OR the bit patterns in registers S and T and place the result in register R. Example: 0x7CB4 would cause the result of ORing the contents of registers 0xB and 0x4 to be placed in register 0xC.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 0x8     | RST     | AND the bit patterns in registers S and T and place the result in register R. Example: 0x8045 would cause the result of ANDing the contents of registers 0x4 and 0x5 to be placed in register 0x0.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| 0x9     | RST     | XOR the bit patterns in registers S and T and place the result in register R. Example: 0x95F3 would cause the result of XORing the contents of registers 0xF and 0x3 to be placed in register 0x5.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| 0xA     | R0X     | ROTATE the bit pattern in register R one bit to the right X times. Each time, place the bit that started at the loworder end at the high-order end. Example: 0xA403 would cause the contents of register 0x4 to be rotated 3 bits to the right in a circular fashion.                                                                                                                                                                                                                                                                                                                                                                                                             |
| 0xB     | RXY     | JUMP to the instruction located in the memory cell at address XY if the bit pattern in register R is equal to the bit pattern in register number 0. Otherwise, continue with the normal sequence of execution. (The jump is implemented by copying XY into the program counter during the execute phase.) Example: 0xB43C would first compare the contents of register 0x4 with the contents of register 0x0. If the two were equal, the pattern 0x3C would be placed in the program counter so that the next instruction executed would be the one located at that memory address. Otherwise, nothing would be done and program execution would continue in its normal sequence. |
| 0xC     | 000     | HALT execution. Example: 0xC000 would cause program execution to stop.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

> Section 1.7
> We first designate the high-order bit of the byte as the sign bit. Once again, a 0 in the sign bit will mean that the value stored is nonnegative, and a 1 will mean that the value is negative. Next, we divide the remaining 7 bits of the byte into two groups, or fields: the exponent field and the mantissa field. Let us designate the 3 bits following the sign bit as the exponent field and the remaining 4 bits as the mantissa field.

### Specification Notes

The specification has the machine implement aspects of knowledge which is covered in the book, however it doesn't specify concrete types for the memory and registers. Based solely on the specification, signed 8-bit integers would be the ideal type for registers and memory values, however this doesn't work in practice. The floating point operation could be implemented manually, however storing the result into the register would likely cause information loss or an incorrect result when being read back.

## Assembler (WIP)

Although not part of the specification, there are so few operations that a simple assembly language is implemented. It's based on the Z80 assembly language since it's one of the more widely known variants that is being used in the homebrew scene.

> [!IMPORTANT]
> Hexadecimal numbers must be prefixed with `0x` and binary with `0b`.

- `ld dest, src` - Load a value from src into dest.
  - Combines the LOAD, STORE, and MOV op-codes.
  - `src` can be a register, memory address, or value
  - `dest` can be a register or memory address
- `adds r, s, t` - Adds registers `s` and `t` as two's compliment, stores the result in register `r`.
- `addf r,s,t` - Adds registers `s` and `t` as floating point, stores the result in register `r`.
- `or r,s,t` - OR the bit patterns in `s` and `t`, stores the result in register `r`.
- `and r,s,t` - AND the bit patterns in `s` and `t`, stores the result in register `r`.
- `xor r,s,t` - XOR the bit patterns in `s` and `t`, stores the result in register `r`.
- `rot r,x` - Rotate the bit pattern in register `r` one bit to the right `x` times. Each time, placing the bit that started at the low order end at the high-order end.
- `jp r,label` - Jump to the label if register `r` is equal to `r0`
- `halt` - Stop program execution

### Registers

Registers are labeled as follows:

- `r0`
- `r1`
- `r2`
- `r3`
- `r4`
- `r5`
- `r6`
- `r7`
- `r8`
- `r9`
- `ra`
- `rb`
- `rc`
- `rd`
- `re`
- `rf`

There is no support for combining two registers for 16-bit values.

### Memory Access

Memory is accessed by placing the address number in parentheses.

Example:

```asm
; The following lines are equivalent, load memory address 254 into register 0
ld r0,(0xFE)
ld r0,(254)
ld r0,(0b11111110)
```

## Attribution

Mouse icon by [ikoiku](https://ikoiku.itch.io/16-x-16-pixel-art-character-mouse), licensed under [Creative Commons Attribution-NoDerivatives 4.0](http://creativecommons.org/licenses/by-nd/4.0/).
