use anchor_lang::prelude::*;
mod instructions;
mod state;
mod utils;

use instructions::*;
use state::*;

mod errors;

declare_id!("GfnsaGsBQ2bWBdoQ2WsgcwJQAKMUBNJdx9aakWtARMs7");

#[program]
pub mod infusing_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn register_strategy(ctx: Context<RegisterStrategy>, weight: u8) -> Result<()> {
        register_strategy_handler(ctx, weight)
    }

    pub fn infuse<'info>(
        ctx: Context<'_, '_, '_, 'info, Infuse<'info>>,
        amount: u64,
    ) -> Result<()> {
        infuse_handler(ctx, amount)
    }
}
