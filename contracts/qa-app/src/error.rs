use abstract_app::sdk::AbstractSdkError;
use abstract_app::std::AbstractError;
use abstract_app::AppError;
use cosmwasm_std::{StdError, Uint128};
use cw_asset::AssetError;
use cw_controllers::AdminError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum QAAppError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Abstract(#[from] AbstractError),

    #[error("{0}")]
    AbstractSdk(#[from] AbstractSdkError),

    #[error("{0}")]
    Asset(#[from] AssetError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("{0}")]
    DappError(#[from] AppError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Account owner must be set to answer any question")]
    AccountOwnerMustBeSetToAnswer {},

    #[error("Only account owner can answer the question")]
    OnlyAccountOwnerCanAnswer {},

    #[error("Question not found in unanswered questions, question_id: {question_id}")]
    QuestionNotFoundInUnansweredQuestions { question_id: u64 },

    #[error("Insufficient funds, required: {required}, paid: {paid}")]
    InsufficientFunds { required: Uint128, paid: Uint128 },
}
