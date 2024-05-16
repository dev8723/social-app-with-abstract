pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
mod replies;
pub mod state;
mod utils;

pub use error::FriendTechAppError;

/// The version of your app
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use contract::interface::FriendTechAppInterface;

pub const MY_NAMESPACE: &str = "bull-market-lab";
pub const FRIEND_TECH_APP_NAME: &str = "friend-tech";
pub const FRIEND_TECH_APP_ID: &str =
    const_format::formatcp!("{MY_NAMESPACE}:{FRIEND_TECH_APP_NAME}");
