import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

export interface GlobalPool {
    superAdmin: PublicKey,      // 32
    totalRound: anchor.BN,      // 8
}

export interface AccountData {
    name: String,
    nftMint: PublicKey,
}

export interface GameData {
    playTime: anchor.BN,         // 8
    amout: anchor.BN,           // 8
    rewardAmount: anchor.BN,    // 8
    setNum: anchor.BN,
    rand: anchor.BN,

}

export interface PlayerPool {
    // 8 + 96 = 104
    player: PublicKey,               // 32
    round: anchor.BN,                // 8
    gameData: GameData,              // 40
    winTimes: anchor.BN,              // 8
    receivedReward: anchor.BN,        // 8
}
