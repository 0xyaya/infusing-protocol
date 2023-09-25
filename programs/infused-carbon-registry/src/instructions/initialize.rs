use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::state::{GlobalRegistryParams, GlobalRegistryState};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,seeds = [ b"global-registry"], payer = signer, space = 136, bump)]
    pub state: Account<'info, GlobalRegistryState>,
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

pub fn initialize_handler(ctx: Context<Initialize>, staleness_threshold: u64) -> Result<()> {
    msg!("initialize the global registry state");
    let state = &mut ctx.accounts.state;
    state.holding_account = ctx.accounts.holding_account.key();
    state.fees_account = ctx.accounts.fees_account.key();
    // state.ctt_mint = params.ctt_mint;
    // state.nct_mint = params.nct_mint;
    state.feed_staleness_threshold = staleness_threshold;
    Ok(())
}
