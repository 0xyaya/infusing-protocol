use crate::state::{Controller, Strategy, StrategyAccount};
use anchor_lang::{prelude::*, accounts::signer};

use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};

#[derive(Accounts)]
pub struct AddStrategy<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [ b"controller"], bump)]
    pub controller: Account<'info, Controller>,
    /// CHECK: This account is not read or written
    pub strategy_authority: UncheckedAccount<'info>,
    /// CHECK: This account is not read or written
    pub redeem_address: UncheckedAccount<'info>,
    /// CHECK: This account is not read or written
    pub price_feed: UncheckedAccount<'info>,
    #[account(init_if_needed, seeds = [b"strategy", controller.next_strategy_id().unwrap().to_le_bytes().as_ref()], payer = signer, space = 8 + StrategyAccount::SIZE, bump)]
    pub strategy: Account<'info, StrategyAccount>, // Maybe not needed as input account ?
    pub system_program: Program<'info, System>,
}

pub fn add_strategy_handler(ctx: Context<AddStrategy>, weight: u8) -> Result<()> {
    let controller_details = &mut ctx.accounts.controller;
    let strategy_id = u32::try_from(controller_details.strategies.len() + 1).unwrap();
    ctx.accounts.strategy.id = strategy_id;
    ctx.accounts.strategy.authority = ctx.accounts.strategy_authority.key();
    ctx.accounts.strategy.active = true;
    ctx.accounts.strategy.weight = weight;
    ctx.accounts.strategy.redeem_address = ctx.accounts.redeem_address.key();
    ctx.accounts.strategy.price_feed = ctx.accounts.price_feed.key();

    let strategy: Strategy = Strategy {
        id: strategy_id,
        weight: weight,
        active: true,
        authority: ctx.accounts.strategy_authority.key(),
        lamports: 0,
        redeem_address: ctx.accounts.redeem_address.key(),
        price_feed: ctx.accounts.price_feed.key()
    };
    controller_details.strategies.push(strategy);
    Ok(())
}
