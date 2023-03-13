#![cfg(test)]

use super::{GameContract, GameContractClient};
use soroban_sdk::{testutils::Address as _, testutils::Logger, Env, Address};

extern crate std;


#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);

    assert_eq!(client.player_a(),player_a);
    assert_eq!(client.player_b(),player_b);
}

#[test]
#[should_panic]
fn test_already_initialized() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);
    client.initialize(&player_a, &player_b);
}