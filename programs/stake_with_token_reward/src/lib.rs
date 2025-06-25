use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, InitializeAccount, TokenAccount as SPLTokenAccount};

declare_id!("D5NFgx6v3h2uZbrRExZ3Bq6NQSrwoTvsQK6U38PbSLhC");

#[program]
pub mod Simple_Token_Swap {
    use super::*;

    pub fn initialize_vault_Token_A(ctx: Context<InitializeVaultTokenA>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_to_vault(ctx: Context<DepositToVaultTokenA>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn initialize_vault_Token_B(ctx: Context<InitializeVaultTokenB>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_to_vault_Token_B(ctx: Context<DepositToVaultTokenB>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

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
pub struct DepositToVaultTokenA<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vaultTokenA", mint.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
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
pub struct DepositToVaultTokenB<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vaultTokenB", mint.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}