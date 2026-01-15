use crate::{
    error::StablecointError, Collateral, Config, MAXIMUM_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT,
    SOL_USD_FEED_ID,
};
use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

pub fn check_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let health_factor = calculate_health_factor(collateral, config, price_feed)?;
    // I don't think this check should be here
    require!(
        health_factor >= config.min_health_factor,
        StablecointError::BelowMinimumHealthFactor
    );
    Ok(health_factor)
}

pub fn calculate_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    // health factor = collateral adjusted for the liquidation threshold / amount minted
    let collateral_value_in_usd = get_usd_value(&collateral.lamport_balance, price_feed)?;
    let numerator = collateral_value_in_usd
        .checked_mul(config.liquidation_threshold)
        .ok_or(StablecointError::MathError)?;
    // we are dividing by 100 because it is %
    let collateral_adjusted_for_liquidation_threshold = numerator
        .checked_div(100)
        .ok_or(StablecointError::MathError)?;
    // health factor = collateral adjusted for liquidation threshold / amount minted;
    if collateral.amount_minted == 0 {
        msg!("Health Factor Max");
        return Ok(u64::MAX);
    }
    let health_factor = collateral_adjusted_for_liquidation_threshold
        .checked_div(collateral.amount_minted)
        .ok_or(StablecointError::MathError)?;
    Ok(health_factor)
}

pub fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(SOL_USD_FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;
    require!(price.price > 0, StablecointError::InvalidPrice);
    // the price feed returns 10^8 so we want to multiply by 10 to reach 10^9 so that the precision will be in lamports
    let price_as_u128 = price.price as u128;
    let price_in_usd = price_as_u128
        .checked_mul(PRICE_FEED_DECIMAL_ADJUSTMENT)
        .ok_or(StablecointError::MathError)?;
    // this amount is in lamports precision
    let amount_in_usd = (*amount_in_lamports as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);
    Ok(amount_in_usd as u64)
}

pub fn get_lamports_from_usd(
    amount_in_usd: &u64,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(SOL_USD_FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    require!(price.price > 0, StablecointError::InvalidPrice);
    let price_as_u128 = price.price as u128;
    let amount_in_usd_u128 = *amount_in_usd as u128;

    let price_in_usd = price_as_u128
        .checked_mul(PRICE_FEED_DECIMAL_ADJUSTMENT)
        .ok_or(StablecointError::MathError)?;

    let numerator = amount_in_usd_u128
        .checked_mul(LAMPORTS_PER_SOL as u128)
        .ok_or(StablecointError::MathError)?;

    let amount_in_lamports = numerator
        .checked_div(price_in_usd)
        .ok_or(StablecointError::MathError)?;

    Ok(amount_in_lamports as u64)
}
