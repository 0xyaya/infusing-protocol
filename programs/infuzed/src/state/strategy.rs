use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Strategy {
    pub holding_account: Pubkey,
    pub weight: u8,
    pub active: bool,
}

impl Strategy {
    pub const SIZE: usize = 32 + 1 + 1;
}
