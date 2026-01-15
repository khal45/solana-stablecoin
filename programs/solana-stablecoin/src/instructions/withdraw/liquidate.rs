use crate::{
    burn_tokens, check_health_factor, collateral,
    error::StablecointError,
    get_lamports_from_usd,
    state::{Collateral, Config},
    withdraw_sol, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,
    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        has_one = sol_account,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/// liquidate the collateral account
/// # Arguments
/// * `amount_to_burn` - Amount to burn in usd
pub fn process_liquidate(context: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
    let health_factor = check_health_factor(
        &context.accounts.collateral_account,
        &context.accounts.config_account,
        &context.accounts.price_update,
    )?;

    // this line will never get hit since the health factor check will error if it is below the minimum
    require!(
        health_factor < context.accounts.config_account.min_health_factor,
        StablecointError::AboveMinimumHealthFactor
    );

    let lamports = get_lamports_from_usd(&amount_to_burn, &context.accounts.price_update)?;
    let liquidation_bonus = lamports
        .checked_mul(context.accounts.config_account.liquidation_bonus)
        .and_then(|n| n.checked_div(100))
        .ok_or(StablecointError::MathError)?;
    let amount_to_liquidate = lamports
        .checked_add(liquidation_bonus)
        .ok_or(StablecointError::MathError)?;

    withdraw_sol(
        &context.accounts.sol_account,
        &context.accounts.liquidator,
        &context.accounts.system_program,
        &context.accounts.collateral_account.depositor,
        context.accounts.collateral_account.bump_sol_account,
        amount_to_liquidate,
    )?;

    burn_tokens(
        &context.accounts.token_program,
        &context.accounts.mint_account,
        &context.accounts.token_account,
        &context.accounts.liquidator,
        amount_to_burn,
    )?;

    let collateral_account = &mut context.accounts.collateral_account;
    // the liquidation already occured so the current amount of lamports is correct
    collateral_account.lamport_balance = context.accounts.sol_account.lamports();
    collateral_account.amount_minted = collateral_account
        .amount_minted
        .checked_sub(amount_to_burn)
        .ok_or(StablecointError::MathError)?;

    check_health_factor(
        &context.accounts.collateral_account,
        &context.accounts.config_account,
        &context.accounts.price_update,
    )?;

    Ok(())
}
