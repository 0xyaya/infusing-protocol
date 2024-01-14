use crate::state::{Controller, Strategy};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddStrategy<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [ b"controller"], bump)]
    pub controller: Account<'info, Controller>,
    /// CHECK: This account is not read or written
    pub holding_account: AccountInfo<'info>, // Maybe not needed as input account ?
    pub system_program: Program<'info, System>,
}

pub fn add_strategy_handler(ctx: Context<AddStrategy>, weight: u8) -> Result<()> {
    let controller_details = &mut ctx.accounts.controller;
    let new_account = &mut ctx.accounts.holding_account;
    let strategy: Strategy = Strategy {
        holding_account: new_account.key(),
        weight: weight,
        active: true,
    };
    controller_details.strategies.push(strategy);

    Ok(())
}
