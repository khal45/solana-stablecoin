use crate::{
    check_health_factor, deposit_sol,
    error::StablecointError,
    mint_tokens,
    state::{Collateral, Config},
    ANCHOR_DISCRIMINATOR, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT, SEED_SOL_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens<'info> {
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = ANCHOR_DISCRIMINATOR + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, depositor.key().as_ref()],
        bump
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub price_update: Account<'info, PriceUpdateV2>,
}

pub fn process_deposit_collateral_and_mint_tokens(
    context: Context<DepositCollateralAndMintTokens>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    let collateral_account = &mut context.accounts.collateral_account;
    collateral_account.lamport_balance = context
        .accounts
        .sol_account
        .lamports()
        .checked_add(amount_collateral)
        .ok_or(StablecointError::MathError)?;
    collateral_account.amount_minted = collateral_account
        .amount_minted
        .checked_add(amount_to_mint)
        .ok_or(StablecointError::MathError)?;

    if !collateral_account.is_initialized {
        collateral_account.is_initialized = true;
        collateral_account.depositor = context.accounts.depositor.key();
        collateral_account.sol_account = context.accounts.sol_account.key();
        collateral_account.token_account = context.accounts.token_account.key();
        collateral_account.bump = context.bumps.collateral_account;
    }

    // why are we checking the health factor?
    check_health_factor(
        &context.accounts.collateral_account,
        &context.accounts.config_account,
        &context.accounts.price_update,
    )?;

    deposit_sol(
        &context.accounts.system_program,
        &context.accounts.depositor,
        &context.accounts.sol_account,
        amount_collateral,
    )?;

    mint_tokens(
        context.accounts.config_account.bump_mint_account,
        &context.accounts.token_program,
        &context.accounts.mint_account,
        &context.accounts.token_account,
        amount_to_mint,
    )?;
    Ok(())
}
