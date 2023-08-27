#![cfg(test)]

use crate::chat::Message;
use soroban_sdk::symbol_short;

use super::{GameContract, GameContractClient};
use soroban_sdk::testutils::{Ledger, LedgerInfo};
use soroban_sdk::Symbol;
use soroban_sdk::{testutils::Address as _, vec, Address, Env};

struct GameTest<'a> {
    env: Env,
    player_a: Address,
    player_b: Address,
    expiration: u64,
    client: GameContractClient<'a>,
}

impl GameTest<'_> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().set(LedgerInfo {
            timestamp: 12345,
            protocol_version: 1,
            sequence_number: 10,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_expiration: 1000,
            min_persistent_entry_expiration: 1000,
            max_entry_expiration: 1000,
        });

        let contract_id = env.register_contract(None, GameContract);
        let client = GameContractClient::new(&env, &contract_id);

        let player_a = Address::random(&env);
        let player_b = Address::random(&env);

        let duration = 60 * 10;
        let expiration = 12345 + duration;

        GameTest {
            env,
            player_a,
            player_b,
            expiration,
            client,
        }
    }

    fn make_player_a_win(client: &GameContractClient, player_a: &Address, player_b: &Address) {
        client.play(&player_a, &0, &0);
        client.play(&player_b, &0, &1);
        client.play(&player_a, &1, &0);
        client.play(&player_b, &1, &1);
        client.play(&player_a, &2, &0);
    }
}

#[test]
fn test_initialize() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    assert_eq!(client.player_a(), player_a);
    assert_eq!(client.player_b(), player_b);

    assert_eq!(client.turn(), player_a);
}

#[test]
#[should_panic]
fn test_already_initialized() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);
    client.init(&player_a, &player_b, &expiration);
}

#[test]
fn test_change_turn() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
    assert_eq!(client.turn(), player_b);

    client.play(&player_b, &(pos_x - 1), &(pos_y - 1));
    assert_eq!(client.turn(), player_a);
}

#[test]
#[should_panic]
fn test_other_player() {
    let GameTest {
        env,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();
    let player_c = Address::random(&env);

    client.init(&player_a, &player_b, &expiration);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
    client.play(&player_c, &(pos_x - 1), &(pos_y - 1));
}

#[test]
#[should_panic]
fn test_twice_play() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
    client.play(&player_a, &(pos_x - 1), &(pos_y - 1));
}

#[test]
fn test_mark_empty_cell() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
}

#[test]
#[should_panic]
fn test_mark_used_cell() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    let pos_x: u32 = 2;
    let pos_y: u32 = 2;

    client.play(&player_a, &pos_x, &pos_y);
    client.play(&player_b, &pos_x, &pos_y);
}

#[test]
#[should_panic]
fn test_no_winner() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    client.winner();
}

#[test]
fn test_winner_a() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    GameTest::make_player_a_win(&client, &player_a, &player_b);

    assert_eq!(client.ended(), true);
    assert_eq!(client.winner(), player_a);
}

#[test]
fn test_winner_b() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    client.play(&player_a, &2, &0);
    client.play(&player_b, &0, &0);
    client.play(&player_a, &1, &0);
    client.play(&player_b, &0, &1);
    client.play(&player_a, &1, &1);
    client.play(&player_b, &0, &2);

    assert_eq!(client.ended(), true);
    assert_eq!(client.winner(), player_b);
}

#[test]
#[should_panic]
fn test_game_over() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    assert_eq!(client.ended(), false);

    client.play(&player_a, &0, &0);
    client.play(&player_b, &0, &1);
    client.play(&player_a, &1, &0);
    client.play(&player_b, &1, &1);
    assert_eq!(client.ended(), false);
    client.play(&player_a, &2, &0); //player_a  already won
    assert_eq!(client.ended(), true);
    client.play(&player_b, &1, &2);
}

#[test]
#[should_panic]
fn test_draw() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    client.play(&player_a, &0, &0);
    client.play(&player_b, &1, &0);
    client.play(&player_a, &2, &0);

    client.play(&player_b, &2, &1);
    client.play(&player_a, &0, &1);
    client.play(&player_b, &1, &1);

    client.play(&player_a, &1, &2);
    client.play(&player_b, &0, &2);
    client.play(&player_a, &2, &2);

    client.winner();
}

#[test]
fn test_grid() {
    let GameTest {
        env,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    const EMPTY: Symbol = symbol_short!("");
    const X: Symbol = symbol_short!("X");
    const O: Symbol = symbol_short!("O");
    let mut grid = vec![
        &env,
        EMPTY.clone(),
        EMPTY.clone(),
        EMPTY.clone(),
        EMPTY.clone(),
        EMPTY.clone(),
        EMPTY.clone(),
        EMPTY.clone(),
        EMPTY.clone(),
        EMPTY.clone(),
    ];

    client.init(&player_a, &player_b, &expiration);
    assert_eq!(client.grid(), grid);

    client.play(&player_a, &2, &2);
    grid.set(0, X.clone());
    assert_eq!(client.grid(), grid);

    client.play(&player_b, &0, &2);
    grid.set(2, O.clone());
    assert_eq!(client.grid(), grid);

    client.play(&player_a, &1, &1);
    grid.set(4, X.clone());
    assert_eq!(client.grid(), grid);

    client.play(&player_b, &1, &0);
    grid.set(7, O.clone());
    assert_eq!(client.grid(), grid);
}

#[test]
fn test_expired() {
    let GameTest {
        env: _,
        player_a,
        player_b,
        expiration: _,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &100);

    assert_eq!(client.ended(), true);
}

#[test]
fn test_empty_chat() {
    let GameTest {
        env,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    assert_eq!(client.chat(), vec![&env]);
}

#[test]
fn test_send_message() {
    let GameTest {
        env,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    let msg = Message {
        author: player_a,
        body: symbol_short!("Hello"),
    };
    client.send_msg(&msg.author, &msg.body);

    assert_eq!(client.chat(), vec![&env, msg]);
}

#[test]
fn test_send_messages() {
    let GameTest {
        env,
        player_a,
        player_b,
        expiration,
        client,
    } = GameTest::setup();

    client.init(&player_a, &player_b, &expiration);

    let msg = Message {
        author: player_a,
        body: symbol_short!("Hello"),
    };
    client.send_msg(&msg.author, &msg.body);

    let msg2 = Message {
        author: player_b,
        body: symbol_short!("No"),
    };
    client.send_msg(&msg2.author, &msg2.body);

    assert_eq!(client.chat(), vec![&env, msg, msg2]);
}
