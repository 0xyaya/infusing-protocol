use anchor_lang::prelude::*;

#[account]
pub struct Controller {
    pub fees_account: Pubkey,
    pub strategies: Vec<Strategy>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Strategy {
    pub holding_account: Pubkey,
    pub weight: u8,
    pub active: bool,
}
