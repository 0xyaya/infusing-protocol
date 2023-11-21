use anchor_lang::prelude::*;

use crate::state::ControllerDetails;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,seeds = [ b"controller-details"], payer = signer, space = 8 + ControllerDetails::MAX_SIZE, bump)]
    pub state: Account<'info, ControllerDetails>,
    /// CHECK: This account is not read or written
    pub fees_account: AccountInfo<'info>, // Maybe not needed as input account ?
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.fees_account = ctx.accounts.fees_account.key();

    Ok(())
}
