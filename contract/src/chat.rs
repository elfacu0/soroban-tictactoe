use crate::storage::DataKey;
use soroban_sdk::{contracttype, vec, Address, Env, Symbol, Vec};

#[derive(Clone, Debug)]
#[contracttype]
pub struct Message {
    pub author: Address,
    pub body: Symbol,
}

pub fn get_chats(env: &Env) -> Vec<Message> {
    env.storage()
        .instance()
        .get(&DataKey::Chats)
        .unwrap_or(vec![env])
}

pub fn add_msg(env: &Env, player: Address, message: Symbol) -> Message {
    player.require_auth();

    let mut chats = get_chats(env);
    let msg = Message {
        author: player,
        body: message,
    };
    chats.push_back(msg.clone());
    env.storage().instance().set(&DataKey::Chats, &chats);
    msg
}
