use crate::{
    contract::{QAApp, QAAppResult},
    msg::QAAppMigrateMsg,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env};

/// Handle the app migrate msg
/// The top-level Abstract app does version checking and dispatches to this handler
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    app: QAApp,
    _msg: QAAppMigrateMsg,
) -> QAAppResult {
    Ok(app.response("migrate"))
}
