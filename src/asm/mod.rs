pub mod asm_result;
pub mod assembler;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssemblerError {
    #[error("Malformed memory address '{1}' at line {0}")]
    MalformedAddress(usize, String),

    #[error("Unknown register '{1}' at line {0}")]
    UnknownRegister(usize, String),

    #[error("Malformed number '{1}' at line {0}")]
    MalformedNumber(usize, String),

    #[error("Type mismatch '{1}' at line {0}")]
    TypeMismatch(usize, String),

    #[error("{1} at line {0}")]
    LoadOpFail(usize, String),

    #[error("Error resolving label '{1}' at line {0}")]
    LabelResolution(usize, String),

    #[error("Unknown argument '{1}' at line {0}")]
    UnknownArgument(usize, String),
}
