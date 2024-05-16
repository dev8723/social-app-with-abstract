use cosmwasm_std::{Addr, Deps, Uint128};

use crate::{contract::FriendTechApp, FriendTechAppError};

fn calculate_price(supply: Uint128, amount: Uint128) -> Uint128 {
    let sum1 = if supply.is_zero() {
        Uint128::zero()
    } else {
        ((supply - Uint128::one())
            * supply
            * (Uint128::from(2u64) * (supply - Uint128::one()) + Uint128::one()))
            / Uint128::from(6_u64)
    };

    let sum2 = if supply.is_zero() && amount == Uint128::one() {
        Uint128::zero()
    } else {
        ((supply - Uint128::one() + amount)
            * (supply + amount)
            * ((supply - Uint128::one() + amount) * Uint128::from(2_u64) + Uint128::one()))
            / Uint128::from(6_u64)
    };

    let summation = sum2 - sum1;
    (summation * Uint128::from(1_000_000_u64)) / Uint128::from(1_600_u64)
}

pub fn calculate_buy_price(supply_before_buy: Uint128, buy_amount: Uint128) -> Uint128 {
    calculate_price(supply_before_buy, buy_amount)
}

pub fn calculate_sell_price(supply_before_sell: Uint128, sell_amount: Uint128) -> Uint128 {
    // We need this to make sure price is the same across buy and sell
    // e.g. old supply is 5, now buy 10 memberships, new supply is 15
    // Now sell 10 memberships, new supply is 5, price to buy 10 memberships should be the same as price to sell 10 memberships
    // Because before supply and after supply is the same
    calculate_price(supply_before_sell - sell_amount, sell_amount)
}

pub fn multiply_percentage(price: Uint128, percentage: u32) -> Uint128 {
    (price * Uint128::from(percentage)) / Uint128::from(100_u64)
}

pub fn get_issuer_addr(deps: Deps, app: &FriendTechApp) -> Result<Addr, FriendTechAppError> {
    let issuer = app.admin.query_account_owner(deps)?.admin;
    match issuer {
        Some(addr) => Ok(deps.api.addr_validate(&addr)?),
        None => Err(FriendTechAppError::AccountOwnerMustBeSetToIssueKey {}),
    }
}
