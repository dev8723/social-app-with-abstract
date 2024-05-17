use abstract_app::sdk::{
    features::{AccountIdentification, Dependencies, ModuleIdentification},
    std::objects::module::ModuleId,
    AbstractSdkResult, AppInterface, ModuleInterface,
};
use cosmwasm_std::{Addr, CosmosMsg, Deps, Uint128};

use crate::{
    msg::{BuyKeyCostResponse, FriendTechAppExecuteMsg, FriendTechAppQueryMsg, IssuerResponse},
    FRIEND_TECH_APP_ID,
};

#[derive(Clone)]
pub struct FriendTech<'a, T: FriendTechInterface> {
    base: &'a T,
    module_id: ModuleId<'a>,
    deps: Deps<'a>,
}

// API for Abstract SDK users
/// Interact with the friend tech app in your module.
pub trait FriendTechInterface: AccountIdentification + Dependencies + ModuleIdentification {
    /// Construct a new friend_tech interface
    fn friend_tech<'a>(&'a self, deps: Deps<'a>) -> FriendTech<Self> {
        FriendTech {
            base: self,
            deps,
            module_id: FRIEND_TECH_APP_ID,
        }
    }
}

impl<T: AccountIdentification + Dependencies + ModuleIdentification> FriendTechInterface for T {}

impl<'a, T: FriendTechInterface> FriendTech<'a, T> {
    /// Get address of this module
    pub fn module_address(&self) -> AbstractSdkResult<Addr> {
        self.base.modules(self.deps).module_address(self.module_id)
    }

    /// Buy key
    pub fn buy_key(&self, amount: Uint128) -> AbstractSdkResult<CosmosMsg> {
        self.base
            .apps(self.deps)
            .execute(self.module_id, FriendTechAppExecuteMsg::BuyKey { amount })
    }

    /// Sell key
    pub fn sell_key(&self, amount: Uint128) -> AbstractSdkResult<CosmosMsg> {
        self.base
            .apps(self.deps)
            .execute(self.module_id, FriendTechAppExecuteMsg::SellKey { amount })
    }

    /// Query issuer
    pub fn query_issuer(&self) -> AbstractSdkResult<IssuerResponse> {
        self.base
            .apps(self.deps)
            .query(self.module_id, FriendTechAppQueryMsg::Issuer {})
    }

    /// Query the cost of buying key
    pub fn query_buy_key_cost(&self, amount: Uint128) -> AbstractSdkResult<BuyKeyCostResponse> {
        self.base
            .apps(self.deps)
            .query(self.module_id, FriendTechAppQueryMsg::BuyKeyCost { amount })
    }

    /// Query the cost of selling key
    pub fn query_sell_key_cost(&self, amount: Uint128) -> AbstractSdkResult<BuyKeyCostResponse> {
        self.base.apps(self.deps).query(
            self.module_id,
            FriendTechAppQueryMsg::SellKeyCost { amount },
        )
    }
}
