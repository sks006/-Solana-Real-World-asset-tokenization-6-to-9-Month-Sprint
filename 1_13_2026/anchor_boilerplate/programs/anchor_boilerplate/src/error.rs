use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Account not initialized")]
    UninitializedAccount,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,  // ADD THIS
}