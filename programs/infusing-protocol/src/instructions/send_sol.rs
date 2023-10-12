use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::{system_instruction, self};
use anchor_spl::token::TokenAccount;
use anchor_lang::system_program;
use crate::state::{GlobalRegistryParams, GlobalRegistryState};

#[derive(Accounts)]
pub struct SendInfo<'info> {
    #[account(mut, seeds = [ b"state"], bump)]
    pub state: Account<'info, GlobalRegistryState>,
    /// CHECK: This account is not read or written
   #[account(mut, constraint = holding_account.key() == state.holding_account.key())]
    pub holding_account: AccountInfo<'info>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct State {
    pub holding_account: Pubkey,
}


pub fn send_sol_handler(ctx: Context<SendInfo>, amount: u64) -> Result<()>
{
    let signer = &mut ctx.accounts.sender;
    let holding_account = &mut ctx.accounts.holding_account;
    let holding_transfer_instruction = system_instruction::transfer(signer.key, holding_account.key, amount);

    invoke_signed(
        &holding_transfer_instruction,
        &[
            signer.to_account_info(),
            holding_account.clone(),
            ctx.accounts.system_program.to_account_info()
            ],
        &[]
        )?;
        Ok(())
}