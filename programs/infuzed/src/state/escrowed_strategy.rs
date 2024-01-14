use anchor_lang::prelude::*;

#[account]
struct EscrowedStrategy {
    pub strategy: Pubkey,
    pub authority: Pubkey,
}