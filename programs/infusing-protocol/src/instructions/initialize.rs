use anchor_lang::prelude::*;

use crate::state::Controller;

#[derive(Accounts)]
pub struct Initialize<'info> {
    // TODO: fix this stupid fixed space
    #[account(init,seeds = [ b"controller"], payer = signer, space = 136, bump)]
    pub state: Account<'info, Controller>,
    /// CHECK: This account is not read or written
    pub fees_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.fees_account = ctx.accounts.fees_account.key();

    Ok(())
}
