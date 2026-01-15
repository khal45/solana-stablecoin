use crate::{state::Config, SEED_CONFIG_ACCOUNT};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump
    )]
    pub config_account: Account<'info, Config>,
}

// this can only update the minimum health factor
pub fn process_update_config(context: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
    let config_account = &mut context.accounts.config_account;
    config_account.min_health_factor = min_health_factor;
    Ok(())
}
