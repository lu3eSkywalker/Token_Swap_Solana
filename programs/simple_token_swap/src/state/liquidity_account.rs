use anchor_lang::prelude::*;

#[account]
pub struct LiquidityAccount {
    pub Owner: Pubkey,
    pub stakedTokenAmount: u64,
}