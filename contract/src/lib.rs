#![no_std]
use crate::bet::Bet;
use soroban_sdk::{contractimpl, Address, BytesN, Env, Symbol, Vec};

mod bet;
mod game;
mod storage;

pub struct GameContract;

#[contractimpl]
impl GameContract {
    pub fn init(env: Env, player_a: Address, player_b: Address, expiration: u64) {
        game::init(env, player_a, player_b, expiration);
    }

    pub fn play(env: Env, player: Address, pos_x: u32, pos_y: u32) -> Vec<Symbol> {
        game::play(env, player, pos_x, pos_y)
    }

    pub fn turn(env: Env) -> Address {
        game::get_player_turn(&env)
    }

    pub fn player_a(env: Env) -> Address {
        game::get_player_a(&env)
    }

    pub fn player_b(env: Env) -> Address {
        game::get_player_b(&env)
    }

    pub fn winner(env: Env) -> Address {
        game::winner(env)
    }

    pub fn ended(env: Env) -> bool {
        game::has_ended(&env)
    }

    pub fn grid(env: Env) -> Vec<Symbol> {
        game::grid(env)
    }

    pub fn bet(env: Env, player: Address, token: BytesN<32>, amount: i128) -> Bet {
        bet::make(&env, player, token, amount)
    }

    pub fn clct_bet(env: Env, player: Address) {
        bet::collect(&env, player);
    }
}

mod test;
