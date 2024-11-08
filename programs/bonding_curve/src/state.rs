use crate::consts::INITIAL_LAMPORTS_FOR_POOL;
use crate::consts::INITIAL_PRICE_DIVIDER;
use crate::consts::PROPORTION;
use crate::errors::CustomError;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[account]
pub struct CurveConfiguration {
    pub fees: f64,
}

impl CurveConfiguration {
    pub const SEED: &'static str = "CurveConfiguration";

    // Discriminator (8) + f64 (8)
    pub const ACCOUNT_SIZE: usize = 8 + 32 + 8;

    pub fn new(fees: f64) -> Self {
        Self { fees }
    }
}

#[account]
pub struct LiquidityProvider {
    pub shares: u64, // The number of shares this provider holds in the liquidity pool ( didnt add to contract now )
}

impl LiquidityProvider {
    pub const SEED_PREFIX: &'static str = "LiqudityProvider"; // Prefix for generating PDAs

    // Discriminator (8) + f64 (8)
    pub const ACCOUNT_SIZE: usize = 8 + 8;
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////
//
//              Linear bonding curve swap
//
/////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////
//
//  Linear bonding curve : S = T * P ( here, p is constant that show initial price )
//  SOL amount => S
//  Token amount => T
//  Initial Price => P
//
//  SOL amount to buy Token a => S_a = ((T_a  + 1) * T_a / 2) * P
//  SOL amount to buy Token b => S_b = ((T_b + 1) * T_b / 2) * P
//
//  If amount a of token sold, and x (x = b - a) amount of token is bought (b > a)
//  S = S_a - S_b = ((T_b + T_a + 1) * (T_b - T_a) / 2) * P
//
//
// let s = amount;
// let T_a = reserve_token - amount;
// let T_b = reserve_token;
// let P = INITIAL_PRICE_DIVIDER;

// let amount_inc = self
//     .reserve_token
//     .checked_mul(2)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?
//     .checked_add(amount)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?
//     .checked_add(1)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;

// let multiplier = amount
//     .checked_div(2)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;

// msg!("multiplier : {}", 200);
// let amount_out = amount_inc
//     .checked_mul(multiplier)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?
//     .checked_mul(INITIAL_PRICE_DIVIDER)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;

// let amount_in_float = convert_to_float(amount, token_accounts.0.decimals);

// // Convert the input amount to float with decimals considered
// let amount_float = convert_to_float(amount, token_accounts.0.decimals);

// Apply fees
// let adjusted_amount_in_float = amount_float
//     .div(100_f64)
//     .mul(100_f64.sub(bonding_configuration_account.fees));

// let adjusted_amount =
//     convert_from_float(adjusted_amount_in_float, token_accounts.0.decimals);

// Linear bonding curve calculations
// let p = 1 / INITIAL_PRICE_DIVIDER;
// let t_a = convert_to_float(self.reserve_token, token_accounts.0.decimals);
// let t_b = t_a + adjusted_amount_in_float;

// let s_a = ((t_a + 1.0) * t_a / 2.0) * p;
// let s_b = ((t_b + 1.0) * t_b / 2.0) * p;

// let s = s_b - s_a;

// let amount_out = convert_from_float(s, sol_token_accounts.0.decimals);

// let new_reserves_one = self
//     .reserve_token
//     .checked_add(amount)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;
// msg!("new_reserves_one : {}", );
// let new_reserves_two = self
//     .reserve_sol
//     .checked_sub(amount_out)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;

// msg!("new_reserves_two : {}", );
// self.update_reserves(new_reserves_one, new_reserves_two)?;

// let adjusted_amount_in_float = convert_to_float(amount, token_accounts.0.decimals)
//     .div(100_f64)
//     .mul(100_f64.sub(bonding_configuration_account.fees));

// let adjusted_amount =
//     convert_from_float(adjusted_amount_in_float, token_accounts.0.decimals);

// let denominator_sum = self
//     .reserve_token
//     .checked_add(adjusted_amount)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;

// let numerator_mul = self
//     .reserve_sol
//     .checked_mul(adjusted_amount)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;

// let amount_out = numerator_mul
//     .checked_div(denominator_sum)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;

// let new_reserves_one = self
//     .reserve_token
//     .checked_add(amount)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;
// let new_reserves_two = self
//     .reserve_sol
//     .checked_sub(amount_out)
//     .ok_or(CustomError::OverflowOrUnderflowOccurred)?;

// self.update_reserves(new_reserves_one, new_reserves_two)?;
// let amount_out = amount.checked_div(2)

// self.transfer_token_to_pool(
//     token_accounts.2,
//     token_accounts.1,
//     1000 as u64,
//     authority,
//     token_program,
// )?;

// self.transfer_token_from_pool(
//     sol_token_accounts.1,
// sol_token_accounts.2,
//     1000 as u64,
//     token_program,
// )?;

// let amount_out: u64 = 1000000000000;
// let amount_out = ((((2 * self.reserve_token + 1) * (2 * self.reserve_token + 1) + amount) as f64).sqrt() as u64 - ( 2 * self.reserve_token + 1)) / 2;

// let token_sold = match self.total_supply.checked_sub(self.reserve_token) {
//     Some(value) if value == 0 => 1_000_000_000,
//     Some(value) => value,
//     None => return err!(CustomError::OverflowOrUnderflowOccurred),
// };

// msg!("token_sold: {}", token_sold);

// let amount_out: u64 = calculate_amount_out(token_sold, amount)?;
// msg!("amount_out: {}", amount_out);

// if self.reserve_token < amount_out {
//     return err!(CustomError::InvalidAmount);
// }
// self.reserve_sol += amount;
// self.reserve_token -= amount_out;

// Function to perform the calculation with error handling

// fn calculate_amount_out(reserve_token_decimal: u64, amount_decimal: u64) -> Result<u64> {
//     let reserve_token = reserve_token_decimal.checked_div(1000000000).ok_or(CustomError::OverflowOrUnderflowOccurred)?;
//     let amount = amount_decimal.checked_div(1000000000).ok_or(CustomError::OverflowOrUnderflowOccurred)?;
//     msg!("Starting calculation with reserve_token: {}, amount: {}", reserve_token, amount);
//     let two_reserve_token = reserve_token.checked_mul(2).ok_or(CustomError::OverflowOrUnderflowOccurred)?;
//     msg!("two_reserve_token: {}", two_reserve_token);

//     let one_added = two_reserve_token.checked_add(1).ok_or(CustomError::OverflowOrUnderflowOccurred)?;
//     msg!("one_added: {}", one_added);

//     let squared = one_added.checked_mul(one_added).ok_or(CustomError::OverflowOrUnderflowOccurred)?;
//     msg!("squared: {}", squared);

//     let amount_divided = amount.checked_mul(INITIAL_PRICE_DIVIDER).ok_or(CustomError::OverflowOrUnderflowOccurred)?;
//     msg!("amount_divided: {}", amount_divided);

//     let amount_added = squared.checked_add(amount_divided).ok_or(CustomError::OverflowOrUnderflowOccurred)?;
//     msg!("amount_added: {}", amount_added);

//     // Convert to f64 for square root calculation
//     let sqrt_result = (amount_added as f64).sqrt();
//     msg!("sqrt_result: {}", sqrt_result);

//     // Check if sqrt_result can be converted back to u64 safely
//     if sqrt_result < 0.0 {
//         msg!("Error: Negative sqrt_result");
//         return err!(CustomError::NegativeNumber);
//     }

//     let sqrt_u64 = sqrt_result as u64;
//     msg!("sqrt_u64: {}", sqrt_u64);

//     let subtract_one = sqrt_u64.checked_sub(one_added).ok_or(CustomError::OverflowOrUnderflowOccurred)?;
//     msg!("subtract_one: {}", subtract_one);

//     let amount_out = subtract_one.checked_div(2).ok_or(CustomError::OverflowOrUnderflowOccurred)?;
//     msg!("amount_out: {}", amount_out);
//     let amount_out_decimal = amount_out.checked_mul(1000000000)
//     Ok(amount_out)
// }
