import { Program, web3 } from '@project-serum/anchor';
import * as anchor from '@project-serum/anchor';
import {
    Keypair,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    Transaction,
    TransactionInstruction,
    sendAndConfirmTransaction
} from '@solana/web3.js';
import { Token, TOKEN_PROGRAM_ID, AccountLayout, MintLayout, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

import fs from 'fs';
import { AccountData, GlobalPool, PlayerPool } from './types';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';

const PLAYER_POOL_SIZE = 104;
const GLOBAL_AUTHORITY_SEED = "global-authority";
const VAULT_AUTHORITY_SEED = "vault-authority";
const ACCOUNT_SEED = "account-data"
const RANDOM_SEED = "set-random-number";

const PROGRAM_ID = "FbudCGq7GiwGhUV3kEaFWfyFE1Lbxs6VNcdn5Jo14nJQ";
const TRESURY_WALLET = new PublicKey("Fs8R7R6dP3B7mAJ6QmWZbomBRuTbiJyiR4QYjoxhLdPu");

anchor.setProvider(anchor.Provider.local(web3.clusterApiUrl('devnet')));
const solConnection = anchor.getProvider().connection;
const payer = anchor.getProvider().wallet;

let rewardVault: PublicKey = null;
let program: Program = null;

// Configure the client to use the local cluster.
// const walletKeypair = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(path.resolve("/home/fury/.config/solana/id.json"), 'utf-8'))), { skipValidation: true });

const idl = JSON.parse(
    fs.readFileSync(__dirname + "/ddr_dice.json", "utf8")
);

// Address of the deployed program.
const programId = new anchor.web3.PublicKey(PROGRAM_ID);

// Generate the program client from IDL.
program = new anchor.Program(idl, programId);
console.log('ProgramId: ', program.programId.toBase58());

const main = async () => {
    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );
    console.log('GlobalAuthority: ', globalAuthority.toBase58());

    const [rewardVault, vaultBump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_AUTHORITY_SEED)],
        program.programId
    );
    console.log('RewardVault: ', rewardVault.toBase58());

    // await initProject();

    const globalPool: GlobalPool = await getGlobalState();
    console.log("GlobalPool Admin =", globalPool.superAdmin.toBase58(), globalPool.totalRound.toNumber());

    // await initUserPool(payer.publicKey);
    // await playGame(payer.publicKey, 5, 0.1);
    await claim(payer.publicKey);
    // await withDraw(payer.publicKey, 0.5);

};

export const initProject = async (
) => {
    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );
    const [rewardVault, vaultBump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_AUTHORITY_SEED)],
        program.programId
    );

    const tx = await program.rpc.initialize(
        bump, vaultBump, {
        accounts: {
            admin: payer.publicKey,
            globalAuthority,
            rewardVault: rewardVault,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        },
        signers: [],
    });
    await solConnection.confirmTransaction(tx, "confirmed");

    console.log("txHash =", tx);
    return false;
}


export const initUserPool = async (
    userAddress: PublicKey,
) => {
    let playerPoolKey = await PublicKey.createWithSeed(
        userAddress,
        "player-pool",
        program.programId,
    );
    console.log(playerPoolKey.toBase58());

    let ix = SystemProgram.createAccountWithSeed({
        fromPubkey: userAddress,
        basePubkey: userAddress,
        seed: "player-pool",
        newAccountPubkey: playerPoolKey,
        lamports: await solConnection.getMinimumBalanceForRentExemption(PLAYER_POOL_SIZE),
        space: PLAYER_POOL_SIZE,
        programId: program.programId,
    });


    const tx = await program.rpc.initializePlayerPool(
        {
            accounts: {
                owner: userAddress,
                playerPool: playerPoolKey,
            },
            instructions: [
                ix
            ],
            signers: []
        });
    await solConnection.confirmTransaction(tx, "confirmed");

    console.log("Your transaction signature", tx);
}

