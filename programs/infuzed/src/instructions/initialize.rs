use anchor_lang::prelude::*;

use crate::state::{Controller, FeeAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,seeds = [ b"controller"], payer = signer, space = 8 + Controller::MAX_SIZE, bump)]
    pub state: Account<'info, Controller>,
    #[account(init, seeds = [ b"fees"], payer = signer, space = 8 + 8, bump)]
    pub fees_account: Account<'info, FeeAccount>, // Maybe not needed as input account ?
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.fees_account = ctx.accounts.fees_account.key();

    Ok(())
}
