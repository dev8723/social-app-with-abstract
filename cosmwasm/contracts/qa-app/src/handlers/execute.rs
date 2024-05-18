use super::query::query_ask_cost;
use crate::{
    contract::{QAApp, QAAppResult},
    msg::QAAppExecuteMsg,
    state::{Question, ANSWERED_QUESTIONS, NEXT_QUESTION_ID, UNANSWERED_QUESTIONS},
    utils::get_account_owner_addr,
    QAAppError,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{coins, BankMsg, DepsMut, Env, MessageInfo};
use cw_utils::{must_pay, nonpayable};

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    app: QAApp,
    msg: QAAppExecuteMsg,
) -> QAAppResult {
    // TODO: verify caller must be an abstract account?
    match msg {
        QAAppExecuteMsg::Ask { content } => ask(deps, info, content, app),
        QAAppExecuteMsg::Answer {
            question_id,
            content,
        } => answer(deps, info, question_id, content, app),
    }
}

/// Anyone can call, ask a question to the module owner
fn ask(deps: DepsMut, msg_info: MessageInfo, content: String, app: QAApp) -> QAAppResult {
    let asker = &msg_info.sender;
    let cost_resp = query_ask_cost(deps.as_ref(), &app)?;
    let paid = must_pay(&msg_info, &cost_resp.fee_denom)?;
    if cost_resp.cost > paid {
        return Err(crate::QAAppError::InsufficientFunds {
            required: cost_resp.cost,
            paid,
        });
    }

    let next_question_id = NEXT_QUESTION_ID.load(deps.storage)?;
    UNANSWERED_QUESTIONS.save(
        deps.storage,
        next_question_id,
        &Question {
            id: next_question_id,
            asker: asker.clone(),
            question_content: content.clone(),
            answered: false,
            answer_content: None,
        },
    )?;

    NEXT_QUESTION_ID.save(deps.storage, &(next_question_id + 1))?;

    Ok(app
        .response("ask")
        .add_message(BankMsg::Send {
            to_address: cost_resp.ask_fee_collector.to_string(),
            amount: coins(cost_resp.cost.u128(), cost_resp.fee_denom.clone()),
        })
        .add_attribute("asker", asker)
        .add_attribute("question_id", next_question_id.to_string())
        .add_attribute("question_content", content)
        .add_attribute("cost", cost_resp.cost)
        .add_attribute("fee_denom", cost_resp.fee_denom)
        .add_attribute("ask_fee_collector", cost_resp.ask_fee_collector))
}

/// Only module owner can call, answer a question
fn answer(
    deps: DepsMut,
    msg_info: MessageInfo,
    question_id: u64,
    content: String,
    app: QAApp,
) -> QAAppResult {
    nonpayable(&msg_info)?;

    let answerer = &msg_info.sender;
    if answerer != get_account_owner_addr(deps.as_ref(), &app)? {
        return Err(QAAppError::OnlyAccountOwnerCanAnswer {});
    }

    if !UNANSWERED_QUESTIONS.has(deps.storage, question_id) {
        return Err(QAAppError::QuestionNotFoundInUnansweredQuestions { question_id });
    }

    let question = UNANSWERED_QUESTIONS.load(deps.storage, question_id)?;
    UNANSWERED_QUESTIONS.remove(deps.storage, question_id);

    ANSWERED_QUESTIONS.save(
        deps.storage,
        question.id,
        &Question {
            id: question.id,
            asker: question.asker.clone(),
            question_content: question.question_content,
            answered: true,
            answer_content: Some(content.clone()),
        },
    )?;

    Ok(app
        .response("answer")
        .add_attribute("asker", question.asker)
        .add_attribute("question_id", question_id.to_string())
        .add_attribute("answer_content", content))
}
