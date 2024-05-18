use crate::contract::{FriendTechApp, FriendTechAppResult};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, Reply};

pub fn instantiate_reply(
    _deps: DepsMut,
    _env: Env,
    app: FriendTechApp,
    _reply: Reply,
) -> FriendTechAppResult {
    Ok(app.response("instantiate_reply"))
}
