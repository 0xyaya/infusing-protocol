use anchor_lang::prelude::*;

#[account]
pub struct GlobalRegistry {
    pub strategies: Vec<Strategy>,
    pub fees_account: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Strategy {
    pub holding_account: Pubkey,
    pub weight: u8,
}
