use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cosmwasm_schema::cw_serde]
pub struct Config {
    pub username: String,
    pub fee_denom: String,
    pub issuer_fee_collector: Addr,
}

pub const CONFIG: Item<Config> = Item::new("CONFIG");
pub const SUPPLY: Item<Uint128> = Item::new("SUPPLY");
pub const HOLDERS: Map<&Addr, Uint128> = Map::new("HOLDERS");
