# Stellar Tic-tac-toe
This is a project that enables players to play Tic-tac-toe games on the Stellar network.

It consists of two contracts: the first contract is used to create and keep track of Tic-tac-toe games, and the second contract is used to play the games

# Contracts

## Manager contract
This contract is used to initialize a new tic-tac-toe game, define its players and who start first.
It Stores all the games and their states, which can be accessed later.

<br />

## Game contract
This contract manage the game status, to make a move you need to call the `play` function.
It checks for winning conditions and a tie, and ends the game when either condition is met
<br />

___

## Game Functions
### Init
Initialize the players using the `init` function. Only use this function if you are not deploying the game using the manager contract.

Player_a always starts first
```
Arguments:
    player_a: Address,
    player_b: Address
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

___

## Development
Before deploying the new game instances, the WASM code needs to be installed on-chain.

The install command will print out the hash derived from the WASM file
```
soroban contract install --wasm target/wasm32-unknown-unknown/release/tictactoe_game.wasm
```

Then you can run

```
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/tictactoe_manager.wasm \
    --id 0 \
    --fn deploy \
    -- \
    --salt 0000000000000000000000000000000000000000000000000000000000000000 \
    --wasm_hash 48a67a58d5d3d777a228aa99c4560fc2f3ebf6e0e56a39709a711b12929d5d51 \
    --init_args '[{"object":{"address":{"account": {"public_key_type_ed25519": "A" }}}}]' //TODO: fix this
```

<br/>

___

## Game Deployment Example
### Deploy Game Contract
```
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/tictactoe_game.wasm \
    --secret-key SDO2A45YT56K2V5B5W3PWQSFPGVHA4VKWY7MMXCBKOYPZJDADLCQC56D \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```


### Initialize Game
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


### Play
```
soroban contract invoke \
    --id 327d8243244fe6f3550951e862d09344cf3fbe319a4ed8c406e40867daf2f730 \
    --secret-key SDO2A45YT56K2V5B5W3PWQSFPGVHA4VKWY7MMXCBKOYPZJDADLCQC56D \
    --rpc-url https://horizon-futurenet.stellar.cash:443/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn play \
    -- \
    --player GBNSNP2FS3EG47OA7JSNO6CPWKDQLCAB7RKI7ROQZGSEXYEMBA66QHOE \
    --pos_x 0 \
    --pos_y 0
```

---
## TODO IF TIME IS SUFFICIENT
- [ ] Make grid n x n
- [ ] Allow bets
- [ ] Implement time limit
