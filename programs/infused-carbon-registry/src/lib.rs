use anchor_lang::prelude::*;
mod instructions;
mod state;
mod utils;

use instructions::*;
use state::*;

declare_id!("3g3YSqpjbWGYSSGTEVhP3jLavuLpH3toyE6zgMUrzzoC");

#[program]
pub mod infused_carbon_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: GlobalRegistryParams) -> Result<()> {
        initialize_handler(ctx, params)
    }

    pub fn infuse(ctx: Context<Infuse>, amount: u64, nct_usd_price: f64) -> Result<()> {
        infuse_handler(ctx, amount, nct_usd_price)
    }
}
