use crate::{
    contract::{FriendTechApp, FriendTechAppResult},
    msg::{
        BuyKeyCostResponse, FriendTechAppQueryMsg, HoldersResponse, HoldingResponse,
        IssuerResponse, SellKeyCostResponse,
    },
    state::{CONFIG, HOLDERS, SUPPLY},
    utils::{calculate_buy_price, calculate_sell_price, multiply_percentage},
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, Order, StdResult, Uint128};
use cw_storage_plus::Bound;

const DEFAULT_QUERY_LIMIT: u32 = 10;
const MAX_QUERY_LIMIT: u32 = 30;

const ISSUER_FEE_PERCENTAGE: u32 = 5;

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _app: &FriendTechApp,
    msg: FriendTechAppQueryMsg,
) -> FriendTechAppResult<Binary> {
    match msg {
        FriendTechAppQueryMsg::Issuer {} => to_json_binary(&query_issuer(deps)?),
        FriendTechAppQueryMsg::BuyKeyCost { amount } => {
            to_json_binary(&query_buy_key_cost(deps, amount)?)
        }
        FriendTechAppQueryMsg::SellKeyCost { amount } => {
            to_json_binary(&query_sell_key_cost(deps, amount)?)
        }
        FriendTechAppQueryMsg::Holders { limit, start_after } => {
            to_json_binary(&query_holders(deps, limit, start_after)?)
        }
        FriendTechAppQueryMsg::Holding { holder } => to_json_binary(&query_holding(deps, holder)?),
    }
    .map_err(Into::into)
}

fn query_issuer(deps: Deps) -> StdResult<IssuerResponse> {
    let config = CONFIG.load(deps.storage)?;
    let supply = SUPPLY.load(deps.storage)?;
    Ok(IssuerResponse {
        username: config.username,
        fee_denom: config.fee_denom,
        issuer_fee_collector: config.issuer_fee_collector,
        supply,
    })
}

pub fn query_buy_key_cost(deps: Deps, amount: Uint128) -> StdResult<BuyKeyCostResponse> {
    let old_supply = SUPPLY.load(deps.storage)?;
    let price = calculate_buy_price(old_supply, amount);
    let issuer_fee = multiply_percentage(price, ISSUER_FEE_PERCENTAGE);
    Ok(BuyKeyCostResponse {
        price,
        issuer_fee,
        total_cost: price + issuer_fee,
    })
}

pub fn query_sell_key_cost(deps: Deps, amount: Uint128) -> StdResult<SellKeyCostResponse> {
    let old_supply = SUPPLY.load(deps.storage)?;
    let price = calculate_sell_price(old_supply - amount, amount);
    let issuer_fee = multiply_percentage(price, ISSUER_FEE_PERCENTAGE);
    Ok(SellKeyCostResponse {
        price,
        issuer_fee,
        total_cost: issuer_fee,
    })
}

fn query_holders(
    deps: Deps,
    limit: Option<u32>,
    start_after: Option<String>,
) -> StdResult<HoldersResponse> {
    let holders = match start_after {
        Some(start_after) => HOLDERS.range(
            deps.storage,
            Some(Bound::exclusive(&deps.api.addr_validate(&start_after)?)),
            None,
            Order::Ascending,
        ),
        None => HOLDERS.range(deps.storage, None, None, Order::Ascending),
    }
    .take(limit.unwrap_or(DEFAULT_QUERY_LIMIT).min(MAX_QUERY_LIMIT) as usize)
    .map(|item| item.map(|(addr, _)| addr))
    .collect::<StdResult<Vec<_>>>()?;
    Ok(HoldersResponse { holders })
}

fn query_holding(deps: Deps, holder: String) -> StdResult<HoldingResponse> {
    let holder_addr = deps.api.addr_validate(&holder)?;
    let amount = HOLDERS
        .may_load(deps.storage, &holder_addr)?
        .unwrap_or(Uint128::zero());
    Ok(HoldingResponse { amount })
}
