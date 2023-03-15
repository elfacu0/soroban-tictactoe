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
fn test() {
    let env = Env::default();
    let client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

    let wasm_hash = env.install_contract_wasm(contract::WASM);

    let salt = Bytes::from_array(&env, &[0; 32]);
    let player_a = Address::random(&env);
    let player_b = Address::random(&env);
    let init_fn_args = (player_a.clone(), player_b.clone()).into_val(&env);
    let (contract_id, init_result) = client.deploy(&salt, &wasm_hash, &init_fn_args);
    assert!(init_result.is_void());

    let client = contract::Client::new(&env, &contract_id);
    assert_eq!(client.player_a(), player_a);
    assert_eq!(client.player_b(), player_b);
}
