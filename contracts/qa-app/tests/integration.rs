use friend_tech_app::{msg::FriendTechAppInstantiateMsg, Friendtech, FRIEND_TECH_APP_ID};
use qa_app::{
    contract::interface::Qa,
    msg::{
        QAAppExecuteMsgFns, QAAppInstantiateMsg, QAAppQueryMsgFns, QuestionResponse,
        QuestionsResponse, StatsResponse,
    },
    state::Question,
    QAAppError, MY_NAMESPACE,
};

use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application, Environment};
use cosmwasm_std::{coins, Uint128};
// Use prelude to get all the necessary imports
use cw_orch::{anyhow, prelude::*};

const DENOM: &str = "ucosm";

const USER1: &str = "user1";

struct TestEnv<Env: CwEnv> {
    abs: AbstractClient<Env>,
    app: Application<Env, Qa<Env>>,
}

impl TestEnv<MockBech32> {
    /// Set up the test environment with an Account that has the App installed
    fn setup() -> anyhow::Result<TestEnv<MockBech32>> {
        // Create a sender and mock env
        let mock = MockBech32::new("mock");
        let sender = mock.sender();
        let namespace = Namespace::new(MY_NAMESPACE)?;

        // You can set up Abstract with a builder.
        let abs_client = AbstractClient::builder(mock).build()?;
        // The app supports setting balances for addresses and configuring ANS.

        // Publish the app
        let qa_publisher = abs_client.publisher_builder(namespace).build()?;
        let friend_tech_publisher = abs_client
            .publisher_builder(Namespace::from_id(FRIEND_TECH_APP_ID)?)
            .build()?;
        qa_publisher.publish_app::<Qa<_>>()?;
        friend_tech_publisher.publish_app::<Friendtech<_>>()?;

        let app = qa_publisher
            .account()
            .install_app_with_dependencies::<Qa<_>>(
                &QAAppInstantiateMsg {},
                FriendTechAppInstantiateMsg {
                    username: "test".to_string(),
                    fee_denom: DENOM.to_string(),
                    issuer_fee_collector: sender.to_string(),
                },
                &[],
            )?;

        Ok(TestEnv {
            abs: abs_client,
            app,
        })
    }
}

#[test]
fn successful_install() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    let stats = app.stats()?;
    assert_eq!(
        stats,
        StatsResponse {
            total_question_count: 0
        }
    );

    Ok(())
}

#[test]
fn failed_ask() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;
    let abs = env.abs;

    let mock_env = abs.environment();

    let ask_cost_resp = app.ask_cost()?;

    let asker_addr = &mock_env.addr_make(USER1);

    mock_env.set_balance(asker_addr, coins(1, ask_cost_resp.fee_denom.clone()))?;

    let err: QAAppError = app
        .call_as(asker_addr)
        .ask(
            "this is a question".to_string(),
            &coins(1, ask_cost_resp.fee_denom),
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        err,
        QAAppError::InsufficientFunds {
            required: ask_cost_resp.cost,
            paid: Uint128::one(),
        }
    );

    Ok(())
}

#[test]
fn successful_ask() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;
    let abs = env.abs;

    let mock_env = abs.environment();

    let ask_cost_resp = app.ask_cost()?;

    let asker_addr = &mock_env.addr_make(USER1);

    mock_env.set_balance(
        asker_addr,
        coins(ask_cost_resp.cost.u128(), ask_cost_resp.fee_denom.clone()),
    )?;

    app.call_as(asker_addr).ask(
        "this is a question".to_string(),
        &coins(ask_cost_resp.cost.u128(), ask_cost_resp.fee_denom.clone()),
    )?;

    assert_eq!(
        mock_env.query_balance(asker_addr, ask_cost_resp.fee_denom.as_str())?,
        Uint128::zero()
    );
    assert_eq!(ask_cost_resp.ask_fee_collector, app.account().owner()?);
    assert_eq!(
        mock_env.query_balance(
            &ask_cost_resp.ask_fee_collector,
            ask_cost_resp.fee_denom.as_str()
        )?,
        ask_cost_resp.cost
    );

    let stats = app.stats()?;
    assert_eq!(
        stats,
        StatsResponse {
            total_question_count: 1
        }
    );

    let unanswered_questions = app.unanswered_questions(None, None)?;
    assert_eq!(
        unanswered_questions,
        QuestionsResponse {
            question_ids: vec![0]
        }
    );

    let question = app.question(unanswered_questions.question_ids[0])?;
    assert_eq!(
        question,
        QuestionResponse {
            question: Question {
                id: 0,
                asker: asker_addr.clone(),
                question_content: "this is a question".to_string(),
                answered: false,
                answer_content: None
            }
        }
    );

    Ok(())
}

#[test]
fn failed_answer_not_answerer() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;
    let abs = env.abs;

    let mock_env = abs.environment();

    let answerer_addr = &mock_env.addr_make(USER1);

    let err: QAAppError = app
        .call_as(answerer_addr)
        .answer("this is an answer".to_string(), 0)
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, QAAppError::OnlyAccountOwnerCanAnswer {});

    Ok(())
}

#[test]
fn failed_answer_question_not_exist() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    let answerer_addr = &app.account().owner()?;

    let err: QAAppError = app
        .call_as(answerer_addr)
        .answer("this is an answer".to_string(), 0)
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        err,
        QAAppError::QuestionNotFoundInUnansweredQuestions { question_id: 0 }
    );

    Ok(())
}

#[test]
fn successful_answer() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;
    let abs = env.abs;

    let mock_env = abs.environment();

    let ask_cost_resp = app.ask_cost()?;

    let asker_addr = &mock_env.addr_make(USER1);
    let answerer_addr = &app.account().owner()?;

    mock_env.set_balance(
        asker_addr,
        coins(ask_cost_resp.cost.u128(), ask_cost_resp.fee_denom.clone()),
    )?;

    app.call_as(asker_addr).ask(
        "this is a question".to_string(),
        &coins(ask_cost_resp.cost.u128(), ask_cost_resp.fee_denom.clone()),
    )?;

    app.call_as(answerer_addr)
        .answer("this is an answer".to_string(), 0)?;

    let unanswered_questions = app.unanswered_questions(None, None)?;
    assert_eq!(
        unanswered_questions,
        QuestionsResponse {
            question_ids: vec![]
        }
    );

    let answered_questions = app.answered_questions(None, None)?;
    assert_eq!(
        answered_questions,
        QuestionsResponse {
            question_ids: vec![0]
        }
    );

    let question = app.question(answered_questions.question_ids[0])?;
    assert_eq!(
        question,
        QuestionResponse {
            question: Question {
                id: 0,
                asker: asker_addr.clone(),
                question_content: "this is a question".to_string(),
                answered: true,
                answer_content: Some("this is an answer".to_string())
            }
        }
    );

    Ok(())
}
