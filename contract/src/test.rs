#![cfg(test)]

use super::{GameContract, GameContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.init(&player_a, &player_b);

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

    client.init(&player_a, &player_b);
    client.init(&player_a, &player_b);
}

#[test]
fn test_change_turn() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.init(&player_a, &player_b);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
    assert_eq!(client.player_turn(), player_b);

    client.play(&player_b, &(pos_x - 1), &(pos_y - 1));
    assert_eq!(client.player_turn(), player_a);
}

#[test]
#[should_panic]
fn test_other_player() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);
    let player_c = Address::random(&env);

    client.init(&player_a, &player_b);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
    client.play(&player_c, &(pos_x - 1), &(pos_y - 1));
}

#[test]
#[should_panic]
fn test_twice_play() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.init(&player_a, &player_b);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
    client.play(&player_a, &(pos_x - 1), &(pos_y - 1));
}

#[test]
fn test_mark_empty_cell() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.init(&player_a, &player_b);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
}

#[test]
#[should_panic]
fn test_mark_used_cell() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.init(&player_a, &player_b);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
    client.play(&player_b, &pos_x, &pos_y);
}

#[test]
fn test_winner_a() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.init(&player_a, &player_b);

    client.play(&player_a, &0, &0);
    client.play(&player_b, &0, &1);
    client.play(&player_a, &1, &0);
    client.play(&player_b, &1, &1);
    client.play(&player_a, &2, &0);

    assert_eq!(client.winner(), player_a);
}

#[test]
fn test_winner_b() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.init(&player_a, &player_b);

    client.play(&player_a, &2, &0);
    client.play(&player_b, &0, &0);
    client.play(&player_a, &1, &0);
    client.play(&player_b, &0, &1);
    client.play(&player_a, &1, &1);
    client.play(&player_b, &0, &2);

    assert_eq!(client.winner(), player_b);
}

#[test]
#[should_panic]
fn test_game_over() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.init(&player_a, &player_b);

    client.play(&player_a, &0, &0);
    client.play(&player_b, &0, &1);
    client.play(&player_a, &1, &0);
    client.play(&player_b, &1, &1);
    client.play(&player_a, &2, &0); //player_a  already won
    client.play(&player_b, &1, &2);
}

#[test]
#[should_panic]
fn test_draw() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GameContract);
    let client = GameContractClient::new(&env, &contract_id);

    let player_a = Address::random(&env);
    let player_b = Address::random(&env);

    client.init(&player_a, &player_b);

    client.play(&player_a, &0, &0);
    client.play(&player_b, &1, &0);
    client.play(&player_a, &2, &0);

    client.play(&player_b, &2, &1);
    client.play(&player_a, &0, &1);
    client.play(&player_b, &1, &1);

    client.play(&player_a, &1, &2);
    client.play(&player_b, &0, &2);
    client.play(&player_a, &2, &2);

    client.winner();
}
