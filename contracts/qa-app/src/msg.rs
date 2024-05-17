use cosmwasm_std::{Addr, Uint128};

use crate::{contract::QAApp, state::Question};

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(QAApp, QAAppExecuteMsg, QAAppQueryMsg);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
pub struct QAAppInstantiateMsg {}

/// App execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum QAAppExecuteMsg {
    /// Anyone can call, ask a question to the module owner
    #[payable]
    Ask { content: String },
    /// Only module owner can call, answer a question
    Answer { question_id: u64, content: String },
}

#[cosmwasm_schema::cw_serde]
pub struct QAAppMigrateMsg {}

/// App query messages
#[cosmwasm_schema::cw_serde]
#[derive(cosmwasm_schema::QueryResponses, cw_orch::QueryFns)]
#[impl_into(QueryMsg)]
pub enum QAAppQueryMsg {
    #[returns(StatsResponse)]
    Stats {},
    #[returns(AskCostResponse)]
    AskCost {},
    #[returns(QuestionsResponse)]
    AnsweredQuestions {
        limit: Option<u32>,
        start_after: Option<u64>,
    },
    #[returns(QuestionsResponse)]
    UnansweredQuestions {
        limit: Option<u32>,
        start_after: Option<u64>,
    },
    #[returns(QuestionResponse)]
    Question { id: u64 },
}

#[cosmwasm_schema::cw_serde]
pub struct StatsResponse {
    pub total_question_count: u64,
}

#[cosmwasm_schema::cw_serde]
pub struct AskCostResponse {
    pub fee_denom: String,
    pub cost: Uint128,
    pub ask_fee_collector: Addr,
}

#[cosmwasm_schema::cw_serde]
pub struct QuestionsResponse {
    pub question_ids: Vec<u64>,
}

#[cosmwasm_schema::cw_serde]
pub struct QuestionResponse {
    pub question: Question,
}
