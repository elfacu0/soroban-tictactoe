#![cfg(test)]

use crate::{Deployer, DeployerClient};
use soroban_sdk::{testutils::Address as _, vec, Address, Bytes, BytesN, Env, IntoVal, Vec};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/tictactoe_game.wasm"
    );
}

struct GameTest {
    env: Env,
    deployer_client: DeployerClient,
    player_a: Address,
    player_b: Address,
    contract_id: BytesN<32>,
    game_client: contract::Client,
}

impl GameTest {
    fn setup() -> Self {
        let env = Env::default();
        let deployer_client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

        let wasm_hash = env.install_contract_wasm(contract::WASM);

        let salt = Bytes::from_array(&env, &[0; 32]);
        let player_a = Address::random(&env);
        let player_b = Address::random(&env);
        let init_fn_args = (player_a.clone(), player_b.clone()).into_val(&env);
        let contract_id = deployer_client.deploy(&salt, &wasm_hash, &init_fn_args);

        let game_client = contract::Client::new(&env, &contract_id);

        GameTest {
            env,
            deployer_client,
            player_a,
            player_b,
            contract_id,
            game_client,
        }
    }
}

#[test]
fn test_deploy() {
    let GameTest {
        env: _,
        deployer_client: _,
        player_a,
        player_b,
        contract_id: _,
        game_client,
    } = GameTest::setup();

    assert_eq!(game_client.player_a(), player_a);
    assert_eq!(game_client.player_b(), player_b);
}

#[test]
fn test_get_game() {
    let GameTest {
        env: _,
        deployer_client,
        player_a,
        player_b,
        contract_id,
        game_client: _,
    } = GameTest::setup();

    let game = crate::Game {
        player_a,
        player_b,
        ended: false,
    };

    assert_eq!(deployer_client.game(&contract_id), game);
    assert_eq!(deployer_client.game(&contract_id), game);
}

#[test]
fn test_set_ended() {
    let GameTest {
        env,
        deployer_client,
        player_a,
        player_b,
        contract_id,
        game_client: _,
    } = GameTest::setup();

    let mut game = crate::Game {
        player_a: player_a.clone(),
        player_b: player_b.clone(),
        ended: false,
    };

    assert_eq!(deployer_client.game(&contract_id), game);

    let game_client = contract::Client::new(&env, &contract_id);
    game_client.play(&player_a, &0, &0);
    game_client.play(&player_b, &0, &1);
    game_client.play(&player_a, &1, &0);
    game_client.play(&player_b, &1, &1);
    game_client.play(&player_a, &2, &0);

    game.ended = true;
    assert_eq!(deployer_client.game(&contract_id), game);
}

#[test]
fn test_scores() {
    let GameTest {
        env,
        deployer_client,
        player_a: _,
        player_b: _,
        contract_id: _,
        game_client: _,
    } = GameTest::setup();

    let exp = vec![&env];
    assert_eq!(deployer_client.scores(), exp);
}

#[test]
fn test_scores_add_win() {
    let GameTest {
        env,
        deployer_client,
        player_a,
        player_b,
        contract_id,
        game_client: _,
    } = GameTest::setup();

    let mut game = crate::Game {
        player_a: player_a.clone(),
        player_b: player_b.clone(),
        ended: false,
    };

    assert_eq!(deployer_client.game(&contract_id), game);

    let game_client = contract::Client::new(&env, &contract_id);
    game_client.play(&player_a, &0, &0);
    game_client.play(&player_b, &0, &1);
    game_client.play(&player_a, &1, &0);
    game_client.play(&player_b, &1, &1);
    game_client.play(&player_a, &2, &0);

    game.ended = true;
    assert_eq!(deployer_client.game(&contract_id), game);

    let exp = Vec::from_array(&env, [(player_a, 1)]);
    assert_eq!(deployer_client.scores(), exp);
}
