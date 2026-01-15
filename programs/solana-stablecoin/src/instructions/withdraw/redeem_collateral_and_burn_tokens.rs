use crate::{
    burn_tokens, check_health_factor,
    error::StablecointError,
    state::{Collateral, Config},
    withdraw_sol, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
#[derive(Accounts)]
pub struct RedeemCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump = collateral_account.bump,
        has_one = sol_account,
        has_one = token_account
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

pub fn process_redeem_collateral_and_burn_tokens(
    context: Context<RedeemCollateralAndBurnTokens>,
    amount_collateral: u64,
    amount_to_burn: u64,
) -> Result<()> {
    let collateral_account = &mut context.accounts.collateral_account;
    collateral_account.lamport_balance = context
        .accounts
        .sol_account
        .lamports()
        .checked_sub(amount_collateral)
        .ok_or(StablecointError::MathError)?;
    collateral_account.amount_minted = collateral_account
        .amount_minted
        .checked_sub(amount_to_burn)
        .ok_or(StablecointError::MathError)?;

    check_health_factor(
        &context.accounts.collateral_account,
        &context.accounts.config_account,
        &context.accounts.price_update,
    )?;

    burn_tokens(
        &context.accounts.token_program,
        &context.accounts.mint_account,
        &context.accounts.token_account,
        &context.accounts.depositor,
        amount_to_burn,
    )?;

    withdraw_sol(
        &context.accounts.sol_account,
        &context.accounts.depositor.to_account_info(),
        &context.accounts.system_program,
        &context.accounts.depositor.key(),
        context.accounts.collateral_account.bump_sol_account,
        amount_collateral,
    )?;

    Ok(())
}
