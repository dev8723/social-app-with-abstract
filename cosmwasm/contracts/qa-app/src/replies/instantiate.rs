use crate::contract::{QAApp, QAAppResult};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, Reply};

pub fn instantiate_reply(_deps: DepsMut, _env: Env, app: QAApp, _reply: Reply) -> QAAppResult {
    Ok(app.response("instantiate_reply"))
}
