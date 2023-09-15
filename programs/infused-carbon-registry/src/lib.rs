use anchor_lang::prelude::*;
mod instructions;
mod state;

use instructions::*;
declare_id!("3g3YSqpjbWGYSSGTEVhP3jLavuLpH3toyE6zgMUrzzoC");

#[program]
pub mod infused_carbon_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: state::GlobalRegistryParams) -> Result<()> {
        initialize_handler(ctx, params)
    }
}
