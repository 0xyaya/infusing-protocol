use crate::state::{Strategy, StrategyAccount};
use anchor_lang::prelude::*;

#[account]
pub struct Controller {
    pub fees_account: Pubkey,
    pub strategies: Vec<Strategy>,
}

impl Controller {
    pub const MAX_SIZE: usize = 32 + 4 + StrategyAccount::SIZE * 10; // For 10 strategies max
    pub fn next_strategy_id(&self) -> Result<u32> {
        Ok(u32::try_from(self.strategies.len()+1).unwrap())
    }
    
}
