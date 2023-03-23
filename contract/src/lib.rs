#![no_std]
use core::cmp::{min, max};

use soroban_sdk::{contractimpl, contracttype, symbol, vec, Address, BytesN, Env, Symbol, Vec};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

#[contracttype]
pub enum DataKey {
    PlayerA,
    PlayerB,
    PlayerTurn,
    Grid,
    Winner,
    Time,
    Expiration,
}

#[contracttype]
pub enum Bets {
    PlayerA,
    PlayerB,
}

#[derive(Clone)]
#[contracttype]
pub struct Bet {
    pub token: BytesN<32>,
    pub amount: i128,
    pub paid: bool,
}

pub struct GameContract;

#[contractimpl]
impl GameContract {
    pub fn init(env: Env, player_a: Address, player_b: Address, expiration: u64) {
        assert!(!has_players(&env), "Already initialized");
        set_players(&env, &player_a, &player_b);
        set_expiration(&env, expiration);
    }

    pub fn play(env: Env, player: Address, pos_x: u32, pos_y: u32) -> Vec<Symbol> {
        assert!(has_players(&env), "Game is not initialized");
        assert!(allowed_player(&env, player), "It's not your turn");
        assert!(!has_ended(&env), "Game has ended");
        assert!(pos_x <= 2, "X position out of range");
        assert!(pos_y <= 2, "Y position out of range");
        assert!(is_empty_cell(&env, pos_x, pos_y), "Cell is already used");

        mark_cell(&env, pos_x, pos_y);
        check_winner(&env);

        change_turn(&env);
        increase_time(&env);

        Self::grid(env)
    }

    pub fn turn(env: Env) -> Address {
        get_player_turn(&env)
    }

    pub fn player_a(env: Env) -> Address {
        get_player_a(&env)
    }

    pub fn player_b(env: Env) -> Address {
        get_player_b(&env)
    }

    pub fn winner(env: Env) -> Address {
        assert!(has_ended(&env), "Game is still being played");
        get_winner(&env)
    }

    pub fn ended(env: Env) -> bool {
        has_ended(&env)
    }

    pub fn grid(env: Env) -> Vec<Symbol> {
        let empty = symbol!("");
        let x = symbol!("X");
        let o = symbol!("O");
        let mut res = vec![&env];
        let mut pointer = 0b110000000000000000;
        let curr_grid = get_grid(&env);
        let mut offset = 16;
        for _ in 0..9 {
            match (curr_grid & pointer) >> offset {
                1 => res.push_back(x.clone()),
                2 => res.push_back(o.clone()),
                _ => res.push_back(empty.clone()),
            }
            pointer >>= 2;
            offset -= 2;
        }
        res
    }

    pub fn bet(env: Env, player: Address, token: BytesN<32>, amount: i128) {
        make_bet(&env, player, token, amount);
    }

    pub fn clct_bet(env: Env, player: Address) {
        collect_bet(&env, player);
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
// | 2-2 | 2-1 | 2-0 |
// | 1-2 | 1-1 | 1-0 |
// | 0-2 | 0-1 | 0-0 |
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

fn get_time(env: &Env) -> u32 {
    env.storage().get(&DataKey::Time).unwrap_or(Ok(0)).unwrap()
}

fn increase_time(env: &Env) {
    env.storage().set(&DataKey::Time, &(get_time(env) + 1))
}

fn has_ended(env: &Env) -> bool {
    env.storage().has(&DataKey::Winner) || get_time(env) >= 9 || is_expired(env)
}

fn has_winner(env: &Env) -> bool {
    env.storage().has(&DataKey::Winner)
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

fn allowed_player(env: &Env, player: Address) -> bool {
    player.require_auth();
    get_player_turn(env) == player
}

fn get_expiration(env: &Env) -> u64 {
    env.storage()
        .get(&DataKey::Expiration)
        .unwrap_or(Ok(0))
        .unwrap()
}

fn set_expiration(env: &Env, expiration: u64) {
    env.storage().set(&DataKey::Expiration, &expiration);
}

fn is_expired(env: &Env) -> bool {
    let ledger_timestamp = env.ledger().timestamp();
    let exp_timestamp = get_expiration(env);
    ledger_timestamp >= exp_timestamp
}

fn has_bet(env: &Env, player: Address) -> bool {
    if player == get_player_a(env) {
        return env.storage().has(&Bets::PlayerA);
    } else {
        return env.storage().has(&Bets::PlayerB);
    }
}

fn add_bet(env: &Env, player: Address, amount: i128) -> Bet {
    let bet = get_bet(env, player.clone());
    bet.amount += amount;
    set_bet(env, player.clone(), bet);
    bet
}

fn get_bet(env: &Env, player: Address) -> Bet {
    if player == get_player_a(env) {
        return env.storage().get(&Bets::PlayerA).unwrap().unwrap();
    } else {
        return env.storage().get(&Bets::PlayerB).unwrap().unwrap();
    }
}

fn set_bet(env: &Env, player: Address, bet: Bet) {
    if player == get_player_a(env) {
        env.storage().set(&Bets::PlayerA, &bet);
    } else {
        env.storage().set(&Bets::PlayerB, &bet);
    }
}

fn make_bet(env: &Env, player: Address, token: BytesN<32>, amount: i128) -> Bet {
    if player != get_player_a(env) && player != get_player_b(env) {
        panic!("You are not allowed to make a bet");
    }
    player.require_auth();

    token::Client::new(&env, &token).xfer(&player, &env.current_contract_address(), &amount);

    let bet;
    if !has_bet(env, player) {
        bet = Bet { token, amount, paid: false };
        set_bet(env, player, bet);
    } else {
        bet = add_bet(env, player, amount)
    }

    bet
}

fn collect_bet(env: &Env,player: Address){
    player.require_auth();
    assert!(!has_bet(env, player),"You don't have a bet");

    let mut bet = get_bet(env, player);
    assert!(bet.paid,"You have already been paid");

    let player_a_bet = get_bet(env, get_player_a(env));
    let player_b_bet = get_bet(env, get_player_b(env));
    let amount = min(player_a_bet.amount, player_b_bet.amount);

    if player == get_player_a(env){
        let returned_ammount = max(0,player_a_bet.amount- player_b_bet.amount);
        pay(env, &player, player_a_bet.token, returned_ammount);

        if has_winner(env) && get_winner(env) == player{
            pay(env, &player, player_b_bet.token, player_b_bet.amount);
        }
    }else{
        let returned_ammount = max(0,player_b_bet.amount- player_a_bet.amount);
        pay(env, &player, player_b_bet.token, returned_ammount);

        if has_winner(env) && get_winner(env) == player{
            pay(env, &player, player_a_bet.token, player_a_bet.amount);
        }
    }

    bet.paid = true;
    set_bet(env, player, bet);
}

fn pay(env: &Env, to: &Address, token: BytesN<32>, ammount: i128) {
    if ammount <= 0{ return }
    if has_winner(env) {
        let player = get_winner(env);
        
        token::Client::new(&env, &bet.token).xfer(
            &env.current_contract_address(),
            &player,
            &amount,
        );
    }
}

mod test;
