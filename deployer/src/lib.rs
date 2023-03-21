#![no_std]

use soroban_sdk::{
    contractimpl, contracttype, symbol, Address, Bytes, BytesN, Env, IntoVal, RawVal, Vec,
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

        let _: RawVal = env.invoke_contract(&id, &init_fn, add_exp(&env,init_args.clone()));
        let game = create_game(&env, &init_args);
        set_game(&env, &id, game);
        id
    }

    pub fn game(env: Env, id: BytesN<32>) -> Game {
        assert!(has_game(&env, &id), "Game doesn't exist");
        let mut game = get_game(&env, &id);
        if !game.ended {
            let client = game_contract::Client::new(&env, &id);
            game.ended = client.ended();
            set_game(&env, &id, game.clone());
        }
        game
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

fn add_exp(env: &Env, init_args: Vec<RawVal>) -> Vec<RawVal>{
    let duration = 60 * 10;
    let expiration = env.ledger().timestamp() + duration;
    let mut init_args_exp = init_args;
    init_args_exp.push_back(expiration.into_val(env));
    init_args_exp
}

mod test;
