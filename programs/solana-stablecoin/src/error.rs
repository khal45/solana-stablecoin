use anchor_lang::prelude::*;

#[error_code]
pub enum StablecointError {
    #[msg("Overflow, underflow or some other math error occured")]
    MathError,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Health factor below minimum")]
    BelowMinimumHealthFactor,
    #[msg("Above minimum health factor")]
    AboveMinimumHealthFactor,
}
