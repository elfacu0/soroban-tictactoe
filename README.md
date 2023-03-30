# 🚀 Stellar Tic-tac-toe 🚀
This is a project that enables players to play Tic-tac-toe games on the Stellar network.

It consists of two contracts: the first contract is used to create and keep track of Tic-tac-toe games, and the second contract is used to play the games

To prevent excessively long games, each game has a default duration of 10 minutes, after which it will end automatically.

# ⚠️ Version
This project was developed using the Soroban SDK and Soroban CLI version 0.6.0. Please note that it is not compatible with earlier versions and may not be compatible with future releases


# Contracts

## Manager contract
This contract is used to initialize a new tic-tac-toe game, define its players and who start first.
It Stores all the games and their states, which can be accessed later.

<br />

## Game contract
This contract manage the game status, to make a move you need to call the `play` function.
It checks for winning conditions and a tie, and ends the game when either condition is met

It also allows the player to make bets, if there is a winner the minium bet is paid and the rest is returned.
<br />

---

<br />

# Functions

## Game Functions
### Init
Initialize the players using the `init` function. Only use this function if you are not deploying the game using the manager contract.

Player_a always starts first
```
Arguments:
    player_a: Address,
    player_b: Address,
    expiration: u64       // Expiration as unix timestamp
```

### Play
To play, each player needs to call the `play` function and pass its own address and the desired position to mark as arguments.

An error will be thrown if the player is not one of the two players ,if it is not their turn or if the player address doesnt match the caller address.
```
Arguments:
    player: Address, 
    pos_x: u32,
    pos_y: u32
```

The position must be within the grid range and correspond to the following grid.
| 2-2 | 1-2 | 0-2 |
|-----|-----|-----|
| 2-1 | 1-1 | 0-1 |
| 2-0 | 1-0 | x=0-y=0 |

### Turn
To know whose turn it is, call the `turn` function without any argument.


### Grid
To view the grid you can call the `grid` function without any argument.

### Bet
Make a bet by calling the `bet` function. If a bet has already been made, the function will increase the existing bet by the specified amount. Note that players can only bet on their own victory.
```
Arguments:
    player: Address, 
    token: BytesN<32>,
    amount: i128,
```

### Collect Bet
After the game has ended, players can collect their winnings. In the event that they have bet a higher amount than their opponent, the difference will be returned to them.
For that call the `clct_bet` with your own address  
```
Arguments:
    player: Address, 
```

### Send Message
Players can interact with each other through a chat feature. To send a message, a player must call the `send_msg` function with the following arguments.
```
Arguments:
    player: Address, 
    message: Bytes
```

### Chat
To view all previous messages in the chat, call the `chat` function without any argument.

<br/>

## Manager Functions
### Deploy
Deploy new games using the `deploy` function with the following arguments:
```
Arguments:
    salt: Bytes,
    wasm_hash: BytesN<32>, // the hash of the game contract 
    init_args: Vec<RawVal> // init_args should contain player_a and player_b addresses
```
It will return the Address of the Game contract

### Get game information
The manager stores all the deployed game and its status,
call `game` with the game address to know the players and if the game is ended
```
Arguments:
    id: BytesN<32> // Game address
```


<br/>

---

<br />

# Local Development
Before deploying the new game instances, the WASM code needs to be installed on-chain.

The install command will print out the hash derived from the WASM file
```
soroban contract install --wasm target/wasm32-unknown-unknown/release/tictactoe_game.wasm
```

Then you can deploy the manager with

```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/tictactoe_manager.wasm \
    --id 0 \
    --fn deploy \
    -- \
    --salt 0000000000000000000000000000000000000000000000000000000000000000 \
    --wasm_hash 48a67a58d5d3d777a228aa99c4560fc2f3ebf6e0e56a39709a711b12929d5d51 \
    --init_args '[{"object":{"address":{"account":{"public_key_type_ed25519":"5b26bf4596c86e7dc0fa64d7784fb287058801fc548fc5d0c9a44be08c083de8"}}}}, {"object":{"address":{"account":{"public_key_type_ed25519":"6e368450016cac667ec57347d70c25a83f399d6803442d031545b89457936c42"}}}}]'
```

Inside `init_args` the first element is player_a and the second player_b, public_key_type_ed25519 must contain the address in hex

