use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError, 
    program_error::ProgramError, 
    program_error::PrintProgramError, 
    msg};
use thiserror::Error;

/// Errors that may be returned by the EmotionBattle program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum UserAccountError {
    // Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction
}

impl PrintProgramError for UserAccountError {
    fn print<E>(&self) {
        msg!("USER-ACCOUNT-ERROR: {}", &self.to_string());
    }
}

impl From<UserAccountError> for ProgramError {
    fn from(e: UserAccountError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for UserAccountError {
    fn type_of() -> &'static str {
        "UserAccountError"
    }
}