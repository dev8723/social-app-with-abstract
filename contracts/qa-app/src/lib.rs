pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
mod replies;
pub mod state;
mod utils;

pub use error::QAAppError;

/// The version of your app
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use contract::interface::Qa;

pub const MY_NAMESPACE: &str = "bull-market-lab";
pub const QA_APP_NAME: &str = "qa-app";
pub const QA_APP_ID: &str = const_format::formatcp!("{MY_NAMESPACE}:{QA_APP_NAME}");
