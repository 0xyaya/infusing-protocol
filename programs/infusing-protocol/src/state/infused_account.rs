use anchor_lang::prelude::*;

#[account]
pub struct InfusedAccount {
    pub nft_mint: Pubkey,
    pub carbon_score: u64,
    pub last_infused_time: u64,
}

impl InfusedAccount {
    pub const SIZE: usize = 32 + 8 + 8;

    pub fn update(&mut self, carbon_tons: u64) {
        let time = Clock::get().unwrap().unix_timestamp as u64;
        self.carbon_score = self.carbon_score.checked_add(carbon_tons).unwrap();
        self.nft_mint = self.nft_mint.key();
        self.last_infused_time = time;

        emit!(AccountInfused {
            amount: carbon_tons,
            nft_mint: self.nft_mint,
            time: time,
        });
    }
}

#[event]
pub struct AccountInfused {
    pub amount: u64,
    pub nft_mint: Pubkey,
    pub time: u64,
}
