#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, map, symbol_short, Address, BytesN, Env, IntoVal, Map,
    Symbol, Val, Vec,
};

mod game_contract {
    soroban_sdk::contractimport!(file = "../game/tictactoe_game.wasm");
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
    Games(Address),
    Scores,
}

#[contract]
pub struct Deployer;

#[contractimpl]
impl Deployer {
    pub fn deploy(
        env: Env,
        salt: BytesN<32>,
        wasm_hash: BytesN<32>,
        init_args: Vec<Val>,
    ) -> Address {
        const INIT_FN: Symbol = symbol_short!("init");
        let deployed_address = env.deployer().with_current_contract(salt).deploy(wasm_hash);
        let _: Val = env.invoke_contract(
            &deployed_address,
            &INIT_FN,
            add_exp(&env, init_args.clone()),
        );

        let game = create_game(&env, &init_args);
        set_game(&env, &deployed_address, game);

        deployed_address
    }

    pub fn game(env: Env, id: Address) -> Game {
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

    pub fn scores(env: Env) -> Map<Address, u32> {
        get_scores(&env)
    }
}

fn has_game(env: &Env, id: &Address) -> bool {
    let key = DataKey::Games(id.clone());
    env.storage().instance().has(&key)
}

fn get_game(env: &Env, id: &Address) -> Game {
    let key = DataKey::Games(id.clone());
    env.storage().instance().get(&key).unwrap()
}

fn create_game(env: &Env, init_args: &Vec<Val>) -> Game {
    let player_a = init_args.get(0).unwrap().into_val(env);
    let player_b = init_args.get(1).unwrap().into_val(env);
    let game = Game {
        player_a,
        player_b,
        ended: false,
    };
    game
}

fn set_game(env: &Env, id: &Address, game: Game) {
    let key = DataKey::Games(id.clone());
    env.storage().instance().set(&key, &game)
}

fn add_exp(env: &Env, init_args: Vec<Val>) -> Vec<Val> {
    let duration = 60 * 10;
    let expiration = env.ledger().timestamp() + duration;
    let mut init_args_exp = init_args;
    init_args_exp.push_back(expiration.into_val(env));
    init_args_exp
}

fn get_scores(env: &Env) -> Map<Address, u32> {
    let default = map![env];
    env.storage()
        .instance()
        .get(&DataKey::Scores)
        .unwrap_or(default)
}

fn add_win(env: &Env, player: Address) {
    let score = get_score(env, player.clone());
    set_score(env, player, score + 1);
}

fn get_score(env: &Env, player: Address) -> u32 {
    let scores = get_scores(env);
    let score = scores.get(player);
    match score {
        Some(val) => val,
        _ => 0,
    }
}

fn set_score(env: &Env, player: Address, score: u32) {
    let mut scores = get_scores(env);
    scores.set(player, score);
    env.storage().instance().set(&DataKey::Scores, &scores);
}

mod test;
