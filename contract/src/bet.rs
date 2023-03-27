use crate::game::{get_player_a, get_player_b, get_winner, has_ended, has_winner};
use crate::storage::DataKey;
use core::cmp::{max, min};
use soroban_sdk::{contracttype, Address, BytesN, Env};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub struct Bet {
    pub token: BytesN<32>,
    pub amount: i128,
    pub paid: bool,
}

fn has_bet(env: &Env, player: Address) -> bool {
    if player == get_player_a(env) {
        return env.storage().has(&DataKey::BetPlayerA);
    } else {
        return env.storage().has(&DataKey::BetPlayerB);
    }
}

fn add_bet(env: &Env, player: Address, amount: i128) -> Bet {
    let mut bet = get_bet(env, player.clone());
    bet.amount += amount;
    set_bet(env, player.clone(), bet)
}

fn get_bet(env: &Env, player: Address) -> Bet {
    let default_bet = Bet {
        token: BytesN::from_array(&env, &[0; 32]),
        amount: 0,
        paid: false,
    };
    if player == get_player_a(env) {
        return env
            .storage()
            .get(&DataKey::BetPlayerA)
            .unwrap_or(Ok(default_bet))
            .unwrap();
    } else {
        return env
            .storage()
            .get(&DataKey::BetPlayerB)
            .unwrap_or(Ok(default_bet))
            .unwrap();
    }
}

fn set_bet(env: &Env, player: Address, bet: Bet) -> Bet {
    if player == get_player_a(env) {
        env.storage().set(&DataKey::BetPlayerA, &bet);
    } else {
        env.storage().set(&DataKey::BetPlayerB, &bet);
    }
    bet
}

pub fn make(env: &Env, player: Address, token: BytesN<32>, amount: i128) -> Bet {
    if player != get_player_a(env) && player != get_player_b(env) {
        panic!("You are not allowed to make a bet");
    }
    player.require_auth();

    token::Client::new(&env, &token).xfer(&player, &env.current_contract_address(), &amount);
    let mut bet = Bet {
        token,
        amount,
        paid: false,
    };

    if !has_bet(env, player.clone()) {
        bet = set_bet(env, player, bet);
    } else {
        bet = add_bet(env, player, amount)
    }

    bet
}

pub fn collect(env: &Env, player: Address) {
    player.require_auth();
    assert!(has_bet(env, player.clone()), "You don't have a bet");
    assert!(has_ended(env), "Game is still being played");

    let mut bet = get_bet(env, player.clone());
    assert!(bet.paid == false, "You have already been paid");

    let player_a_bet = get_bet(env, get_player_a(env));
    let player_b_bet = get_bet(env, get_player_b(env));
    let amount = min(player_a_bet.amount, player_b_bet.amount);

    if player == get_player_a(env) {
        let returned_amount = max(0, player_a_bet.amount - player_b_bet.amount);
        pay(env, &player, player_a_bet.token, returned_amount);

        if has_winner(env) && get_winner(env) == player {
            let diff = player_a_bet.amount - returned_amount;
            pay(env, &player, player_b_bet.token, amount + diff);
        }
    } else {
        let returned_amount = max(0, player_b_bet.amount - player_a_bet.amount);
        pay(env, &player, player_b_bet.token, returned_amount);

        if has_winner(env) && get_winner(env) == player {
            let diff = player_b_bet.amount - returned_amount;
            pay(env, &player, player_a_bet.token, amount + diff);
        }
    }

    bet.paid = true;
    set_bet(env, player, bet);
}

fn pay(env: &Env, to: &Address, token: BytesN<32>, amount: i128) {
    if amount <= 0 {
        return;
    }
    token::Client::new(&env, &token).xfer(&env.current_contract_address(), &to, &amount);
}
