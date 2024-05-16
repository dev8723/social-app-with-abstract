use friend_tech_app::{
    contract::interface::FriendTechAppInterface,
    msg::{
        FriendTechAppExecuteMsgFns, FriendTechAppInstantiateMsg, FriendTechAppQueryMsgFns,
        HoldersResponse, IssuerResponse,
    },
    FriendTechAppError, MY_NAMESPACE,
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
    app: Application<Env, FriendTechAppInterface<Env>>,
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
        let publisher = abs_client.publisher_builder(namespace).build()?;
        publisher.publish_app::<FriendTechAppInterface<_>>()?;

        let app = publisher
            .account()
            .install_app::<FriendTechAppInterface<_>>(
                &FriendTechAppInstantiateMsg {
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

    let issuer = app.issuer()?;
    assert_eq!(
        issuer,
        IssuerResponse {
            username: "test".to_string(),
            fee_denom: DENOM.to_string(),
            issuer_fee_collector: env.abs.sender(),
            supply: Uint128::one(),
        }
    );
    let holders = app.holders(None, None)?;
    assert_eq!(
        holders,
        HoldersResponse {
            holders: vec![env.abs.sender()]
        }
    );
    let holding = app.holding(env.abs.sender().to_string())?;
    assert_eq!(holding.amount, Uint128::one());

    Ok(())
}

#[test]
fn failed_buy_key() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;
    let abs = env.abs;

    let issuer = app.issuer()?;
    let fee_denom = issuer.fee_denom.as_str();

    let buy_amount = 10u128;

    let mock_env = abs.environment();

    let simulation_buy_resp = app.simulate_buy_key(Uint128::from(buy_amount))?;

    let buyer_addr = &mock_env.addr_make(USER1);

    mock_env.set_balance(buyer_addr, coins(1, fee_denom))?;

    let err: FriendTechAppError = app
        .call_as(buyer_addr)
        .buy_key(Uint128::from(buy_amount), &coins(1, fee_denom))
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        err,
        FriendTechAppError::InsufficientFunds {
            required: simulation_buy_resp.total_cost,
            paid: Uint128::one(),
        }
    );

    Ok(())
}

#[test]
fn successful_buy_key() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;
    let abs = env.abs;

    let issuer = app.issuer()?;
    let fee_denom = issuer.fee_denom.as_str();

    let buy_amount = 10u128;

    let mock_env = abs.environment();

    let simulation_buy_resp = app.simulate_buy_key(Uint128::from(buy_amount))?;

    let buyer_addr = &mock_env.addr_make(USER1);

    mock_env.set_balance(
        buyer_addr,
        coins(simulation_buy_resp.total_cost.u128(), fee_denom),
    )?;

    app.call_as(buyer_addr).buy_key(
        Uint128::from(buy_amount),
        &coins(simulation_buy_resp.total_cost.u128(), fee_denom),
    )?;

    assert_eq!(
        mock_env.query_balance(buyer_addr, fee_denom)?,
        Uint128::zero()
    );
    assert_eq!(
        mock_env.query_balance(&issuer.issuer_fee_collector, fee_denom)?,
        simulation_buy_resp.issuer_fee
    );
    assert_eq!(
        mock_env.query_balance(&app.address()?, fee_denom)?,
        simulation_buy_resp.price
    );

    let issuer = app.issuer()?;
    assert_eq!(issuer.supply, Uint128::from(buy_amount) + Uint128::one());

    let holders = app.holders(None, None)?;
    assert_eq!(
        holders,
        HoldersResponse {
            holders: vec![abs.sender(), buyer_addr.clone()]
        }
    );

    let holding = app.holding(buyer_addr.to_string())?;
    assert_eq!(holding.amount, Uint128::from(buy_amount));

    Ok(())
}

#[test]
fn successful_sell_key() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;
    let abs = env.abs;

    let issuer = app.issuer()?;
    let fee_denom = issuer.fee_denom.as_str();

    let buy_amount = 10u128;
    let sell_amount = 5u128;

    let mock_env = abs.environment();

    let simulation_buy_resp = app.simulate_buy_key(Uint128::from(buy_amount))?;

    let trader_addr = &mock_env.addr_make(USER1);

    mock_env.set_balance(
        trader_addr,
        coins(simulation_buy_resp.total_cost.u128(), fee_denom),
    )?;

    app.call_as(trader_addr).buy_key(
        Uint128::from(buy_amount),
        &coins(simulation_buy_resp.total_cost.u128(), fee_denom),
    )?;

    let simulation_sell_resp = app.simulate_sell_key(Uint128::from(sell_amount))?;
    mock_env.set_balance(
        trader_addr,
        coins(simulation_sell_resp.total_cost.u128(), fee_denom),
    )?;

    app.call_as(trader_addr).sell_key(
        Uint128::from(sell_amount),
        &coins(simulation_sell_resp.total_cost.u128(), fee_denom),
    )?;

    assert_eq!(
        mock_env.query_balance(trader_addr, fee_denom)?,
        simulation_sell_resp.price
    );
    assert_eq!(
        mock_env.query_balance(&issuer.issuer_fee_collector, fee_denom)?,
        simulation_buy_resp.issuer_fee + simulation_sell_resp.issuer_fee
    );
    assert_eq!(
        mock_env.query_balance(&app.address()?, fee_denom)?,
        simulation_buy_resp.price - simulation_sell_resp.price
    );

    let issuer = app.issuer()?;
    assert_eq!(issuer.supply, Uint128::from(buy_amount - sell_amount + 1));

    let holders = app.holders(None, None)?;
    assert_eq!(
        holders,
        HoldersResponse {
            holders: vec![abs.sender(), trader_addr.clone()]
        }
    );

    let holding = app.holding(trader_addr.to_string())?;
    assert_eq!(holding.amount, Uint128::from(buy_amount - sell_amount));

    Ok(())
}