export const playGame = async (userAddress: PublicKey, setNum: number, deposit: number) => {

    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );
    console.log('GlobalAuthority: ', globalAuthority.toBase58());

    const [rewardVault, vaultBump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_AUTHORITY_SEED)],
        program.programId
    );
    console.log('RewardVault: ', rewardVault.toBase58());

    let playerPoolKey = await PublicKey.createWithSeed(
        userAddress,
        "player-pool",
        program.programId,
    );
    console.log(playerPoolKey.toBase58());

    let poolAccount = await solConnection.getAccountInfo(playerPoolKey);
    if (poolAccount === null || poolAccount.data === null) {
        console.log('init');
        await initUserPool(userAddress);
    }

    const tx = await program.rpc.playGame(
        bump, vaultBump, new anchor.BN(setNum), new anchor.BN(deposit * 1000000000), {
        accounts: {
            owner: userAddress,
            playerPool: playerPoolKey,
            globalAuthority,
            rewardVault: rewardVault,
            treasuryWallet1: TRESURY_WALLET,
            treasuryWallet2: TRESURY_WALLET,
            treasuryWallet3: TRESURY_WALLET,
            treasuryWallet4: TRESURY_WALLET,
            treasuryWallet5: TRESURY_WALLET,
            treasuryWallet6: TRESURY_WALLET,
            treasuryWallet7: TRESURY_WALLET,
            treasuryWallet8: TRESURY_WALLET,
            treasuryWallet9: TRESURY_WALLET,
            treasuryWallet10: TRESURY_WALLET,
            systemProgram: SystemProgram.programId,
        },
        signers: [],
    });

    await solConnection.confirmTransaction(tx, "singleGossip");
    let userPoolData = await program.account.playerPool.fetch(playerPoolKey);
    console.log(userPoolData.gameData);
    return userPoolData.gameData;
}

export const claim = async (userAddress: PublicKey) => {

    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );
    console.log('GlobalAuthority: ', globalAuthority.toBase58());

    const [rewardVault, vaultBump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_AUTHORITY_SEED)],
        program.programId
    );

    let playerPoolKey = await PublicKey.createWithSeed(
        userAddress,
        "player-pool",
        program.programId,
    );
    console.log(playerPoolKey.toBase58());

    const tx = await program.rpc.claimReward(
        bump, vaultBump, {
        accounts: {
            owner: userAddress,
            playerPool: playerPoolKey,
            globalAuthority,
            rewardVault: rewardVault,
            systemProgram: SystemProgram.programId,
        }
    });
    await solConnection.confirmTransaction(tx, "singleGossip");
    console.log("CLaim Succeed!=========================");

}

export const withDraw = async (userAddress: PublicKey, deposit: number) => {

    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );
    console.log('GlobalAuthority: ', globalAuthority.toBase58());

    const [rewardVault, vaultBump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_AUTHORITY_SEED)],
        program.programId
    );

    const tx = await program.rpc.withdraw(
        bump, vaultBump, new anchor.BN(deposit * 1000000000), {
        accounts: {
            admin: userAddress,
            globalAuthority,
            rewardVault: rewardVault,
            systemProgram: SystemProgram.programId,
        }
    });
    await solConnection.confirmTransaction(tx, "singleGossip");
    console.log("WithDraw Succeed!=========================");

}


export const getGlobalState = async (
): Promise<GlobalPool | null> => {
    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(GLOBAL_AUTHORITY_SEED)],
        program.programId
    );
    try {
        let globalState = await program.account.globalPool.fetch(globalAuthority);
        return globalState as GlobalPool;
    } catch {
        return null;
    }
}

export const getUserPoolState = async (
    userAddress: PublicKey
): Promise<PlayerPool | null> => {
    if (!userAddress) return null;

    let playerPoolKey = await PublicKey.createWithSeed(
        userAddress,
        "player-pool",
        program.programId,
    );
    console.log('Player Pool: ', playerPoolKey.toBase58());
    try {
        let poolState = await program.account.playerPool.fetch(playerPoolKey);
        return poolState as PlayerPool;
    } catch {
        return null;
    }
}

main();