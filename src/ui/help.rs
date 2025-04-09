pub const ASM_SYNTAX: &str = r".org value - Offset the program by value, emulator specific instruction.

ld dest,src - Load a value from src into dest. Src can be a memory address, register, or value. Dest can be a memory address or register.

adds r,s,t - Adds registers s and t as two's compliment, stores result in register r.

addf r,s,t - Adds registers s and t as floating point, stores result in register r.

or r,s,t - OR the bit patterns in registers s and t, stores the result in r.

and r,s,t - AND the bit patterns in registers s and t, stores the result in register r.

xor r,s,t - XOR the bit patterns in s and t, stores the result in register r.

rot r,x - Rotates the bit pattern in register r to the right x times.

jp r,<label> - Jump to the label if register r is equal to register 0

halt - Stop program execution

r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, ra, rb, rc, rd, re, rf - Registers.

Memory addresses are a hex or binary value surrounded by parentheses, values are hex (prefix 0x) or binary (prefix 0b) patterns.";
