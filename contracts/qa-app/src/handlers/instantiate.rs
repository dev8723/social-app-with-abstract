use crate::{
    contract::{QAApp, QAAppResult},
    msg::QAAppInstantiateMsg,
    state::NEXT_QUESTION_ID,
    utils::get_account_owner_addr,
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    app: QAApp,
    _msg: QAAppInstantiateMsg,
) -> QAAppResult {
    let account_owner_addr = &get_account_owner_addr(deps.as_ref(), &app)?;

    NEXT_QUESTION_ID.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("account_owner", account_owner_addr))
}
