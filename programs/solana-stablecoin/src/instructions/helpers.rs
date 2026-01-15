use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{
    mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
};

// ============================================================================
// SPL Token Transfer Helpers
// ============================================================================

/// Transfers SPL tokens from a **user-owned token account**.
///
/// This should be used when the authority is a regular signer
/// (i.e. a wallet / externally owned account).
///
/// Internally performs a `transfer_checked` CPI to the SPL Token program.
pub fn transfer_spl_from_user_token_account<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
        mint: mint.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, transfer_cpi_accounts);

    transfer_checked(cpi_context, amount, mint.decimals)
}

/// Transfers SPL tokens from a **PDA-owned token account**.
///
/// This should be used when the authority is a program-derived address (PDA),
/// such as a vault or bank account owned by the program.
///
/// The caller must provide the correct `signer_seeds` for the PDA.
/// Internally performs a `transfer_checked` CPI with signer seeds.
pub fn transfer_spl_from_pda_token_account<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &impl AsRef<AccountInfo<'info>>,
    token_program: &Interface<'info, TokenInterface>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let transfer_cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.as_ref().clone(),
        mint: mint.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(cpi_program, transfer_cpi_accounts, signer_seeds);

    transfer_checked(cpi_context, amount, mint.decimals)
}

// ============================================================================
// SOL (Lamport) Transfer Helpers
// ============================================================================

/// Transfers SOL from a **signer wallet** to any system-owned account
/// (EOA or PDA).
pub fn transfer_sol_from_user<'info>(
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    amount: u64,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let cpi_context = CpiContext::new(
        system_program.to_account_info(),
        Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
        },
    );

    transfer(cpi_context, amount).map_err(|e| {
        msg!("SOL transfer (user) failed: {:?}", e);
        error!(Errors::SolTransferFailed)
    })
}

// /// Transfers SOL from a **PDA (program-owned system account)** to any
// /// system-owned account (EOA or PDA).
// pub fn transfer_sol_from_pda<'info>(
//     from: &SystemAccount<'info>,
//     to: &SystemAccount<'info>,
//     amount: u64,
//     system_program: &Program<'info, System>,
//     signer_seeds: &[&[&[u8]]],
// ) -> Result<()> {
//     let cpi_context = CpiContext::new_with_signer(
//         system_program.to_account_info(),
//         Transfer {
//             from: from.to_account_info(),
//             to: to.to_account_info(),
//         },
//         signer_seeds,
//     );

//     transfer(cpi_context, amount).map_err(|e| {
//         msg!("SOL transfer (pda) failed: {:?}", e);
//         error!(Errors::SolTransferFailed)
//     })
// }

// temporal function for transfering sol from pda, check this later
/// Transfers SOL from a **PDA (program-owned system account)** to any
/// system-owned account (EOA or PDA).
pub fn transfer_sol_from_pda<'info>(
    from: &SystemAccount<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
    system_program: &Program<'info, System>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let cpi_context = CpiContext::new_with_signer(
        system_program.to_account_info(),
        Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
        },
        signer_seeds,
    );

    transfer(cpi_context, amount).map_err(|e| {
        msg!("SOL transfer (pda) failed: {:?}", e);
        error!(Errors::SolTransferFailed)
    })
}

// ============================================================================
// SPL / Token-2022 Minting Helpers (Unchecked)
// ============================================================================

/// Mints tokens using a **PDA mint authority**.
///
/// ⚠️ Unchecked:
/// - `amount` is raw units
/// - decimals are NOT verified
///
/// This is required for Token-2022 compatibility.
pub fn mint_spl_tokens_2022<'info>(
    mint: &InterfaceAccount<'info, Mint>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint_authority: &InterfaceAccount<'info, Mint>,
    amount: u64,
    token_program: &Program<'info, Token2022>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: mint.to_account_info(),
                to: to.to_account_info(),
                authority: mint_authority.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}

/// Mints tokens using a **user signer** as mint authority.
///
/// ⚠️ Unchecked:
/// - `amount` is raw units
/// - decimals are NOT verified
///
/// Token-2022 compatible.
pub fn mint_tokens_with_user_authority<'info>(
    mint: &InterfaceAccount<'info, Mint>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint_authority: &Signer<'info>,
    amount: u64,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let cpi_context = CpiContext::new(
        token_program.to_account_info(),
        MintTo {
            mint: mint.to_account_info(),
            to: to.to_account_info(),
            authority: mint_authority.to_account_info(),
        },
    );

    mint_to(cpi_context, amount).map_err(|e| {
        msg!("SPL mint (user) failed: {:?}", e);
        error!(Errors::SPLMintFailed)
    })
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum Errors {
    #[msg("SOL transfer failed")]
    SolTransferFailed,
    #[msg("SPL mint failed")]
    SPLMintFailed,
}
