#![cfg(test)]

use super::{GameContract, GameContractClient};
use soroban_sdk::{testutils::Address as _, testutils::Logger, Address, Env};

extern crate std;

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);

    assert_eq!(client.player_a(), player_a);
    assert_eq!(client.player_b(), player_b);

    assert_eq!(client.player_turn(), player_a);
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

#[test]
fn test_play() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&pos_x, &pos_y);
    assert_eq!(client.player_turn(), player_b);
    
    client.play(&(pos_x-1), &(pos_y-1));
    assert_eq!(client.player_turn(), player_a);
}

#[test]
fn test_mark_empty_cell() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&pos_x, &pos_y);
}

#[test]
#[should_panic]
fn test_mark_used_cell() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&pos_x, &pos_y);
    client.play(&pos_x, &pos_y);
}

#[test]
fn test_winner_a() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);

    client.play(&0, &0); //player_a
    client.play(&0, &1); //player_b
    client.play(&1, &0); //player_a
    client.play(&1, &1); 
    client.play(&2, &0); //player_a

    assert_eq!(client.winner(),player_a);
}

#[test]
fn test_winner_b() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);

    client.play(&2, &0); //player_a
    client.play(&0, &0); //player_b
    client.play(&1, &0); 
    client.play(&0, &1); 
    client.play(&1, &1);
    client.play(&0, &2); 

    assert_eq!(client.winner(),player_b);
}

#[test]
#[should_panic]
fn test_game_over() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);

    client.play(&0, &0); //player_a
    client.play(&0, &1); //player_b
    client.play(&1, &0); //player_a
    client.play(&1, &1); 
    client.play(&2, &0); //player_a  already won
    client.play(&1, &2); 
}

#[test]
#[should_panic]
fn test_draw() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.initialize(&player_a, &player_b);

    client.play(&0, &0);
    client.play(&1, &0);
    client.play(&2, &0);

    client.play(&2, &1);
    client.play(&0, &1);
    client.play(&1, &1);
    
    client.play(&1, &2);
    client.play(&0, &2);
    client.play(&2, &2);

    client.winner();
}
