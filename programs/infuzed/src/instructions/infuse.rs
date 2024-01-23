use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::ErrorCode;
use crate::state::{Controller, InfusedAccount, StrategyAccount};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::system_program::{Transfer, transfer};
use pyth_sdk_solana::load_price_feed_from_account_info;

const BTC_USDC_FEED: &str = "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J";
const STALENESS_THRESHOLD: u64 = 60; // staleness threshold in seconds

#[derive(Accounts)]
pub struct Infuse<'info> {
    #[account( seeds = [ b"controller"], bump)]
    pub controller: Account<'info, Controller>,
    /// CHECK: This account is not read or written
    pub nft_mint: UncheckedAccount<'info>,
    #[account(init_if_needed, seeds = [ b"infused-account", nft_mint.key().as_ref()], payer = signer, space = 8 + InfusedAccount::SIZE, bump)]
    pub infused_account: Account<'info, InfusedAccount>,
    #[account(mut)]
    pub strategy: Account<'info, StrategyAccount>, // Maybe not needed as input account ?
    /// CHECK: This account is not read or written
    #[account(mut)]
    pub fees_account: AccountInfo<'info>,
    /// CHECK: This account is not read or written
    pub price_feed: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Infuse<'info> {
    pub fn transfer_fees(&self, fees: f64) -> Result<()> {
        let fees_transfer_instruction =
            system_instruction::transfer(self.signer.key, self.fees_account.key, fees as u64);

        invoke_signed(
            &fees_transfer_instruction,
            &[
                self.signer.to_account_info(),
                self.fees_account.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[],
        )?;
        Ok(())
    }

    pub fn transfer_to_strategies(
        &mut self,
        amount: u64
    ) -> Result<()> {
        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(), 
            Transfer {
                from: self.signer.to_account_info(),
                to: self.strategy.to_account_info(),
            });
        transfer(cpi_context, amount)?;

        self.strategy.lamports += amount;
        Ok(())
    }

    // pub fn transfer_strategies(
    //     &self,
    //     remaining_accounts: &[AccountInfo<'info>],
    //     amount: u64,
    // ) -> Result<()> {
    //     if remaining_accounts.len() != self.controller.strategies.len() {
    //         return Err(ErrorCode::InvalidRemainingAccountsLength.into());
    //     }

    //     for i in 0..remaining_accounts.len() {
    //         let holding_account = &remaining_accounts[i];
    //         let strategy = self
    //             .controller
    //             .strategies
    //             .iter()
    //             .find(|s| s.holding_account == *holding_account.key)
    //             .unwrap();

    //         if !strategy.active {
    //             continue;
    //         }

    //         let share_amount = amount as f64 * (strategy.weight as f64 / 100.00);
    //         let holding_transfer_instruction = system_instruction::transfer(
    //             self.signer.key,
    //             holding_account.key,
    //             share_amount as u64,
    //         );

    //         invoke_signed(
    //             &holding_transfer_instruction,
    //             &[
    //                 self.signer.to_account_info(),
    //                 holding_account.to_account_info(),
    //                 self.system_program.to_account_info(),
    //             ],
    //             &[],
    //         )?;
    //     }

    //     Ok(())
    // }

    pub fn update_account(&mut self, amount: u64) -> Result<()> {
        let price = 1.40 as f64 / 29 as f64;
        let carbon_tons = (amount as f64 / (LAMPORTS_PER_SOL as f64 * price)) as u64;

        self.infused_account.update(carbon_tons);

        Ok(())
    }
}

pub fn infuse_handler<'info>(
    ctx: Context<'_, '_, '_, 'info, Infuse<'info>>,
    amount: u64,
) -> Result<()> {
    let lamports = amount.checked_mul(LAMPORTS_PER_SOL).unwrap();
    let fees = lamports as f64 * 0.04;
    let amount_to_burn = lamports.checked_sub(fees as u64).unwrap();

    let price_account_info = &ctx.accounts.price_feed;
    let price_feed = load_price_feed_from_account_info( &price_account_info ).unwrap();
    let current_timestamp = Clock::get()?.unix_timestamp;
    let current_price = price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();

    // 2-Format display values rounded to nearest dollar
    let display_price = u64::try_from(current_price.price).unwrap() / 10u64.pow(u32::try_from(-current_price.expo).unwrap());
    let display_confidence = u64::try_from(current_price.conf).unwrap() / 10u64.pow(u32::try_from(-current_price.expo).unwrap());
    msg!("BTC/USD price: ({} +- {})", display_price, display_confidence);
    // let clock = Clock::get()?;
    // let current_timestamp = clock.unix_timestamp;
    // let mut feed = load_price_feed_from_account_info(&ctx.accounts.price_feed).unwrap();
    // let maybe_price = feed.get_price_no_older_than(current_timestamp, 60);
    // match maybe_price {
    //     Some(p) => {
    //         println!("price ........... {} x 10^{}", p.price, p.expo);
    //         println!("conf ............ {} x 10^{}", p.conf, p.expo);
    //     }
    //     None => {
    //         println!("price ........... unavailable");
    //         println!("conf ............ unavailable");
    //     }
    // }

    ctx.accounts.transfer_fees(fees)?;
    ctx.accounts
        .transfer_to_strategies(amount_to_burn)?;
    ctx.accounts.update_account(amount_to_burn)?;

    Ok(())
}
