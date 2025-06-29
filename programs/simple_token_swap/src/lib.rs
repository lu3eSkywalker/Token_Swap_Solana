use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod utils;
pub mod contexts;
pub use contexts::accounts::*;

use instructions::*;

declare_id!("D2NkS3RjEAizQCz8YLA8sK25MkXSDmwDRK4b8FkRQYj5");

#[program]
pub mod Simple_Token_Swap {
    use super::*;

    pub fn initialize_vault_token_a(ctx: Context<InitializeVaultTokenA>) -> Result<()> {
        instructions::initialize::initialize_vault_token_a(ctx)
    }

    pub fn initialize_vault_token_b(ctx: Context<InitializeVaultTokenB>) -> Result<()> {
        instructions::initialize::initialize_vault_token_b(ctx)
    }

    pub fn initialize_user_liquidity_account(ctx: Context<InitializeUserLiquidityAccount>) -> Result<()> {
        instructions::initialize::initialize_user_liquidity_account(ctx)
    }

    pub fn addLiquidity(ctx: Context<Liquidity>, tokenAmount: u64) -> Result<()> {
        instructions::add_liquidity::addLiquidity(ctx, tokenAmount)
    }

    pub fn removeLiquidity(ctx: Context<Liquidity>, tokenAmount: u64) -> Result<()> {
        instructions::remove_liquidity::removeLiquidity(ctx, tokenAmount)
    }

    pub fn swap_b_for_a(ctx: Context<TokenSwap>, amountOfTokenB: u64) -> Result<()> {
        instructions::swap_b::swap_b_for_a(ctx, amountOfTokenB)
    }

    pub fn swap_a_for_b(ctx: Context<TokenSwap>, amountOfTokenA: u64) -> Result<()> {
        instructions::swap_a::swap_a_for_b(ctx, amountOfTokenA)
    }
}