use soroban_sdk::contracttype;

#[contracttype]
pub enum DataKey {
    PlayerA,
    PlayerB,
    PlayerTurn,
    Grid,
    Winner,
    Time,
    Expiration,
    BetPlayerA,
    BetPlayerB,
    Chats,
}
