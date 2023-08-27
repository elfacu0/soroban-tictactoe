use crate::game::{get_player_a, get_player_b, get_winner, has_ended, has_winner};
use crate::storage::DataKey;
use core::cmp::{max, min};
use soroban_sdk::{contracttype, token, vec, Address, Env, Vec};

#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub struct Bet {
    pub token: Address,
    pub amount: i128,
    pub paid: bool,
}

fn has_bet(env: &Env, player: Address) -> bool {
    match player == get_player_a(env) {
        true => env.storage().instance().has(&DataKey::BetPlayerA),
        false => env.storage().instance().has(&DataKey::BetPlayerB),
    }
}

fn add_bet(env: &Env, player: Address, amount: i128) -> Bet {
    let mut bet = get_bet(env, player.clone());
    bet.amount += amount;
    set_bet(env, player.clone(), bet)
}

fn get_bet(env: &Env, player: Address) -> Bet {
    let default_bet = Bet {
        token: Address::from(player.clone()),
        amount: 0,
        paid: false,
    };
    let player_key = match player == get_player_a(env) {
        true => &DataKey::BetPlayerA,
        false => &DataKey::BetPlayerB,
    };
    env.storage()
        .instance()
        .get(player_key)
        .unwrap_or(default_bet)
}

fn set_bet(env: &Env, player: Address, bet: Bet) -> Bet {
    match player == get_player_a(env) {
        true => env.storage().instance().set(&DataKey::BetPlayerA, &bet),
        false => env.storage().instance().set(&DataKey::BetPlayerB, &bet),
    };
    bet
}

pub fn make(env: &Env, player: Address, token: Address, amount: i128) -> Bet {
    if player != get_player_a(env) && player != get_player_b(env) {
        panic!("You are not allowed to make a bet");
    }
    player.require_auth();

    token::Client::new(&env, &token).transfer(&player, &env.current_contract_address(), &amount);
    let bet = Bet {
        token,
        amount,
        paid: false,
    };

    match has_bet(env, player.clone()) {
        true => add_bet(env, player, amount),
        false => set_bet(env, player, bet),
    }
}

pub fn collect(env: &Env, player: Address) -> Vec<Bet> {
    player.require_auth();
    assert!(has_bet(env, player.clone()), "You don't have a bet");
    assert!(has_ended(env), "Game is still being played");

    let mut bet = get_bet(env, player.clone());
    assert!(bet.paid == false, "You have already been paid");

    let mut res = vec![env];

    let player_a_bet = get_bet(env, get_player_a(env));
    let player_b_bet = get_bet(env, get_player_b(env));
    let profit = min(player_a_bet.amount, player_b_bet.amount);

    let (player_bet, opponent_bet) = match player == get_player_a(env) {
        true => (player_a_bet, player_b_bet),
        false => (player_b_bet, player_a_bet),
    };

    let returned_amount = max(0, player_bet.amount - opponent_bet.amount);
    pay(env, &player, player_bet.token.clone(), returned_amount);

    res.push_back(Bet {
        token: player_bet.token.clone(),
        amount: returned_amount,
        paid: true,
    });

    if has_winner(env) && get_winner(env) == player && profit > 0 {
        let diff = player_bet.amount - returned_amount;
        pay(env, &player, opponent_bet.token.clone(), profit + diff);

        res.push_back(Bet {
            token: opponent_bet.token.clone(),
            amount: profit + diff,
            paid: true,
        });
    }

    bet.paid = true;
    set_bet(env, player, bet);

    res
}

fn pay(env: &Env, to: &Address, token: Address, amount: i128) {
    if amount <= 0 {
        return;
    }
    token::Client::new(&env, &token).transfer(&env.current_contract_address(), &to, &amount);
}
