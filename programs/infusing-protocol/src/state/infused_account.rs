use anchor_lang::prelude::*;

#[account]
pub struct InfusedAccount {
    pub nft_mint: Pubkey,
    pub carbon_score: u64,
    pub last_infused_time: u64,
}

impl InfusedAccount {
    pub const SIZE: usize = 32 + 8 + 8;
}
