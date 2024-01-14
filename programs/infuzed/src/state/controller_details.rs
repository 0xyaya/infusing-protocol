use crate::state::Strategy;
use anchor_lang::prelude::*;

#[account]
pub struct Controller {
    pub fees_account: Pubkey,
    pub strategies: Vec<Strategy>,
}

impl Controller {
    pub const MAX_SIZE: usize = 32 + 4 + Strategy::SIZE * 10; // For 10 strategies max
}
