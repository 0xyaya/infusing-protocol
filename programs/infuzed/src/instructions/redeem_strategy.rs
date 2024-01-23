use anchor_lang::prelude::*;

use crate::state::{Controller, StrategyAccount};

#[derive(Accounts)]
pub struct RedeemStrategy<'info> {
    #[account(mut, address = strategy.authority)] // address = strategy.authority
    pub signer: Signer<'info>, // Signer == Strategy.authority
    #[account(mut, seeds = [b"controller"], bump)]
    pub controller: Account<'info, Controller>,
    /// CHECK: This account is not read or written
    #[account(mut, address = strategy.redeem_address)]
    pub redeem_address: AccountInfo<'info>,
    #[account(mut)]
    pub strategy: Account<'info, StrategyAccount>,
    pub system_program: Program<'info, System>
}

pub fn redeem_strategy_handler(ctx:Context<RedeemStrategy>) -> Result<()> {
    if ctx.accounts.strategy.lamports > 0 {
        let transfer_amount: u64 = ctx.accounts.strategy.lamports;

        **ctx.accounts.strategy.to_account_info().try_borrow_mut_lamports()? -= transfer_amount;

        **ctx.accounts.redeem_address.to_account_info().try_borrow_mut_lamports()? += transfer_amount;

        ctx.accounts.strategy.lamports = 0;
    }

    Ok(())
}