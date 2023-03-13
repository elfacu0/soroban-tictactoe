#![no_std]
use soroban_sdk::{contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum Players {
    A,
    B,
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
    }

    pub fn player_a(env: Env) -> Address{
        env.storage().get(&Players::A).unwrap().unwrap()
    }

    pub fn player_b(env: Env) -> Address{
        env.storage().get(&Players::B).unwrap().unwrap()
    }
}

fn has_players(env: &Env) -> bool {
    env.storage().has(&Players::A) && env.storage().has(&Players::B)
}

fn set_players(env: &Env, player_a: &Address, player_b: &Address) {
    env.storage().set(&Players::A, player_a);
    env.storage().set(&Players::B, player_b);
}

mod test;
