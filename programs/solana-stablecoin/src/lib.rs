pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6B2Hxx7Lv6ohAtSKEoEE5T2rrf9XzKA8mGddiAknpeJA");

#[program]
pub mod solana_stablecoin {
    use super::*;

    pub fn initialize_config(context: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(context)
    }

    pub fn update_config(context: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(context, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_tokens(
        context: Context<DepositCollateralAndMintTokens>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(context, amount_collateral, amount_to_mint)
    }

    pub fn redeem_collateral_and_burn_tokens(
        context: Context<RedeemCollateralAndBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(context, amount_collateral, amount_to_burn)
    }

    pub fn liquidate(context: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        process_liquidate(context, amount_to_burn)
    }
}
