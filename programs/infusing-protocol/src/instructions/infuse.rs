use crate::state::{GlobalRegistry, InfusedAccount};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::{self, system_instruction};
use anchor_lang::{prelude::*, system_program};

#[derive(Accounts)]
pub struct Infuse<'info> {
    #[account( seeds = [ b"global-registry"], bump)]
    pub global_registry: Account<'info, GlobalRegistry>,
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
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn infuse_handler(ctx: Context<Infuse>, amount: u64) -> Result<()> {
    let global_registry = &mut ctx.accounts.global_registry;
    let infused_account = &mut ctx.accounts.infused_account;
    let fees_account = &mut ctx.accounts.fees_account;
    let holding_account = &mut ctx.accounts.holding_account;
    let signer = &mut ctx.accounts.signer;
    let lamports = amount.checked_mul(LAMPORTS_PER_SOL).unwrap();
    let fees = lamports as f64 * 0.04;
    let amount_to_burn = lamports.checked_sub(fees as u64).unwrap();
    let fees_transfer_instruction =
        system_instruction::transfer(signer.key, fees_account.key, fees as u64);

    // A first routing controller
    for i in 0..global_registry.strategies.len() {
        let strategy = &global_registry.strategies[i];
        let amount = amount_to_burn as f64 * (strategy.weight as f64 / 100.00);
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
    let price = 1.40 as f64 / 21 as f64;

    let carbon_tons = (amount_to_burn as f64 / (LAMPORTS_PER_SOL as f64 * price)) as u64;
    infused_account.carbon_score = infused_account
        .carbon_score
        .checked_add(carbon_tons)
        .unwrap();
    infused_account.nft_mint = ctx.accounts.nft_mint.key();

    Ok(())
}
