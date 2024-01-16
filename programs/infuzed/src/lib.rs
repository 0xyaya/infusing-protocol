use anchor_lang::prelude::*;
mod instructions;
mod state;
mod utils;

use instructions::*;

mod errors;

declare_id!("GfnsaGsBQ2bWBdoQ2WsgcwJQAKMUBNJdx9aakWtARMs7");

#[program]
pub mod infuzed {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn add_strategy(ctx: Context<AddStrategy>, weight: u8, strategy_id: u64) -> Result<()> {
        add_strategy_handler(ctx, weight, strategy_id)
    }

    pub fn infuse<'info>(
        ctx: Context<'_, '_, '_, 'info, Infuse<'info>>,
        amount: u64,
    ) -> Result<()> {
        infuse_handler(ctx, amount)
    }
}
