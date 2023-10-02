use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GlobalRegistryParams {
    pub holding_account: Pubkey,
    pub fees_account: Pubkey,
    // pub ctt_mint: Pubkey,
    // pub nct_mint: Pubkey,
    // pub feed_staleness_threshold: u64,
}

#[account]
pub struct GlobalRegistryState {
    pub holding_account: Pubkey,
    pub fees_account: Pubkey,
    // pub ctt_mint: Pubkey,
    // pub nct_mint: Pubkey,
    // pub feed_staleness_threshold: u64,
}
