pub const ASM_SYNTAX: &str = r#"ld dest,src - Load a value from src into dest

adds r,s,t - Adds registers s and t as two's compliment, stores result in register r.

addf r,s,t - Adds registers s and t as floating point, stores result in register r.

or r,s,t - OR the bit patterns in registers s and t, stores the result in r.

and r,s,t - AND the bit patterns in registers s and t, stores the result in register r.

xor r,s,t - XOR the bit patterns in s and t, stores the result in register r.

rot r,x - Rotates the bit pattern in register r to the right x times.

jp r,<label> - Jump to the label if register r is equal to register 0

halt - Stop program execution"#;
