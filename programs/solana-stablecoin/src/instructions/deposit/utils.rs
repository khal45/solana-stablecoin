use crate::helpers::{mint_spl_tokens_2022, transfer_sol_from_user};
use crate::SEED_MINT_ACCOUNT;
use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};

pub fn mint_tokens<'info>(
    bump: u8,
    token_program: &Program<'info, Token2022>,
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];
    mint_spl_tokens_2022(
        mint_account,
        token_account,
        mint_account,
        amount,
        token_program,
        signer_seeds,
    )?;
    Ok(())
}

pub fn deposit_sol<'info>(
    system_program: &Program<'info, System>,
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    amount: u64,
) -> Result<()> {
    transfer_sol_from_user(from, to, amount, system_program)?;
    Ok(())
}
