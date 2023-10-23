use crate::state::{Controller, Strategy};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RegisterStrategy<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [ b"controller"], bump)]
    pub global_registry: Account<'info, Controller>,
    /// CHECK: This account is not read or written
    pub holding_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn register_strategy_handler(ctx: Context<RegisterStrategy>, weight: u8) -> Result<()> {
    let global_restitry = &mut ctx.accounts.global_registry;
    let new_account = &mut ctx.accounts.holding_account;
    let strategy: Strategy = Strategy {
        holding_account: new_account.key(),
        weight: weight,
        active: true,
    };
    global_restitry.strategies.push(strategy);

    Ok(())
}
