use cosmwasm_std::{Addr, Deps};

use crate::{contract::QAApp, QAAppError};

pub fn get_account_owner_addr(deps: Deps, app: &QAApp) -> Result<Addr, QAAppError> {
    let issuer = app.admin.query_account_owner(deps)?.admin;
    match issuer {
        Some(addr) => Ok(deps.api.addr_validate(&addr)?),
        None => Err(QAAppError::AccountOwnerMustBeSetToAnswer {}),
    }
}
