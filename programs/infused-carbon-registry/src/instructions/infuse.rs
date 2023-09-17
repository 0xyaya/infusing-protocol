use crate::state::{GlobalRegistryState, InfusedAccount};
use crate::utils::get_latest_price;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use switchboard_v2::AggregatorAccountData;

#[derive(Accounts)]
pub struct Infuse<'info> {
    #[account( seeds = [ b"global-registry"], bump)]
    pub global_registry: Account<'info, GlobalRegistryState>,
    /// CHECK: This account is not read or written
    pub nft_mint: UncheckedAccount<'info>,
    #[account(init_if_needed, seeds = [ b"infused-account", nft_mint.key().as_ref()], payer = signer, space = 168, bump)]
    pub infused_account: Account<'info, InfusedAccount>,
    #[account(mut)]
    pub holding_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub sol_usd_price_feed: AccountLoader<'info, AggregatorAccountData>,
    // pub nct_usd_price_feed: AccountLoader<'info, AggregatorAccountData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn infuse_handler(ctx: Context<Infuse>, amount: u64, nct_usd_price: f64) -> Result<()> {
    // let clock = Clock::get()?;
    let global_registry = &mut ctx.accounts.global_registry;
    let infused_account = &mut ctx.accounts.infused_account;
    let mint = &mut ctx.accounts.mint;
    // calcul fees
    let fees = amount
        .checked_sub(amount.checked_mul(1 / 100).unwrap())
        .unwrap();
    // send fees to the fees_account
    // get oracle price feed
    let price = get_latest_price(
        &ctx.accounts.sol_usd_price_feed,
        // &ctx.accounts.nct_usd_price_feed,
        &nct_usd_price,
        global_registry.feed_staleness_threshold,
    )?;
    // Price is token price in SOL
    // Amount is in lamports (9 dp)
    // We need to convert to the token amount in minor units
    // token amount = lamports / (10^(9-decimals)) * price
    // Note, this works even if decimals > 9
    let token_decimal_denominator = (10_f64).powi(9_i32 - mint.decimals as i32);
    let token_amount_to_buy_and_burn = (amount as f64 / (token_decimal_denominator * price)) as u64;

    msg!(
        "{} CTT sent to infused {}",
        amount,
        infused_account.to_account_info().key().to_string()
    );
    msg!("{} NCT bought", token_amount_to_buy_and_burn);
    msg!("{} CTT collected as fees", fees);
    // calcul right amount of NCT with amount CTT
    // send NCT bought value in CTT to the holding account
    // increase carbon score with NCT burnt
    // infused_account.carbon_score = infused_account.carbon_score + amount;
    // infused_account.last_infused_time = clock.unix_timestamp as u64;
    Ok(())
}
