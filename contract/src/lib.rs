#![no_std]
use soroban_sdk::{contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    PlayerA,
    PlayerB,
    PlayerTurn,
    Grid
}

pub struct GameContract;

#[contractimpl]
impl GameContract {
    pub fn initialize(env: Env, player_a: Address, player_b: Address) {
        assert!(!has_players(&env), "Already initialized");
        set_players(&env, &player_a, &player_b);
    }

    pub fn play(env: Env, pos_x: u32, pos_y: u32) {
        assert!(has_players(&env), "Game is not initialized");
        assert!(pos_x <= 2, "X position out of range");
        assert!(pos_y <= 2, "Y position out of range");
        change_turn(&env);
    }

    pub fn player_turn(env: Env) -> Address{
        get_player_turn(&env)
    }

    pub fn player_a(env: Env) -> Address{
        get_player_a(&env)
    }

    pub fn player_b(env: Env) -> Address{
        get_player_b(&env)
    }
}

fn has_players(env: &Env) -> bool {
    env.storage().has(&DataKey::PlayerA) && env.storage().has(&DataKey::PlayerB)
}

fn set_players(env: &Env, player_a: &Address, player_b: &Address) {
    env.storage().set(&DataKey::PlayerA, player_a);
    env.storage().set(&DataKey::PlayerB, player_b);

    env.storage().set(&DataKey::PlayerTurn, player_a);
}

fn get_player_a(env: &Env) -> Address{
    env.storage().get(&DataKey::PlayerA).unwrap().unwrap()
}

fn get_player_b(env: &Env) -> Address{
    env.storage().get(&DataKey::PlayerB).unwrap().unwrap()
}

fn get_player_turn(env: &Env) -> Address{
    env.storage().get(&DataKey::PlayerTurn).unwrap().unwrap()
}

fn change_turn(env: &Env){
    if get_player_turn(env) == get_player_a(env){
        env.storage().set(&DataKey::PlayerTurn, &(get_player_b(env)));
    }else{
        env.storage().set(&DataKey::PlayerTurn, &(get_player_a(env)));
    }
}

mod test;
