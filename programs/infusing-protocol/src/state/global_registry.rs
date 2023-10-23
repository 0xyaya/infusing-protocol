use anchor_lang::prelude::*;

#[account]
pub struct Controller {
    pub strategies: Vec<Strategy>,
    pub fees_account: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Strategy {
    pub holding_account: Pubkey,
    pub weight: u8,
    pub active: bool,
}
