use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

/// An incrementing number to assign unique IDs to questions
pub const NEXT_QUESTION_ID: Item<u64> = Item::new("NEXT_QUESTION_ID");

#[cosmwasm_schema::cw_serde]
pub struct Question {
    pub id: u64,
    pub asker: Addr,
    pub question_content: String,
    pub answered: bool,
    pub answer_content: Option<String>,
}

/// Key is the question ID, value is the question struct
pub const UNANSWERED_QUESTIONS: Map<u64, Question> = Map::new("UNANSWERED_QUESTIONS");

/// Key is the question ID, value is the question struct
pub const ANSWERED_QUESTIONS: Map<u64, Question> = Map::new("ANSWERED_QUESTIONS");
