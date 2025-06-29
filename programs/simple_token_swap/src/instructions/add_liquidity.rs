use crate::state::liquidity_account::LiquidityAccount;
use anchor_lang::prelude::*;
use crate::contexts::Liquidity;
use crate::utils::transfer::deposit_to_vault_token_a;
use crate::utils::transfer::deposit_to_vault_token_b;

pub fn addLiquidity(ctx: Context<Liquidity>, tokenAmount: u64) -> Result<()> {
    deposit_to_vault_token_a(
        &ctx.accounts.user.to_account_info(),
        &ctx.accounts.user_token_account_for_token_a,
        &ctx.accounts.vault_token_a_account,
        &ctx.accounts.token_program,
        tokenAmount,
    )?;

    deposit_to_vault_token_b(
        &ctx.accounts.user.to_account_info(),
        &ctx.accounts.user_token_account_for_token_b,
        &ctx.accounts.vault_token_b_account,
        &ctx.accounts.token_program,
        tokenAmount,
    )?;

    let pda = &mut ctx.accounts.user_pda_account;
    pda.stakedTokenAmount += tokenAmount;

    msg!("Liquidity Added Successfully");

    Ok(())
}