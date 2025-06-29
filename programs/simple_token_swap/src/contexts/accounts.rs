use crate::state::liquidity_account::LiquidityAccount;
use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, InitializeAccount, Mint, Token, TokenAccount, TokenAccount as SPLTokenAccount, Transfer,
};

#[derive(Accounts)]
#[instruction()]
pub struct InitializeVaultTokenA<'info> {
    #[account(
        init_if_needed,
        seeds = [b"vaultTokenA", mint.key().as_ref()],
        bump,
        payer = payer,
        token::mint = mint,
        token::authority = vault_auth
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA will be the authority for the vault PDA
    #[account{
        seeds = [b"vaultTokenA", mint.key().as_ref()],
        bump
    }]
    pub vault_auth: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeVaultTokenB<'info> {
    #[account(
        init_if_needed,
        seeds = [b"vaultTokenB", mint.key().as_ref()],
        bump,
        payer = payer,
        token::mint = mint,
        token::authority = vault_auth
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA will be the authority for the vault PDAs
    #[account{
        seeds = [b"vaultTokenB", mint.key().as_ref()],
        bump
    }]
    pub vault_auth: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeUserLiquidityAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"liquidityPDA", user.key().as_ref()],
        bump
    )]
    pub user_pda_account: Account<'info, LiquidityAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Liquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"liquidityPDA", user.key().as_ref()],
        bump
    )]
    pub user_pda_account: Account<'info, LiquidityAccount>,

    #[account(mut)]
    pub user_token_account_for_token_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account_for_token_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vaultTokenA", mint_a.key().as_ref()],
        bump
    )]
    pub vault_token_a_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vaultTokenB", mint_b.key().as_ref()],
        bump
    )]
    pub vault_token_b_account: Account<'info, TokenAccount>,

    /// CHECK: This is just a signer PDA, no data
    #[account(
        seeds = [b"vaultTokenA", mint_a.key().as_ref()],
        bump
    )]
    pub vault_auth_a: AccountInfo<'info>,

    /// CHECK: This is just a signer PDA, no data
    #[account(
        seeds = [b"vaultTokenB", mint_b.key().as_ref()],
        bump
    )]
    pub vault_auth_b: AccountInfo<'info>,

    pub mint_a: Account<'info, Mint>,

    pub mint_b: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TokenSwap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account_for_token_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account_for_token_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vaultTokenA", mint_a.key().as_ref()],
        bump
    )]
    pub vault_token_a_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vaultTokenB", mint_b.key().as_ref()],
        bump
    )]
    pub vault_token_b_account: Account<'info, TokenAccount>,

    /// CHECK: This is just a signer PDA, no data
    #[account(
        seeds = [b"vaultTokenA", mint_a.key().as_ref()],
        bump
    )]
    pub vault_auth_a: AccountInfo<'info>,

    /// CHECK: This is just a signer PDA, no data
    #[account(
        seeds = [b"vaultTokenB", mint_b.key().as_ref()],
        bump
    )]
    pub vault_auth_b: AccountInfo<'info>,

    pub mint_a: Account<'info, Mint>,

    pub mint_b: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}
