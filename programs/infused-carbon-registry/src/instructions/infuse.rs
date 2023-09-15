use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state::InfusedAccount;

#[derive(Accounts)]
pub struct Infuse<'info> {
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = signer, space = 8)]
    pub infused_account: Account<'info, InfusedAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn infuse_handler(ctx: Context<Infuse>, amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let infused_account = &mut ctx.accounts.infused_account;
    infused_account.carbon_score = infused_account.carbon_score + amount;
    infused_account.last_infused_time = clock.unix_timestamp as u64;
    Ok(())
}
