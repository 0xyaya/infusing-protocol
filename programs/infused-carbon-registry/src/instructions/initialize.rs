use anchor_lang::prelude::*;

use crate::state::{GlobalRegistryParams, GlobalRegistryState};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,seeds = [ b"global-registry"], payer = signer, space = 8, bump)]
    pub state: Account<'info, GlobalRegistryState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>, params: GlobalRegistryParams) -> Result<()> {
    msg!("initialize the global registry state");
    let state = &mut ctx.accounts.state;
    state.holding_account = params.holding_account;
    state.fees_account = params.fees_account;
    state.ctt_mint = params.ctt_mint;
    state.nct_mint = params.nct_mint;
    Ok(())
}
