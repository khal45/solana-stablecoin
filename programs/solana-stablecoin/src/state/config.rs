use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub authority: Pubkey,
    pub mint_account: Pubkey,
    // the liquidation threshold & bonus should be scaled don't use floats
    pub liquidation_threshold: u64, // this means you can only borrow up to `liquidation_threshold` of your collateral
    pub liquidation_bonus: u64,
    pub min_health_factor: u64,
    pub bump: u8,
    pub bump_mint_account: u8,
}
