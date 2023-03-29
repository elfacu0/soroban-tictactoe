#![no_std]
use soroban_sdk::{
    contractimpl, contracttype, symbol, vec, Address, Bytes, BytesN, Env, IntoVal, RawVal, Vec,
};

mod game_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/tictactoe_game.wasm"
    );
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    pub player_a: Address,
    pub player_b: Address,
    pub ended: bool,
}

#[contracttype]
pub enum DataKey {
    Games(BytesN<32>),
    Scores,
}

pub struct Deployer;

#[contractimpl]
impl Deployer {
    pub fn deploy(
        env: Env,
        salt: Bytes,
        wasm_hash: BytesN<32>,
        init_args: Vec<RawVal>,
    ) -> BytesN<32> {
        let id = env
            .deployer()
            .with_current_contract(&salt)
            .deploy(&wasm_hash);
        let init_fn = symbol!("init");

        let _: RawVal = env.invoke_contract(&id, &init_fn, add_exp(&env, init_args.clone()));
        let game = create_game(&env, &init_args);
        set_game(&env, &id, game);
        id
    }

    pub fn game(env: Env, id: BytesN<32>) -> Game {
        assert!(has_game(&env, &id), "Game doesn't exist");
        let mut game = get_game(&env, &id);
        if !game.ended {
            let client = game_contract::Client::new(&env, &id);
            if client.ended() {
                game.ended = true;
                set_game(&env, &id, game.clone());
                if client.has_winner() {
                    add_win(&env, client.winner());
                }
            }
        }
        game
    }

    pub fn scores(env: Env) -> Vec<(Address, u32)> {
        get_scores(&env)
    }
}

fn has_game(env: &Env, id: &BytesN<32>) -> bool {
    let key = DataKey::Games(id.clone());
    env.storage().has(&key)
}

fn get_game(env: &Env, id: &BytesN<32>) -> Game {
    let key = DataKey::Games(id.clone());
    env.storage().get(&key).unwrap().unwrap()
}

fn create_game(env: &Env, init_args: &Vec<RawVal>) -> Game {
    let player_a = init_args.get(0).unwrap().unwrap().into_val(env);
    let player_b = init_args.get(1).unwrap().unwrap().into_val(env);
    let game = Game {
        player_a,
        player_b,
        ended: false,
    };
    game
}

fn set_game(env: &Env, id: &BytesN<32>, game: Game) {
    let key = DataKey::Games(id.clone());
    env.storage().set(&key, &game)
}

fn add_exp(env: &Env, init_args: Vec<RawVal>) -> Vec<RawVal> {
    let duration = 60 * 10;
    let expiration = env.ledger().timestamp() + duration;
    let mut init_args_exp = init_args;
    init_args_exp.push_back(expiration.into_val(env));
    init_args_exp
}

fn get_scores(env: &Env) -> Vec<(Address, u32)> {
    let default = vec![env];
    env.storage()
        .get(&DataKey::Scores)
        .unwrap_or(Ok(default))
        .unwrap()
}

fn add_win(env: &Env, player: Address) {
    let score = get_score(env, player.clone());
    set_score(env, player, score + 1);
}

fn get_score(env: &Env, player: Address) -> u32 {
    let scores = get_scores(env);
    let score = scores.iter().find(|x| x.as_ref().unwrap().0 == player);
    match score {
        Some(val) => val.unwrap().1,
        _ => 0,
    }
}

fn set_score(env: &Env, player: Address, score: u32) {
    let mut scores = get_scores(env);
    let index = scores.iter().position(|x| x.as_ref().unwrap().0 == player);
    match index {
        Some(i) => scores.set(i.try_into().unwrap(), (player, score)),
        _ => scores.push_back((player, score)),
    }
    env.storage().set(&DataKey::Scores, &scores);
}

mod test;
