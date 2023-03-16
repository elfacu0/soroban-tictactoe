#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Bytes, BytesN, Env, RawVal, Symbol, Vec, IntoVal};

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
    ) -> (BytesN<32>, RawVal) {
        let id = env
            .deployer()
            .with_current_contract(&salt)
            .deploy(&wasm_hash);
        let init_fn = Symbol::short("init");
        let res: RawVal = env.invoke_contract(&id, &init_fn, init_args.clone());
        set_game(&env, &id, &init_args);
        (id, res)
    }

    pub fn game(env: Env, id: BytesN<32>) -> Game {
        assert!(has_game(&env, &id), "Game doesn't exist");
        get_game(&env, id)
    }
}

fn has_game(env: &Env, id: &BytesN<32>) -> bool {
    let key = DataKey::Games(id.clone());
    env.storage().has(&key)
}

fn get_game(env: &Env, id: BytesN<32>) -> Game {
    let key = DataKey::Games(id.clone());
    env.storage().get(&key).unwrap().unwrap()
}

fn set_game(env: &Env, id: &BytesN<32>, init_args: &Vec<RawVal>) {
    let key = DataKey::Games(id.clone());
    let player_a = init_args.get(0).unwrap().unwrap().into_val(env);
    let player_b = init_args.get(1).unwrap().unwrap().into_val(env);
    let game = Game {
        player_a,
        player_b,
        ended: false,
    };
    env.storage().set(&key, &game)
}

mod test;
