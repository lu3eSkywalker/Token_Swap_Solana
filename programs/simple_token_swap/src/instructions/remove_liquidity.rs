use crate::state::liquidity_account::LiquidityAccount;
use anchor_lang::prelude::*;
use crate::contexts::Liquidity;
use crate::errors::errors::TokenSwapError;
use crate::utils::transfer::send_token_a_from_token_vault_to_user;
use crate::utils::transfer::send_token_b_from_token_vault_to_user;

pub fn removeLiquidity(ctx: Context<Liquidity>, tokenAmount: u64) -> Result<()> {
    let userProvidedLiquidity = &mut ctx.accounts.user_pda_account;

    require!(
        userProvidedLiquidity.stakedTokenAmount >= tokenAmount,
        TokenSwapError::InsufficientLiquidityTokens
    );

    send_token_a_from_token_vault_to_user(
        &ctx.accounts.mint_a,
        &ctx.accounts.vault_auth_a,
        &ctx.accounts.vault_token_a_account,
        &ctx.accounts.user_token_account_for_token_a,
        &ctx.accounts.token_program,
        ctx.bumps.vault_auth_a,
        tokenAmount,
    )?;

    send_token_b_from_token_vault_to_user(
        &ctx.accounts.mint_b,
        &ctx.accounts.vault_auth_b,
        &ctx.accounts.vault_token_b_account,
        &ctx.accounts.user_token_account_for_token_b,
        &ctx.accounts.token_program,
        ctx.bumps.vault_auth_b,
        tokenAmount,
    )?;

    userProvidedLiquidity.stakedTokenAmount -= tokenAmount;

    Ok(())
}