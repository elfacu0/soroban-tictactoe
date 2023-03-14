#![no_std]
use soroban_sdk::{contractimpl, contracttype, log, Address, Env};

#[contracttype]
pub enum DataKey {
    PlayerA,
    PlayerB,
    PlayerTurn,
    Grid,
    Winner,
    Time
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
        assert!(!has_ended(&env), "Game has ended");
        assert!(pos_x <= 2, "X position out of range");
        assert!(pos_y <= 2, "Y position out of range");
        assert!(is_empty_cell(&env, pos_x, pos_y), "Cell is already used");

        mark_cell(&env, pos_x, pos_y);
        check_winner(&env);

        change_turn(&env);
        increase_time(&env);
    }

    pub fn player_turn(env: Env) -> Address {
        get_player_turn(&env)
    }

    pub fn player_a(env: Env) -> Address {
        get_player_a(&env)
    }

    pub fn player_b(env: Env) -> Address {
        get_player_b(&env)
    }

    pub fn winner(env: Env) -> Address {
        get_winner(&env)
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

fn get_player_a(env: &Env) -> Address {
    env.storage().get(&DataKey::PlayerA).unwrap().unwrap()
}

fn get_player_b(env: &Env) -> Address {
    env.storage().get(&DataKey::PlayerB).unwrap().unwrap()
}

fn get_player_turn(env: &Env) -> Address {
    env.storage().get(&DataKey::PlayerTurn).unwrap().unwrap()
}

fn change_turn(env: &Env) {
    if get_player_turn(env) == get_player_a(env) {
        env.storage()
            .set(&DataKey::PlayerTurn, &(get_player_b(env)));
    } else {
        env.storage()
            .set(&DataKey::PlayerTurn, &(get_player_a(env)));
    }
}

fn get_grid(env: &Env) -> u32 {
    env.storage().get(&DataKey::Grid).unwrap_or(Ok(0)).unwrap()
}

fn set_grid(env: &Env, grid: u32) {
    env.storage().set(&DataKey::Grid, &grid)
}

// 00 00 00 00 00 00 00 || 00 00 00 00 00 00 00 00 00 => x-y
// xx xx xx xx xx xx xx || 22 12 02 21 11 10 20 10 00
fn get_cell_pos(pos_x: u32, pos_y: u32) -> (u32, u32) {
    let offset = (pos_y * 3 + pos_x) << 1;
    let mask = 0b11 << offset;
    (offset, mask)
}

fn is_empty_cell(env: &Env, pos_x: u32, pos_y: u32) -> bool {
    let grid = get_grid(env);
    let (offset, mask) = get_cell_pos(pos_x, pos_y);
    let val = (grid & mask) >> offset;
    val == 0
}

fn mark_cell(env: &Env, pos_x: u32, pos_y: u32) {
    let mut grid = get_grid(env);
    let (offset, _) = get_cell_pos(pos_x, pos_y);

    if get_player_turn(env) == get_player_a(env) {
        grid |= 0b01 << offset;
    } else {
        grid |= 0b10 << offset;
    }

    set_grid(env, grid);
}

fn get_time(env: &Env) -> u32{
    env.storage().get(&DataKey::Time).unwrap_or(Ok(0)).unwrap()
}

fn increase_time(env: &Env){
    env.storage().set(&DataKey::Time, &(get_time(env)+1))
}

fn has_ended(env: &Env) -> bool {
    env.storage().has(&DataKey::Winner) || get_time(env) >= 9
}

fn get_winner(env: &Env) -> Address {
    env.storage().get(&DataKey::Winner).unwrap().unwrap()
}

fn set_winner(env: &Env, winner: Address) {
    env.storage().set(&DataKey::Winner, &winner)
}

fn check_winner(env: &Env) {
    let grid = get_grid(env);
    if grid & 0b010101 == 0b010101
        || grid & 0b010101000000 == 0b010101000000
        || grid & 0b010101000000000000 == 0b010101000000000000
        || grid & 0b010000010000010000 == 0b010000010000010000
        || grid & 0b000100000100000100 == 0b000100000100000100
        || grid & 0b000001000001000001 == 0b000001000001000001
        || grid & 0b010000000100000001 == 0b010000000100000001
        || grid & 0b000001000100010000 == 0b000001000100010000
    {
        set_winner(env, get_player_a(env));
    }

    if grid & 0b101010 == 0b101010
        || grid & 0b101010000000 == 0b101010000000
        || grid & 0b101010000000000000 == 0b101010000000000000
        || grid & 0b100000100000100000 == 0b100000100000100000
        || grid & 0b001000001000001000 == 0b001000001000001000
        || grid & 0b000010000010000010 == 0b000010000010000010
        || grid & 0b100000001000000010 == 0b100000001000000010
        || grid & 0b000010001000100000 == 0b000010001000100000
    {
        set_winner(env, get_player_b(env));
    }
}

mod test;
