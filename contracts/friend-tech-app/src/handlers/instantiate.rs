use crate::{
    contract::{FriendTechApp, FriendTechAppResult},
    msg::FriendTechAppInstantiateMsg,
    state::{Config, CONFIG, HOLDERS, SUPPLY},
    utils::get_account_owner_addr,
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    app: FriendTechApp,
    msg: FriendTechAppInstantiateMsg,
) -> FriendTechAppResult {
    let account_owner_addr = &get_account_owner_addr(deps.as_ref(), &app)?;

    let issuer_fee_collector = deps.api.addr_validate(&msg.issuer_fee_collector)?;

    let config: Config = Config {
        username: msg.username.clone(),
        fee_denom: msg.fee_denom.clone(),
        issuer_fee_collector,
    };
    CONFIG.save(deps.storage, &config)?;
    SUPPLY.save(deps.storage, &Uint128::one())?;
    HOLDERS.save(deps.storage, account_owner_addr, &Uint128::one())?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("account_owner", account_owner_addr)
        .add_attribute("username", msg.username)
        .add_attribute("fee_denom", msg.fee_denom)
        .add_attribute("issuer_fee_collector", msg.issuer_fee_collector))
}
