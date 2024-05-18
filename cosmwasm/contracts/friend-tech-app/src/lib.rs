mod api;
pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
mod replies;
pub mod state;
mod utils;

pub use error::FriendTechAppError;

/// The version of your app
pub const FRIEND_TECH_APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use api::{FriendTech, FriendTechInterface};
pub use contract::interface::Friendtech;

pub const MY_NAMESPACE: &str = "bull-market-lab";
pub const FRIEND_TECH_APP_NAME: &str = "friend-tech-app";
pub const FRIEND_TECH_APP_ID: &str =
    const_format::formatcp!("{MY_NAMESPACE}:{FRIEND_TECH_APP_NAME}");
