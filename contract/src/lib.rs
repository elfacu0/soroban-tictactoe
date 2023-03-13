#![no_std]
use soroban_sdk::{contractimpl, log, Env, Symbol, Address, contracttype};

#[contracttype]
pub enum Players {
    A,
    B,
}

pub struct GameContract;

#[contractimpl]
impl GameContract {
    pub fn initialize(env: Env, player_a: Address, player_b: Address) {
        assert!(!has_players(&env), "already initialized");
        set_players(&env, &player_a, &player_b);
    }
}

fn has_players(env: &Env) -> bool{
    env.storage().has(&Players::A) && env.storage().has(&Players::B)
}   

fn set_players(env: &Env, player_a: &Address, player_b: &Address) {
    env.storage().set(&Players::A, player_a);
    env.storage().set(&Players::B, player_b);
}

mod test;