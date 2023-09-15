use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GlobalRegistryParams {
    pub holding_account: Pubkey,
    pub fees_account: Pubkey,
    pub ctt_mint: Pubkey,
    pub nct_mint: Pubkey,
}

#[account]
pub struct GlobalRegistryState {
    pub holding_account: Pubkey,
    pub fees_account: Pubkey,
    pub ctt_mint: Pubkey,
    pub nct_mint: Pubkey,
}
