#![cfg(test)]

use crate::{Deployer, DeployerClient};
use soroban_sdk::{map, testutils::Address as _, Address, BytesN, Env, IntoVal};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(file = "../game/tictactoe_game.wasm");
}

struct GameTest<'a> {
    env: Env,
    deployer_client: DeployerClient<'a>,
    player_a: Address,
    player_b: Address,
    contract_id: Address,
    game_client: contract::Client<'a>,
}

impl GameTest<'_> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let deployer_client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

        let wasm_hash = env.deployer().upload_contract_wasm(contract::WASM);

        let salt = BytesN::from_array(&env, &[0; 32]);
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

    fn deploy_new_game(&self, salt: BytesN<32>) -> contract::Client {
        let wasm_hash = self.env.deployer().upload_contract_wasm(contract::WASM);

        let init_fn_args = (self.player_a.clone(), self.player_b.clone()).into_val(&self.env);
        let contract_id = self
            .deployer_client
            .deploy(&salt, &wasm_hash, &init_fn_args);

        contract::Client::new(&self.env, &contract_id)
    }

    fn make_player_a_win(client: &contract::Client, player_a: Address, player_b: Address) {
        client.play(&player_a, &0, &0);
        client.play(&player_b, &0, &1);
        client.play(&player_a, &1, &0);
        client.play(&player_b, &1, &1);
        client.play(&player_a, &2, &0);
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
        game_client,
    } = GameTest::setup();

    let mut game = crate::Game {
        player_a: player_a.clone(),
        player_b: player_b.clone(),
        ended: false,
    };

    assert_eq!(deployer_client.game(&contract_id), game);
    GameTest::make_player_a_win(&game_client, player_a, player_b);
    env.budget().reset_unlimited();
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

    let exp = map![&env];
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
        game_client,
    } = GameTest::setup();

    let mut game = crate::Game {
        player_a: player_a.clone(),
        player_b: player_b.clone(),
        ended: false,
    };

    assert_eq!(deployer_client.game(&contract_id), game);

    GameTest::make_player_a_win(&game_client, player_a.clone(), player_b);

    game.ended = true;
    env.budget().reset_unlimited();
    assert_eq!(deployer_client.game(&contract_id), game);

    let exp = map![&env, (player_a, 1)];
    assert_eq!(deployer_client.scores(), exp);
}

#[test]
fn test_scores_add_wins() {
    let game_test = GameTest::setup();

    let game1 = game_test.deploy_new_game(BytesN::from_array(&game_test.env, &[1; 32]));
    GameTest::make_player_a_win(
        &game1,
        game_test.player_a.clone(),
        game_test.player_b.clone(),
    );
    game_test.env.budget().reset_unlimited();
    game_test.deployer_client.game(&game1.address);

    let game2 = game_test.deploy_new_game(BytesN::from_array(&game_test.env, &[2; 32]));
    GameTest::make_player_a_win(
        &game2,
        game_test.player_a.clone(),
        game_test.player_b.clone(),
    );
    game_test.deployer_client.game(&game2.address);

    let exp = map![&game_test.env, (game_test.player_a, 2)];
    assert_eq!(game_test.deployer_client.scores(), exp);
}

#[test]
fn test_scores_add_wins_2() {
    let game_test = GameTest::setup();

    let game1 = game_test.deploy_new_game(BytesN::from_array(&game_test.env, &[1; 32]));
    game_test.env.budget().reset_unlimited();
    GameTest::make_player_a_win(
        &game1,
        game_test.player_a.clone(),
        game_test.player_b.clone(),
    );
    game_test.deployer_client.game(&game1.address);

    let game2 = game_test.deploy_new_game(BytesN::from_array(&game_test.env, &[2; 32]));
    game2.play(&game_test.player_a, &2, &2);
    GameTest::make_player_a_win(
        &game2,
        game_test.player_b.clone(),
        game_test.player_a.clone(),
    );
    game_test.deployer_client.game(&game2.address);

    let exp = map![
        &game_test.env,
        (game_test.player_a, 1),
        (game_test.player_b, 1),
    ];
    assert_eq!(game_test.deployer_client.scores(), exp);
}
