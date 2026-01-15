use crate::{
    state::Config, ANCHOR_DISCRIMINATOR, LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_DECIMALS,
    MIN_HEALTH_FACTOR, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};
// // what are the accounts we'll need to initialize the config?
//  pub authority: Pubkey,
//     pub mint_account: Pubkey,
//     pub liquidation_threshold: u64, // the liquidation threshold & bonus should be scaled don't use floats
//     pub liquidation_bonus: u64,
//     pub min_health_factor: u64,
//     pub bump: u8,
//     pub bump_mint_account: u8,
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_initialize_config(context: Context<InitializeConfig>) -> Result<()> {
    *context.accounts.config_account = Config {
        authority: context.accounts.authority.key(),
        mint_account: context.accounts.mint_account.key(),
        liquidation_threshold: LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,
        bump: context.bumps.config_account,
        bump_mint_account: context.bumps.mint_account,
    };
    Ok(())
}