It will return the Address of the Game, use it to play the game

```
soroban contract invoke \
    --account GBXDNBCQAFWKYZT6YVZUPVYMEWUD6OM5NABUILIDCVC3RFCXSNWEEQEZ \
    --id 33d2785d1d0ca4d30bd0f6dc26d3a990b4f4d27bcbb44f429c49c8435a760bdc \
    --fn play \
    -- \
    --player GBXDNBCQAFWKYZT6YVZUPVYMEWUD6OM5NABUILIDCVC3RFCXSNWEEQEZ \
    --pos_x 0 \
    --pos_y 1 
```

<br/>

---

<br />

# Deployment

## Manager Deployment Example
Deploy the manger contract
```
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/tictactoe_manager.wasm \
    --secret-key SDO2A45YT56K2V5B5W3PWQSFPGVHA4VKWY7MMXCBKOYPZJDADLCQC56D \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```

### Install WASM Game file
```
soroban contract install \
    --wasm target/wasm32-unknown-unknown/release/tictactoe_game.wasm \
    --secret-key SDO2A45YT56K2V5B5W3PWQSFPGVHA4VKWY7MMXCBKOYPZJDADLCQC56D \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```

### Deploy new instances of the game
id: Manager Contract ID
wasm_hash: hash returned from the installation of the game
```
soroban contract invoke \
    --id 8d94a6e95049ed77625e176883e71d272e4948e5e2ea9ed438e0755af3246578 \
    --secret-key SDO2A45YT56K2V5B5W3PWQSFPGVHA4VKWY7MMXCBKOYPZJDADLCQC56D \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn deploy \
    -- \
    --salt 0000000000000000000000000000000000000000000000000000000000000000 \
    --wasm_hash 31690bfa9ca85e6e0b5c3d48fd3fd9cf5ab0df56d714dfb3b222be4e0b221011 \
    --init_args '[{"object":{"address":{"account":{"public_key_type_ed25519":"5b26bf4596c86e7dc0fa64d7784fb287058801fc548fc5d0c9a44be08c083de8"}}}}, {"object":{"address":{"account":{"public_key_type_ed25519":"6e368450016cac667ec57347d70c25a83f399d6803442d031545b89457936c42"}}}}]'
```

<br />

---

### (Alternative) Deploy Game Contract
You can also deploy the game without using the manager
```
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/tictactoe_game.wasm \
    --secret-key SDO2A45YT56K2V5B5W3PWQSFPGVHA4VKWY7MMXCBKOYPZJDADLCQC56D \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```

### (Alternative) Initialize Game (Only if the game was deployed without using the manager)
```
soroban contract invoke \
    --id 327d8243244fe6f3550951e862d09344cf3fbe319a4ed8c406e40867daf2f730 \
    --secret-key SDO2A45YT56K2V5B5W3PWQSFPGVHA4VKWY7MMXCBKOYPZJDADLCQC56D \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn init \
    -- \
    --player_a GBNSNP2FS3EG47OA7JSNO6CPWKDQLCAB7RKI7ROQZGSEXYEMBA66QHOE \
    --player_b GBXDNBCQAFWKYZT6YVZUPVYMEWUD6OM5NABUILIDCVC3RFCXSNWEEQEZ
```

---

<br />

### Play
id: Hash returned after calling the deploy function from the manager
```
soroban contract invoke \
    --id 5b88e05db7228ecb0f3f1929c94abba76ff674efbeb2653146db79e31feb0d3a \
    --secret-key SDO2A45YT56K2V5B5W3PWQSFPGVHA4VKWY7MMXCBKOYPZJDADLCQC56D \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn play \
    -- \
    --player GBNSNP2FS3EG47OA7JSNO6CPWKDQLCAB7RKI7ROQZGSEXYEMBA66QHOE \
    --pos_x 0 \
    --pos_y 0
```

<br/>

---
## Utils
### Convert Address into Hex
Using javascript stellar-sdk
```
StellarSdk.Keypair.fromPublicKey("GBXDNBCQAFWKYZT6YVZUPVYMEWUD6OM5NABUILIDCVC3RFCXSNWEEQEZ").rawPublicKey().toString('hex');
```  
[Online demo](https://jsfiddle.net/byrq3ntu/)

---
## TODO IF TIME IS SUFFICIENT
- [ ] Make grid n x n