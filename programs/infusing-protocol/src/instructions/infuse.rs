use crate::errors::ErrorCode;
use crate::state::{ControllerDetails, InfusedAccount};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::{self, system_instruction};
use anchor_lang::{prelude::*, system_program};

#[event]
pub struct AccountInfused {
    pub amount: u64,
    pub nft_mint: Pubkey,
    pub time: u64,
}

#[derive(Accounts)]
pub struct Infuse<'info> {
    #[account( seeds = [ b"controller-details"], bump)]
    pub global_registry: Account<'info, ControllerDetails>,
    /// CHECK: This account is not read or written
    pub nft_mint: UncheckedAccount<'info>,
    #[account(init_if_needed, seeds = [ b"infused-account", nft_mint.key().as_ref()], payer = signer, space = 168, bump)]
    pub infused_account: Account<'info, InfusedAccount>,

    /// CHECK: This account is not read or written
    #[account(mut)]
    pub fees_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn infuse_handler<'info>(
    ctx: Context<'_, '_, '_, 'info, Infuse<'info>>,
    amount: u64,
) -> Result<()> {
    let controller = &mut ctx.accounts.global_registry;
    let infused_account = &mut ctx.accounts.infused_account;
    let fees_account = &mut ctx.accounts.fees_account;
    let signer = &mut ctx.accounts.signer;
    let lamports = amount.checked_mul(LAMPORTS_PER_SOL).unwrap();
    let fees = lamports as f64 * 0.04;
    let amount_to_burn = lamports.checked_sub(fees as u64).unwrap();
    let fees_transfer_instruction =
        system_instruction::transfer(signer.key, fees_account.key, fees as u64);
    // filter controller strategies to only active ones
    // let strategies = controller
    //     .strategies
    //     .iter()
    //     .filter(|s| s.active)
    //     .collect::<Vec<_>>();

    if ctx.remaining_accounts.len() != controller.strategies.len() {
        return Err(ErrorCode::InvalidRemainingAccountsLength.into());
    }

    // An alternative routing controller
    for i in 0..ctx.remaining_accounts.len() {
        let holding_account = &ctx.remaining_accounts[i];
        let strategy = controller
            .strategies
            .iter()
            .find(|s| s.holding_account == *holding_account.key)
            .unwrap();

        if !strategy.active {
            continue;
        }

        let amount = amount_to_burn as f64 * (strategy.weight as f64 / 100.00);
        msg!("holding_account: {}", holding_account.key);
        msg!("amount: {}", amount);
        let holding_transfer_instruction =
            system_instruction::transfer(signer.key, holding_account.key, amount as u64);

        invoke_signed(
            &holding_transfer_instruction,
            &[
                signer.to_account_info(),
                holding_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
    }

    invoke_signed(
        &fees_transfer_instruction,
        &[
            signer.to_account_info(),
            ctx.accounts.fees_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    // TODO: use switchboard price feed
    let price = 1.40 as f64 / 29 as f64;

    let carbon_tons = (amount_to_burn as f64 / (LAMPORTS_PER_SOL as f64 * price)) as u64;
    let time = Clock::get().unwrap().unix_timestamp as u64;
    infused_account.carbon_score = infused_account
        .carbon_score
        .checked_add(carbon_tons)
        .unwrap();
    infused_account.nft_mint = ctx.accounts.nft_mint.key();
    infused_account.last_infused_time = time;

    emit!(AccountInfused {
        amount: carbon_tons,
        nft_mint: *ctx.accounts.nft_mint.key,
        time: time,
    });
    Ok(())
}
