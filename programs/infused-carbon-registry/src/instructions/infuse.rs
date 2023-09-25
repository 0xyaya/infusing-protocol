use crate::state::{GlobalRegistryState, InfusedAccount};
use crate::utils::get_latest_price;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::{system_instruction, self};
use anchor_lang::{prelude::*, system_program};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
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
    /// CHECK: This account is not read or written
    #[account(mut)]
    pub holding_account: AccountInfo<'info>,
    /// CHECK: This account is not read or written
    #[account(mut)]
    pub fees_account: AccountInfo<'info>,
    pub sol_usd_price_feed: AccountLoader<'info, AggregatorAccountData>, // check if the feed is the good one
    pub nct_usd_price_feed: AccountLoader<'info, AggregatorAccountData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn infuse_handler(ctx: Context<Infuse>, amount: u64, nct_usd_price: f64) -> Result<()> {
    // let clock = Clock::get()?;
    // let global_registry = &mut ctx.accounts.global_registry;
    let infused_account = &mut ctx.accounts.infused_account;
    let fees_account = &mut ctx.accounts.fees_account;
    let holding_account = &mut ctx.accounts.holding_account;
    let signer = &mut ctx.accounts.signer;
    // calcul fees
    let lamports = amount.checked_mul(LAMPORTS_PER_SOL).unwrap();
    let fees = lamports as f64 * 0.04;
    let amount_to_burn = lamports
        .checked_sub(fees as u64)
        .unwrap();
    
    msg!("fees: {}",fees);
    msg!("amount to burn: {}", amount_to_burn);

    let holding_transfer_instruction = system_instruction::transfer(signer.key, holding_account.key, amount_to_burn);
    let fees_transfer_instruction = system_instruction::transfer(signer.key, fees_account.key, fees as u64);

    invoke_signed(
        &holding_transfer_instruction,
        &[
            signer.to_account_info(),
            holding_account.to_account_info(),
            ctx.accounts.system_program.to_account_info()
            ],
        &[]
        )?;

        invoke_signed(
            &fees_transfer_instruction,
            &[
                signer.to_account_info(),
                fees_account.to_account_info(),
                ctx.accounts.system_program.to_account_info()
                ],
            &[]
            )?;

    // send fees to the fees_account
    // get oracle price feed
    // let price = get_latest_price(
    //     &ctx.accounts.sol_usd_price_feed,
    //     &ctx.accounts.nct_usd_price_feed,
    //     nct_usd_price,
    //     global_registry.feed_staleness_threshold,
    // )?;
    let price = 1.41 as f64;
    // Price is token price in SOL
    // Amount is in lamports (9 dp)
    // We need to convert to the token amount in minor units
    // token amount = lamports / (10^(9-decimals)) * price
    // Note, this works even if decimals > 9
    // let token_decimal_denominator = (10_f64).powi(9_i32 - LAMPORTS_PER_SOL as i32);
    let carbon_tons = (amount_to_burn as f64 / (LAMPORTS_PER_SOL as f64 * price)) as u64;
    infused_account.carbon_score = infused_account.carbon_score.checked_add(carbon_tons).unwrap();
    msg!(
        "{} CTT sent to infused {}",
        amount_to_burn,
        infused_account.to_account_info().key().to_string()
    );
    msg!("{} NCT bought", carbon_tons);
    msg!("{} CTT collected as fees", fees);
    // calcul right amount of NCT with amount CTT
    // send NCT bought value in CTT to the holding account
    // increase carbon score with NCT burnt
    // infused_account.carbon_score = infused_account.carbon_score + amount;
    // infused_account.last_infused_time = clock.unix_timestamp as u64;
    Ok(())
}
