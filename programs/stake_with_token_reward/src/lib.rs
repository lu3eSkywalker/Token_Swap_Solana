use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, InitializeAccount, Mint, Token, TokenAccount, TokenAccount as SPLTokenAccount, Transfer,
};

declare_id!("D2NkS3RjEAizQCz8YLA8sK25MkXSDmwDRK4b8FkRQYj5");

#[program]
pub mod Simple_Token_Swap {
    use super::*;

    pub fn initialize_vault_token_a(ctx: Context<InitializeVaultTokenA>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_vault_token_b(ctx: Context<InitializeVaultTokenB>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_user_liquidity_account(
        ctx: Context<InitializeUserLiquidityAccount>,
    ) -> Result<()> {
        msg!("Liquidity account created successfully");

        let pda = &mut ctx.accounts.user_pda_account;
        pda.Owner = ctx.accounts.user.key();
        pda.stakedTokenAmount = 0;

        Ok(())
    }

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

    pub fn swap_b_for_a(ctx: Context<TokenSwap>, amountOfTokenB: u64) -> Result<()> {
        let token_a_quantity = ctx.accounts.vault_token_a_account.amount;
        let token_b_quantity = ctx.accounts.vault_token_b_account.amount;

        let (x) = amm_calculation(token_a_quantity, token_b_quantity)?;

        let tokenAToSend = (x / ((token_b_quantity as u128) + (amountOfTokenB as u128)))
            .try_into()
            .map_err(|_| error!(TokenSwapError::CalculationError))?;

        let tokenAtoGive = (token_a_quantity as u128)
            .checked_sub(tokenAToSend)
            .ok_or(error!(TokenSwapError::CalculationError))?;

        require!(
            tokenAtoGive <= token_a_quantity as u128,
            TokenSwapError::InsufficientTokenA
        );

        // Transfer Token B from user to Token Vault
        deposit_to_vault_token_b(
            &ctx.accounts.user.to_account_info(),
            &ctx.accounts.user_token_account_for_token_b,
            &ctx.accounts.vault_token_b_account,
            &ctx.accounts.token_program,
            amountOfTokenB,
        )?;

        // Convert to u64 before transferring
        let tokenAtoGive: u64 = tokenAtoGive
            .try_into()
            .map_err(|_| error!(TokenSwapError::CalculationError))?;


        // Transfer Token A from Token Vault to user
        send_token_a_from_token_vault_to_user(
            &ctx.accounts.mint_a,
            &ctx.accounts.vault_auth_a,
            &ctx.accounts.vault_token_a_account,
            &ctx.accounts.user_token_account_for_token_a,
            &ctx.accounts.token_program,
            ctx.bumps.vault_auth_a,
            tokenAtoGive
        )?;

        Ok(())
    }

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
}

fn amm_calculation(token_a_quantity: u64, token_b_quantity: u64) -> Result<(u128)> {
    let token_a_128 = token_a_quantity as u128;
    let token_b_128 = token_b_quantity as u128;

    let x = token_a_128
        .checked_mul(token_b_128)
        .ok_or_else(|| error!(TokenSwapError::CalculationError))?;

    Ok(x)
}

// This function deposits the Token A from user to the token vault
fn deposit_to_vault_token_a<'info>(
    user: &AccountInfo<'info>,
    user_token_account_for_token_a: &Account<'info, TokenAccount>,
    vault_token_a_account: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: user_token_account_for_token_a.to_account_info(),
        to: vault_token_a_account.to_account_info(),
        authority: user.clone(),
    };

    let cpi_program = token_program.to_account_info();

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

// This function deposits the Token B from user to the token vault
fn deposit_to_vault_token_b<'info>(
    user: &AccountInfo<'info>,
    user_token_account_for_token_b: &Account<'info, TokenAccount>,
    vault_token_b_account: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: user_token_account_for_token_b.to_account_info(),
        to: vault_token_b_account.to_account_info(),
        authority: user.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

// This function sends token A from Token Vault to User
fn send_token_a_from_token_vault_to_user<'info>(
    mint_a: &Account<'info, Mint>,
    vault_auth_a: &AccountInfo<'info>,
    vault_token_a_account: &Account<'info, TokenAccount>,
    user_token_account_for_token_a: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    vault_auth_a_bump: u8,
    tokenAmount: u64,
) -> Result<()> {
    let mint_a_key = mint_a.key();

    let seeds = &[
        b"vaultTokenA",
        mint_a_key.as_ref(),
        &[vault_auth_a_bump],
    ];

    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: vault_token_a_account.to_account_info(),
        to: user_token_account_for_token_a.to_account_info(),
        authority: vault_auth_a.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::transfer(cpi_ctx, tokenAmount);

    Ok(())
}

// This function sends token B from Token Vault to User
fn send_token_b_from_token_vault_to_user<'info>(
    mint_b: &Account<'info, Mint>,
    vault_auth_b: &AccountInfo<'info>,
    vault_token_b_account: &Account<'info, TokenAccount>,
    user_token_account_for_token_b: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    vault_auth_b_bump: u8,
    tokenAmount: u64,
) -> Result<()> {
    let mint_b_key = mint_b.key();

    let seeds = &[
        b"vaultTokenB",
        mint_b_key.as_ref(),
        &[vault_auth_b_bump]
    ];

    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: vault_token_b_account.to_account_info(),
        to: user_token_account_for_token_b.to_account_info(),
        authority: vault_auth_b.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::transfer(cpi_ctx, tokenAmount);

    Ok(())
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

#[account]
pub struct LiquidityAccount {
    pub Owner: Pubkey,
    pub stakedTokenAmount: u64,
}

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
