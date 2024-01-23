use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Strategy {
    pub id: u32,
    pub weight: u8,
    pub active: bool,
    pub authority: Pubkey,
    pub redeem_address: Pubkey,
    pub lamports:u64,
    pub price_feed: Pubkey
}

#[account]
pub struct StrategyAccount {
    pub id: u32,
    pub weight: u8,
    pub active: bool,
    pub authority: Pubkey,
    pub redeem_address: Pubkey,
    pub lamports:u64 ,
    pub price_feed: Pubkey
}

impl StrategyAccount {
    pub const SIZE: usize = 1 + 1 + 4 + 32 + 8 + 1 + 32 + 32;
}
