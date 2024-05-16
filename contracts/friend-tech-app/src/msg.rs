use crate::contract::FriendTechApp;

use cosmwasm_std::{Addr, Uint128};

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(
    FriendTechApp,
    FriendTechAppExecuteMsg,
    FriendTechAppQueryMsg
);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
pub struct FriendTechAppInstantiateMsg {
    pub username: String,
    pub issuer_fee_collector: String,
    pub fee_denom: String,
}

/// App execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum FriendTechAppExecuteMsg {
    /// Anyone can call, buy key issued by the module owner
    #[payable]
    BuyKey { amount: Uint128 },
    /// Anyone can call, sell key issued by the module owner
    #[payable]
    SellKey { amount: Uint128 },
}

#[cosmwasm_schema::cw_serde]
pub struct FriendTechAppMigrateMsg {}

/// App query messages
#[cosmwasm_schema::cw_serde]
#[derive(cosmwasm_schema::QueryResponses, cw_orch::QueryFns)]
#[impl_into(QueryMsg)]
pub enum FriendTechAppQueryMsg {
    #[returns(IssuerResponse)]
    Issuer {},
    #[returns(SimulateBuyKeyResponse)]
    SimulateBuyKey { amount: Uint128 },
    #[returns(SimulateSellKeyResponse)]
    SimulateSellKey { amount: Uint128 },
    #[returns(HoldersResponse)]
    Holders {
        limit: Option<u32>,
        start_after: Option<String>,
    },
    #[returns(HoldingResponse)]
    Holding { holder: String },
}

#[cosmwasm_schema::cw_serde]
pub struct IssuerResponse {
    pub username: String,
    pub fee_denom: String,
    pub issuer_fee_collector: Addr,
    pub supply: Uint128,
}

#[cosmwasm_schema::cw_serde]
pub struct SimulateBuyKeyResponse {
    /// Price of buying amount of key
    pub price: Uint128,
    /// Fee charged by the issuer
    pub issuer_fee: Uint128,
    /// Total cost of the transaction
    pub total_cost: Uint128,
}

#[cosmwasm_schema::cw_serde]
pub struct SimulateSellKeyResponse {
    /// Price of selling amount of key
    pub price: Uint128,
    /// Fee charged by the issuer
    pub issuer_fee: Uint128,
    /// Total cost of the transaction
    pub total_cost: Uint128,
}

#[cosmwasm_schema::cw_serde]
pub struct HoldersResponse {
    pub holders: Vec<Addr>,
}

#[cosmwasm_schema::cw_serde]
pub struct HoldingResponse {
    pub amount: Uint128,
}
