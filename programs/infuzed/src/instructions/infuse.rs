use crate::errors::ErrorCode;
use crate::state::{AccountInfused, ControllerDetails, InfusedAccount};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;

#[derive(Accounts)]
pub struct Infuse<'info> {
    #[account( seeds = [ b"controller-details"], bump)]
    pub controller_details: Account<'info, ControllerDetails>,
    /// CHECK: This account is not read or written
    pub nft_mint: UncheckedAccount<'info>,
    #[account(init_if_needed, seeds = [ b"infused-account", nft_mint.key().as_ref()], payer = signer, space = 8 + InfusedAccount::SIZE, bump)]
    pub infused_account: Account<'info, InfusedAccount>,
    /// CHECK: This account is not read or written
    #[account(mut)]
    pub fees_account: AccountInfo<'info>,
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

    pub fn transfer_strategies(
        &self,
        remaining_accounts: &[AccountInfo<'info>],
        amount: u64,
    ) -> Result<()> {
        if remaining_accounts.len() != self.controller_details.strategies.len() {
            return Err(ErrorCode::InvalidRemainingAccountsLength.into());
        }

        for i in 0..remaining_accounts.len() {
            let holding_account = &remaining_accounts[i];
            let strategy = self
                .controller_details
                .strategies
                .iter()
                .find(|s| s.holding_account == *holding_account.key)
                .unwrap();

            if !strategy.active {
                continue;
            }

            let share_amount = amount as f64 * (strategy.weight as f64 / 100.00);
            let holding_transfer_instruction = system_instruction::transfer(
                self.signer.key,
                holding_account.key,
                share_amount as u64,
            );

            invoke_signed(
                &holding_transfer_instruction,
                &[
                    self.signer.to_account_info(),
                    holding_account.to_account_info(),
                    self.system_program.to_account_info(),
                ],
                &[],
            )?;
        }

        Ok(())
    }

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
    ctx.accounts.transfer_fees(fees)?;
    ctx.accounts
        .transfer_strategies(ctx.remaining_accounts, amount_to_burn)?;
    ctx.accounts.update_account(amount_to_burn)?;

    Ok(())
}
