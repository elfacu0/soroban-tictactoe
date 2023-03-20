#![cfg(test)]

use crate::{Deployer, DeployerClient};
use soroban_sdk::{testutils::Address as _, Address, Bytes, Env, IntoVal};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/tictactoe_game.wasm"
    );
}

#[test]
fn test_deploy() {
    let env = Env::default();
    let client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

    let wasm_hash = env.install_contract_wasm(contract::WASM);

    let salt = Bytes::from_array(&env, &[0; 32]);
    let player_a = Address::random(&env);
    let player_b = Address::random(&env);
    let init_fn_args = (player_a.clone(), player_b.clone()).into_val(&env);
    let contract_id = client.deploy(&salt, &wasm_hash, &init_fn_args);

    let client = contract::Client::new(&env, &contract_id);
    assert_eq!(client.player_a(), player_a);
    assert_eq!(client.player_b(), player_b);
}

#[test]
fn test_get_game() {
    let env = Env::default();
    let client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

    let wasm_hash = env.install_contract_wasm(contract::WASM);

    let salt = Bytes::from_array(&env, &[0; 32]);
    let player_a = Address::random(&env);
    let player_b = Address::random(&env);
    let init_fn_args = (player_a.clone(), player_b.clone()).into_val(&env);
    let contract_id = client.deploy(&salt, &wasm_hash, &init_fn_args);
    
    let game = crate::Game { player_a, player_b, ended: false };

    assert_eq!(client.game(&contract_id), game);
    assert_eq!(client.game(&contract_id), game);
}

#[test]
fn test_set_ended() {
    let env = Env::default();
    let client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

    let wasm_hash = env.install_contract_wasm(contract::WASM);

    let salt = Bytes::from_array(&env, &[0; 32]);
    let player_a = Address::random(&env);
    let player_b = Address::random(&env);
    let init_fn_args = (player_a.clone(), player_b.clone()).into_val(&env);
    let contract_id = client.deploy(&salt, &wasm_hash, &init_fn_args);
    
    let mut game = crate::Game { player_a: player_a.clone(), player_b: player_b.clone(), ended: false };

    assert_eq!(client.game(&contract_id), game);

    let game_client = contract::Client::new(&env, &contract_id);
    game_client.play(&player_a, &0, &0);
    game_client.play(&player_b, &0, &1);
    game_client.play(&player_a, &1, &0);
    game_client.play(&player_b, &1, &1);
    game_client.play(&player_a, &2, &0);

    game.ended = true;
    assert_eq!(client.game(&contract_id), game);
    
}
