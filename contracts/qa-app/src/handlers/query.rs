use crate::{
    contract::{QAApp, QAAppResult},
    msg::{AskCostResponse, QAAppQueryMsg, QuestionResponse, QuestionsResponse, StatsResponse},
    state::{ANSWERED_QUESTIONS, NEXT_QUESTION_ID, UNANSWERED_QUESTIONS},
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, Order, StdError, StdResult, Uint128};
use cw_storage_plus::Bound;
use friend_tech_app::FriendTechInterface;

const DEFAULT_QUERY_LIMIT: u32 = 10;
const MAX_QUERY_LIMIT: u32 = 30;

pub fn query_handler(
    deps: Deps,
    _env: Env,
    app: &QAApp,
    msg: QAAppQueryMsg,
) -> QAAppResult<Binary> {
    match msg {
        QAAppQueryMsg::Stats {} => to_json_binary(&query_stats(deps)?),
        QAAppQueryMsg::AskCost {} => to_json_binary(&query_ask_cost(deps, app)?),
        QAAppQueryMsg::AnsweredQuestions { limit, start_after } => {
            to_json_binary(&query_answered_questions(deps, limit, start_after)?)
        }
        QAAppQueryMsg::UnansweredQuestions { limit, start_after } => {
            to_json_binary(&query_unanswered_questions(deps, limit, start_after)?)
        }
        QAAppQueryMsg::Question { id } => to_json_binary(&query_question(deps, id)?),
    }
    .map_err(Into::into)
}

fn query_stats(deps: Deps) -> StdResult<StatsResponse> {
    Ok(StatsResponse {
        total_question_count: NEXT_QUESTION_ID.load(deps.storage)?,
    })
}

pub fn query_ask_cost(deps: Deps, app: &QAApp) -> StdResult<AskCostResponse> {
    let friend_tech = app.friend_tech(deps);
    let answerer = friend_tech.query_issuer().unwrap();
    let cost = friend_tech.query_buy_key_cost(Uint128::one()).unwrap();

    Ok(AskCostResponse {
        fee_denom: answerer.fee_denom,
        cost: cost.total_cost,
        ask_fee_collector: answerer.issuer_fee_collector,
    })
}

fn query_answered_questions(
    deps: Deps,
    limit: Option<u32>,
    start_after: Option<u64>,
) -> StdResult<QuestionsResponse> {
    let question_ids = match start_after {
        Some(start_after) => ANSWERED_QUESTIONS.range(
            deps.storage,
            Some(Bound::exclusive(start_after)),
            None,
            Order::Ascending,
        ),
        None => ANSWERED_QUESTIONS.range(deps.storage, None, None, Order::Ascending),
    }
    .take(limit.unwrap_or(DEFAULT_QUERY_LIMIT).min(MAX_QUERY_LIMIT) as usize)
    .map(|item| item.map(|(addr, _)| addr))
    .collect::<StdResult<Vec<_>>>()?;
    Ok(QuestionsResponse { question_ids })
}

fn query_unanswered_questions(
    deps: Deps,
    limit: Option<u32>,
    start_after: Option<u64>,
) -> StdResult<QuestionsResponse> {
    let question_ids = match start_after {
        Some(start_after) => UNANSWERED_QUESTIONS.range(
            deps.storage,
            Some(Bound::exclusive(start_after)),
            None,
            Order::Ascending,
        ),
        None => UNANSWERED_QUESTIONS.range(deps.storage, None, None, Order::Ascending),
    }
    .take(limit.unwrap_or(DEFAULT_QUERY_LIMIT).min(MAX_QUERY_LIMIT) as usize)
    .map(|item| item.map(|(addr, _)| addr))
    .collect::<StdResult<Vec<_>>>()?;
    Ok(QuestionsResponse { question_ids })
}

fn query_question(deps: Deps, question_id: u64) -> StdResult<QuestionResponse> {
    let question = if UNANSWERED_QUESTIONS.has(deps.storage, question_id) {
        UNANSWERED_QUESTIONS.load(deps.storage, question_id)?
    } else if ANSWERED_QUESTIONS.has(deps.storage, question_id) {
        ANSWERED_QUESTIONS.load(deps.storage, question_id)?
    } else {
        return Err(StdError::not_found("Question not found"));
    };
    Ok(QuestionResponse { question })
}
