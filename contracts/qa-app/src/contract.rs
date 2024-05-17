#![allow(unused_imports)]

use crate::{
    error::QAAppError,
    handlers,
    msg::{QAAppExecuteMsg, QAAppInstantiateMsg, QAAppMigrateMsg, QAAppQueryMsg},
    replies::{self, INSTANTIATE_REPLY_ID},
    APP_VERSION, QA_APP_ID,
};

use abstract_app::{
    objects::dependency::StaticDependency, std::manager::ModuleInstallConfig, AppContract,
};
use abstract_interface::{AbstractInterfaceError, DependencyCreation, InstallConfig};
use cosmwasm_std::Response;
use friend_tech_app::{
    msg::FriendTechAppInstantiateMsg, Friendtech, FRIEND_TECH_APP_ID, FRIEND_TECH_APP_VERSION,
};

/// The type of the result returned by your app's entry points.
pub type QAAppResult<T = Response> = Result<T, QAAppError>;

/// The type of the app that is used to build your app and access the Abstract SDK features.
pub type QAApp =
    AppContract<QAAppError, QAAppInstantiateMsg, QAAppExecuteMsg, QAAppQueryMsg, QAAppMigrateMsg>;

const QA_APP: QAApp = QAApp::new(QA_APP_ID, APP_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_migrate(handlers::migrate_handler)
    .with_replies(&[(INSTANTIATE_REPLY_ID, replies::instantiate_reply)])
    .with_dependencies(&[
        // This module application is dependent on another modules: the Friend Tech module.
        StaticDependency::new(FRIEND_TECH_APP_ID, &[FRIEND_TECH_APP_VERSION]),
    ]);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(QA_APP, QAApp);

abstract_app::cw_orch_interface!(QA_APP, QAApp, Qa);

// TODO: add to docmuentation
// https://linear.app/abstract-sdk/issue/ABS-414/add-documentation-on-dependencycreation-trait
#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::Qa<Chain>
{
    type DependenciesConfig = FriendTechAppInstantiateMsg;

    fn dependency_install_configs(
        configuration: Self::DependenciesConfig,
    ) -> Result<Vec<ModuleInstallConfig>, AbstractInterfaceError> {
        let friend_tech_dependency_install_configs: Vec<ModuleInstallConfig> =
            <Friendtech<Chain> as DependencyCreation>::dependency_install_configs(
                cosmwasm_std::Empty {},
            )?;

        let friend_tech_install_config = <Friendtech<Chain> as InstallConfig>::install_config(
            // &friend_tech_app::msg::FriendTechAppInstantiateMsg {
            //     username: "test_user",
            //     issuer_fee_collector: todo!(),
            //     fee_denom: todo!(),
            // },
            &configuration,
        )?;

        Ok([
            friend_tech_dependency_install_configs,
            vec![friend_tech_install_config],
        ]
        .concat())
    }
}
