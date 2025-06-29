use crate::contexts::TokenSwap;
use crate::errors::errors::TokenSwapError;
use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, InitializeAccount, Mint, Token, TokenAccount, TokenAccount as SPLTokenAccount, Transfer,
};
use crate::utils::amm::amm_calculation;
use crate::utils::transfer::deposit_to_vault_token_a;
use crate::utils::transfer::send_token_b_from_token_vault_to_user;

pub fn swap_a_for_b(ctx: Context<TokenSwap>, amountOfTokenA: u64) -> Result<()> {
    let token_a_quantity = ctx.accounts.vault_token_a_account.amount;
    let token_b_quantity = ctx.accounts.vault_token_b_account.amount;

    let (x) = amm_calculation(token_a_quantity, token_b_quantity)?;

    let tokenBtoSend = (x / ((token_a_quantity as u128) + (amountOfTokenA as u128)))
        .try_into()
        .map_err(|_| error!(TokenSwapError::CalculationError))?;

    let tokenBtoGive = (token_b_quantity as u128)
        .checked_sub(tokenBtoSend)
        .ok_or(error!(TokenSwapError::CalculationError))?;

    require!(
        tokenBtoGive <= token_b_quantity as u128,
        TokenSwapError::InsufficientTokenB
    );

    // Transfer Token A from user to Token Vault
    deposit_to_vault_token_a(
        &ctx.accounts.user.to_account_info(),
        &ctx.accounts.user_token_account_for_token_a,
        &ctx.accounts.vault_token_a_account,
        &ctx.accounts.token_program,
        amountOfTokenA,
    )?;

    // Convert to u64 before transferring
    let tokenBtoGive: u64 = tokenBtoGive
        .try_into()
        .map_err(|_| error!(TokenSwapError::CalculationError))?;

    // Transfer Token B from Token Vault to user

    send_token_b_from_token_vault_to_user(
        &ctx.accounts.mint_b,
        &ctx.accounts.vault_auth_b,
        &ctx.accounts.vault_token_b_account,
        &ctx.accounts.user_token_account_for_token_b,
        &ctx.accounts.token_program,
        ctx.bumps.vault_auth_b,
        tokenBtoGive,
    )?;

    Ok(())
}
