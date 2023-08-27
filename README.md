# 🚀 Stellar Tic-Tac-Toe 🚀
This is a project that enables players to play Tic-tac-toe games on the Stellar network.

It consists of two contracts: the first contract is used to create and keep track of Tic-tac-toe games, and the second contract is used to play the games

To prevent excessively long games, each game has a default duration of 10 minutes, after which it will end automatically.

## Table of Contents
<ol>
<li><a href="#functions">Functions</a></li>
<li><a href="#install">Install and Build</a></li>
<li><a href="#deployment-to-futurenet">Deployment</a></li>
<li><a href="#playing-the-game">Interacting with game</a></li>
</ol>

# ⚠️ Version
This project was developed using the Soroban SDK and Soroban CLI version 0.9.4. Please note that it is not compatible with earlier versions and may not be compatible with future releases


# Contracts

## Manager contract
This contract is used to initialize a new tic-tac-toe game and define its players. The order matters; the first player starts first.
It stores all the games and their states, which can be accessed later.

<br />

## Game contract
This contract manages the game itself. To make a move, you need to call the `play` function.
It checks for winning conditions or a tie and ends the game when either condition is met.

It also allows the player to make bets. If there is a winner, the minimum bet is paid and the rest is returned.
If there is no winner at the end of the game, the players ask for their bet to be returned.

<br />

---

<br />

# Functions

## Game Functions
### Init
Initialize the players using the `init` function.

⚠️ Only use this function if you are not deploying the game using the manager contract.

Player_a always starts first
```
Arguments:
    player_a: Address,
    player_b: Address,
    expiration: u64       // Expiration as unix timestamp
```

### Play
To play, each player needs to call the play function and pass their own address and the desired position to mark as arguments.

An error will be thrown if the player is not one of the two players, if it is not their turn, or if the player's address doesn't match the caller's address.

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
    message: Symbol
```

### Chat
To view all previous messages in the chat, call the `chat` function without any argument.

<br/>

## Manager Functions
### Deploy
⚠️ Before deploying you must deploy the Game contract


Deploy new games using the `deploy` function with the following arguments:
```
Arguments:
    salt: Bytes,
    wasm_hash: BytesN<32>, // the hash of the game contract 
    init_args: Vec<Val> // init_args should contain player_a and player_b addresses
```
It will return the Address of the Game contract

### Get game information
The manager stores all the deployed game and its status,
call `game` with the game address to know the players and if the game is ended
```
Arguments:
    id: Address // Game address
```

<br/>

---

<br />

# Install
## Clone this repository
```
git clone https://github.com/elfacu0/soroban-tictactoe.git
```

## You can run the test with
```
cargo test
```

## Build the Contracts
⚠️ Since the contract manager uses certain functions from the game contract, you need to first build the game contract and place it inside the game folder. (In this repository, a game contract is already provided in the game folder.)

To build the contracts, execute the following command:
```
soroban contract build
```

<br/>

---

<br />

# Deployment to Futurenet

## Configure Futurenet in your CLI
You can do this by running the following command:
```
soroban config network add --global futurenet \
  --rpc-url https://rpc-futurenet.stellar.org:443 \
  --network-passphrase "Test SDF Future Network ; October 2022"
```

## Configure Identities
```
soroban config identity generate --global alice
soroban config identity generate --global bob
```
From here onwards alice represents the Address GD324GL3IVXNY4GR4JOWINAD3RHYNVYN4LT4HH4QC7CTHN7ZJBHU4AEX

## Fund the Addresses
```
curl "https://friendbot-futurenet.stellar.org/?addr=$(soroban config identity address alice)"
curl "https://friendbot-futurenet.stellar.org/?addr=$(soroban config identity address bob)"
```

## Manager Deployment Example
Deploy the manger contract
```
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/tictactoe_manager.wasm \
    --source alice \
    --network futurenet
```
This will return the Address the the deployer contract
Eg: CANQ55GIJUEVKFCMDTR43PEXKMCDIU77AWCATVRIVZKY5HM2XF3CKDK5

### Install WASM Game file
```
soroban contract install \
    --wasm target/wasm32-unknown-unknown/release/tictactoe_game.wasm \
    --source alice \
    --network futurenet
```
This will return the id of the game
Eg: 010dab4c6fa53e9d9e673e0319d23d682ad352ea4b6b75726ed2e92fb7e30c96

### Deploy new instances of the game
id: Manager Contract Address
wasm_hash: hash returned from the installation of the game
salt: BytesN<32> salt, make sure no to use the same salt twice 
public_key_type_ed25519: Address of the players as Hex, see [Convert Address into Hex](#utils)
```
soroban contract invoke \
    --id CANQ55GIJUEVKFCMDTR43PEXKMCDIU77AWCATVRIVZKY5HM2XF3CKDK5 \
    --source alice \
    --network futurenet \
    -- deploy \
    --salt 0000000000000000000000000000000000000000000000000000000000000000 \
    --wasm_hash 010dab4c6fa53e9d9e673e0319d23d682ad352ea4b6b75726ed2e92fb7e30c96 \
    --init_args '[{"address":{"account":{"public_key_type_ed25519":"f7ae197b456edc70d1e25d643403dc4f86d70de2e7c39f9017c533b7f9484f4e"}}}, {"address":{"account":{"public_key_type_ed25519":"097550c7985a8b04704215307fec174bbfa9b9aea3fb545e0d92d70ce403bccf"}}}]'
```
This will return the Contract Address of the game
Eg: CC6JF7LCBH7B57KBUUG7Q5DFFEGXUENME445H4WBSYLDF6IB3M3BCD7I

<br />

---

## Game Contract can also be deployed without manager
### (Alternative) Deploy Game Contract
You can also deploy the game without using the manager
```
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/tictactoe_game.wasm \
    --source alice \
    --network futurenet
```

### (Alternative) Initialize Game (Only if the game was deployed without using the manager)
```
soroban contract invoke \
    --id 327d8243244fe6f3550951e862d09344cf3fbe319a4ed8c406e40867daf2f730 \
    --source alice \
    --network futurenet \
    -- init \
    --player_a GD324GL3IVXNY4GR4JOWINAD3RHYNVYN4LT4HH4QC7CTHN7ZJBHU4AEX \
    --player_b GBXDNBCQAFWKYZT6YVZUPVYMEWUD6OM5NABUILIDCVC3RFCXSNWEEQEZ
```

---

<br />

## Playing the game

### Play
id: Hash returned after calling the deploy function from the manager

player: Address of the player playing the game, if not the same as the previously defined player will throw an error
```
soroban contract invoke \
    --id CC6JF7LCBH7B57KBUUG7Q5DFFEGXUENME445H4WBSYLDF6IB3M3BCD7I \
    --source alice \
    --network futurenet \
    -- play \
    --player $(soroban config identity address alice) \
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
## Possible upgrades
- [ ] Make grid n x n