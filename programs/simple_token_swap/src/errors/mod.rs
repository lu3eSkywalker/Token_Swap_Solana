use anchor_lang::prelude::*;
pub mod errors;
pub use errors::*;

#[error_code]
pub enum TokenSwapError {
    #[msg("Insufficient amount of token A in the liquidity pool")]
    InsufficientTokenA,

    #[msg("Insufficient amount of token B in the liquidity pool")]
    InsufficientTokenB,

    #[msg("Multiplication overflow in calculation error")]
    CalculationError,

    #[msg("Insufficient amount of tokens provided in the liquidity pool")]
    InsufficientLiquidityTokens,
}