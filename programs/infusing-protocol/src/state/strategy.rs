use anchor_lang::prelude::*;

#[account]
pub struct StrategiesState {
    pub holding_accounts: Vec<Pubkey>,
}
