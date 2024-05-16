use super::query::{query_simulate_buy_key, query_simulate_sell_key};
use crate::{
    contract::{FriendTechApp, FriendTechAppResult},
    msg::FriendTechAppExecuteMsg,
    state::{CONFIG, HOLDERS, SUPPLY},
    utils::get_issuer_addr,
    FriendTechAppError,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{coins, BankMsg, DepsMut, Env, MessageInfo, Uint128};
use cw_utils::must_pay;

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    app: FriendTechApp,
    msg: FriendTechAppExecuteMsg,
) -> FriendTechAppResult {
    match msg {
        FriendTechAppExecuteMsg::BuyKey { amount } => buy_key(deps, info, amount, app),
        FriendTechAppExecuteMsg::SellKey { amount } => sell_key(deps, info, amount, app),
    }
}

/// Anyone can call, buy key issued by the module owner
fn buy_key(
    deps: DepsMut,
    msg_info: MessageInfo,
    amount: Uint128,
    app: FriendTechApp,
) -> FriendTechAppResult {
    let buyer = &msg_info.sender;
    let config = CONFIG.load(deps.storage)?;
    let paid = must_pay(&msg_info, &config.fee_denom)?;
    let simulation_resp = query_simulate_buy_key(deps.as_ref(), amount)?;
    if simulation_resp.total_cost > paid {
        return Err(crate::FriendTechAppError::InsufficientFunds {
            required: simulation_resp.total_cost,
            paid,
        });
    }

    let old_supply = SUPPLY.load(deps.storage)?;
    SUPPLY.save(deps.storage, &(old_supply + amount))?;

    if HOLDERS.has(deps.storage, buyer) {
        let old_amount = HOLDERS.load(deps.storage, buyer)?;
        HOLDERS.save(deps.storage, buyer, &(old_amount + amount))?;
    } else {
        HOLDERS.save(deps.storage, buyer, &amount)?;
    }

    Ok(app
        .response("buy_key")
        .add_message(BankMsg::Send {
            to_address: config.issuer_fee_collector.to_string(),
            amount: coins(simulation_resp.issuer_fee.u128(), config.fee_denom),
        })
        .add_attribute("buyer", buyer.to_string())
        .add_attribute("amount", amount.to_string()))
}

/// Anyone can call, sell key issued by the module owner
fn sell_key(
    deps: DepsMut,
    msg_info: MessageInfo,
    amount: Uint128,
    app: FriendTechApp,
) -> FriendTechAppResult {
    let issuer_addr = &get_issuer_addr(deps.as_ref(), &app)?;
    let seller = &msg_info.sender;
    let config = CONFIG.load(deps.storage)?;
    let paid = must_pay(&msg_info, &config.fee_denom)?;
    let simulation_resp = query_simulate_sell_key(deps.as_ref(), amount)?;
    if simulation_resp.total_cost > paid {
        return Err(crate::FriendTechAppError::InsufficientFunds {
            required: simulation_resp.total_cost,
            paid,
        });
    }

    if !HOLDERS.has(deps.storage, seller) {
        return Err(FriendTechAppError::CannotSellMoreThanOwned {
            to_sell: amount,
            owned: Uint128::zero(),
        });
    }

    let old_amount = HOLDERS.load(deps.storage, seller)?;

    if amount <= old_amount {
        if seller == issuer_addr && amount == old_amount {
            return Err(FriendTechAppError::IssuerCannotSellLastKey {});
        }
    } else {
        return Err(FriendTechAppError::CannotSellMoreThanOwned {
            to_sell: amount,
            owned: amount,
        });
    }

    let old_supply = SUPPLY.load(deps.storage)?;

    SUPPLY.save(deps.storage, &(old_supply - amount))?;
    if old_amount == amount {
        HOLDERS.remove(deps.storage, seller);
    } else {
        HOLDERS.save(deps.storage, seller, &(old_amount - amount))?;
    }

    Ok(app
        .response("sell_key")
        .add_message(BankMsg::Send {
            to_address: config.issuer_fee_collector.to_string(),
            amount: coins(simulation_resp.issuer_fee.u128(), &config.fee_denom),
        })
        .add_message(BankMsg::Send {
            to_address: seller.to_string(),
            amount: coins(simulation_resp.price.u128(), &config.fee_denom),
        })
        .add_attribute("seller", seller.to_string())
        .add_attribute("amount", amount.to_string()))
}
