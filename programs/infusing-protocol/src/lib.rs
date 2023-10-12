use anchor_lang::prelude::*;
mod instructions;
mod state;
mod utils;

use instructions::*;
use state::*;

declare_id!("GfnsaGsBQ2bWBdoQ2WsgcwJQAKMUBNJdx9aakWtARMs7");

#[program]
pub mod infusing_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn infuse(ctx: Context<Infuse>, amount: u64) -> Result<()> {
        infuse_handler(ctx, amount)
    }
}
