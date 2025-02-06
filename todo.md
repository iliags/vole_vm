# TODO

- Export
  - CSV
  - Markdown table
- Execution log

## Assembler

Due to the simplicity of this assembly, a custom parser will be used.

Pseudocode:

- For each line in the script
  - If it starts with a `;`, continue
  - If the line starts with 2-4 characters followed by a space or terminates (for `halt`)
    - Check for keywords
  - If the line has characters followed by a `:`
    - Handle labels

After each successful line parsed, push the opcodes to the byte list. If a label needs to be resolved, mark it for later.

If a label is encountered while parsing `jp`, push it to a hashmap for later comparison.

Registers are annotated with `r0` through `rf`, memory addresses are annotated by a hex/binary value in parentheses, and literals are hex/binary values with no extra punctuation.

Hex values are always prefixed with `0x` and binary values are always prefixed with `0b`.

An assembler status box needs to be added to print errors.
