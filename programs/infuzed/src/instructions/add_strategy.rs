use crate::state::{Controller, Strategy, StrategyAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(strategy_id: u64)]
pub struct AddStrategy<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [ b"controller"], bump)]
    pub controller: Account<'info, Controller>,
    // #[account(init, seeds = [ b"escrowed"], payer = signer, space = 8 + 32, bump)]
    // pub escrowed: Account<'info, EscrowedStrategy>,
    
    #[account(init_if_needed, seeds = [b"strategy"], payer = signer, space = 8 + 32 + 1 + 1, bump)]
    pub strategy: Account<'info, StrategyAccount>, // Maybe not needed as input account ?
    pub system_program: Program<'info, System>,
}

pub fn add_strategy_handler(ctx: Context<AddStrategy>, weight: u8, strategy_id: u64) -> Result<()> {
    let controller_details = &mut ctx.accounts.controller;
    let new_account = &mut ctx.accounts.strategy;
    let strategy: Strategy = Strategy {
        holding_account: new_account.key(),
        weight: weight,
        active: true,
    };
    controller_details.strategies.push(strategy);

    Ok(())
}
