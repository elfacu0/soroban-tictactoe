# Stellar Tic-tac-toe
This is a project that enables players to play Tic-tac-toe games on the Stellar network.

It consists of two contracts: the first contract is used to create and keep track of Tic-tac-toe games, and the second contract is used to play the games

# Contracts

## Manager contract
This contract is used to initialize a new tic-tac-toe game, define its players and who start first.
It Stores all the games and their states, which can be accessed later.

<br />

## Tic-tac-toe contract
This contract manage the game status, to make a move you need to call the `play` function.
It checks for winning conditions and a tie, and ends the game when either condition is met

---
## TODO IF TIME IS SUFFICIENT
- [ ] Make grid n x n
- [ ] Allow bets
- [ ] Implement time limit

