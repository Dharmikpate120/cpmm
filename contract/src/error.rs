// In src/error.rs

use num_derive::FromPrimitive;
use num_enum;
use spl_program_error::spl_program_error;
use thiserror::Error;

#[spl_program_error(hash_error_code_start = 3058088441)]
pub enum AMMError {
    #[error("this account is not owned by current account")]
    IllegalOwner,

    #[error("admin account must be signer!")]
    InvalidSigner,
    
    #[error("Token account is frozen.")]
    TokenAccountFrozen,
    
    #[error("Token account is UnInitialized.")]
    TokenAccountNotInitialized,

    #[error("Insufficient token balance in admin account.")]
    InsufficientTokenBalance,

    #[error("account must be writable!")]
    NotWritable,

    #[error("account already initialized")]
    AccountAlreadyInitialized,

    #[error("account is not initialized")]
    AccountNotInitialized,

    #[error("invalid mint account provided")]
    InvalidMintAccount,
    
    #[error("invalid system program account provided")]
    InvalidSystemProgram,

    #[error("invalid spl token program account provided")]
    InvalidSPLTokenProgram,

    #[error("invalid lp token mint account provided")]
    InvalidLPTokenMintAccount,

    #[error("invalid amm token account provided.")]
    InvalidAMMTokenAccount,

    #[error("invalid PDA account provided.")]
    InvalidPDA,

    #[error("Insufficient Lp Tokens Available")]
    InsufficientLpTokensAvailable,
    
    #[error("Price exceeds the privided Max price")]
    PriceTooHigh
}

// Implement the conversion from `CounterError` to `ProgramError`.
// This allows us to use `?` to propagate our custom errors.
// impl From<StakeError> for ProgramError {
//     fn from(e: CounterError) -> Self {
//         // Log the error message
//         e.print::<Self>();
//         // Convert the enum variant to its `u32` representation
//         // and wrap it in `ProgramError::Custom`.
//         ProgramError::Custom(e as u32)
//     }
// }

// impl<T> PrintProgramError<T> for CounterError
// where
//     T: 'static + std::error::Error + DecodeError<T> + FromPrimitive,
// {
//     fn print(&self) {
//         solana_program::msg!("Error: {}", self);
//     }
// }
