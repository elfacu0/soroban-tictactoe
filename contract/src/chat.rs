use crate::storage::DataKey;
use soroban_sdk::{contracttype, vec, Address, Bytes, Env, Vec};

#[derive(Clone, Debug)]
#[contracttype]
pub struct Message {
    pub author: Address,
    pub body: Bytes,
}

pub fn get_chats(env: &Env) -> Vec<Message> {
    env.storage()
        .get(&DataKey::Chats)
        .unwrap_or(Ok(vec![env]))
        .unwrap()
}

pub fn add_msg(env: &Env, player: Address, message: Bytes) -> Message {
    player.require_auth();

    let mut chats = get_chats(env);
    let msg = Message {
        author: player,
        body: message,
    };
    chats.push_back(msg.clone());
    env.storage().set(&DataKey::Chats, &chats);
    msg
}
