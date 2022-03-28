# DDR-Dice

## Install Dependencies
- Install `node` and `yarn`
- Install `ts-node` as global command
- Confirm the solana wallet preparation: `/home/fury/.config/solana/id.json` in test case

## Usage
- Main script source for all functionality is here: `/cli/scripts.ts`
- Program account types are declared here: `/cli/types.ts`
- Idl to make the JS binding easy is here: `/cli/dice_gaming.json`

Able to test the script functions working in this way.
- Change commands properly in the main functions of the `scripts.ts` file to call the other functions
- Confirm the `ANCHOR_WALLET` environment variable of the `ts-node` script in `package.json`
- Run `yarn ts-node`

## Features

### As a Smart Contract Owner
For the first time use, the Smart Contract Owner should `initialize` the Smart Contract for global account allocation.
- `initProject`

The smart contract owner can only withdraw SOL from this PDA
- `withdraw`
 
### As a player(Summary of Game Logic)
Players can play this game.

If the player set the number 1-3 and deposit 0.1 SOL, then call function `play_game`, the program generate the random number and compare with the set number. 

If the random number is in [1, 3], then he will win and call function `claim_reward` to receive the reward. The reward amount is double of deposit amount.

If the random number isn't in [1, 3], then he will lose.
