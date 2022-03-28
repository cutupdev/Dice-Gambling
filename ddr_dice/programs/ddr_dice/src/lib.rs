use anchor_lang::{prelude::*, AnchorDeserialize, System};

use solana_program::pubkey::Pubkey;
use solana_program::{program::invoke, system_instruction};

pub mod account;
pub mod constants;
pub mod error;
pub mod utils;

use account::*;
use constants::*;
use error::*;
use utils::*;

declare_id!("FbudCGq7GiwGhUV3kEaFWfyFE1Lbxs6VNcdn5Jo14nJQ");

#[program]
pub mod ddr_dice {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, global_bump: u8, vault_bump: u8) -> ProgramResult {
        let global_authority = &mut ctx.accounts.global_authority;
        global_authority.super_admin = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn initialize_player_pool(
        ctx: Context<InitializePlayerPool>,
    ) -> ProgramResult {
        let mut player_pool = ctx.accounts.player_pool.load_init()?;
        player_pool.player = ctx.accounts.owner.key();
        msg!("Owner: {:?}", player_pool.player.to_string());

        Ok(())
    }

    /**
    The main function to play dice.
    Input Args:
    set_number: The number is set by a player to play
    deposit:    The SOL amount to deposit 
    */
    #[access_control(user(&ctx.accounts.player_pool, &ctx.accounts.owner))]
    pub fn play_game(
        ctx: Context<PlayRound>,
        global_bump: u8,
        vault_bump: u8,
        set_number: u64,
        deposit: u64,
    ) -> ProgramResult {
        let mut player_pool = ctx.accounts.player_pool.load_mut()?;
        msg!("Deopsit: {}", deposit);
        msg!(
            "Vault: {}",
            ctx.accounts.reward_vault.to_account_info().key()
        );
        msg!(
            "Lamports: {}",
            ctx.accounts.reward_vault.to_account_info().lamports()
        );
        msg!(
            "Owner Lamports: {}",
            ctx.accounts.owner.to_account_info().lamports()
        );
        require!(
            ctx.accounts.owner.to_account_info().lamports() > deposit,
            GameError::InsufficientUserBalance
        );

        require!(
            ctx.accounts.reward_vault.to_account_info().lamports() > 2 * deposit,
            GameError::InsufficientRewardVault
        );

        // Transfer deposit Sol to this PDA
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.reward_vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            deposit,
        )?;

        // 0.5% of deposit Sol
        let treasury_price = deposit * 5 / 100 / 10;

        // Transfer SOL to the treasury_wallet1
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet1.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;
        // Transfer SOL to the treasury_wallet2
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet2.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;
        // Transfer SOL to the treasury_wallet3
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet3.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;
        // Transfer SOL to the treasury_wallet4
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet4.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;
        // Transfer SOL to the treasury_wallet5
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet5.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;
        // Transfer SOL to the treasury_wallet6
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet6.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;
        // Transfer SOL to the treasury_wallet7
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet7.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;
        // Transfer SOL to the treasury_wallet8
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet8.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;
        // Transfer SOL to the treasury_wallet9
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet9.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;
        // Transfer SOL to the treasury_wallet10
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.treasury_wallet10.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            treasury_price,
        )?;

        // Generate random number
        let mut reward: u64 = 0;
        let timestamp = Clock::get()?.unix_timestamp;
        let owner_address = ctx.accounts.owner.to_account_info().key();
        let (player_address, bump) = Pubkey::find_program_address(
            &[
                RANDOM_SEED.as_bytes(),
                timestamp.to_string().as_bytes(),
                &owner_address.to_bytes(),
            ],
            &ddr_dice::ID,
        );

        let char_vec: Vec<char> = player_address.to_string().chars().collect();
        let number = u32::from(char_vec[0]) + u32::from(char_vec[2]) + u32::from(char_vec[4]);
        let rand = (number % 6 + 1) as u64;
        
        // Compare random number and set_number
        if (rand - 1) / 3 as u64 == (set_number - 1) / 3 as u64 {
            reward = 2 * deposit;
        }

        // Add game data to the blockchain
        player_pool.add_game_data(timestamp, deposit, reward, set_number, rand);

        ctx.accounts.global_authority.total_round += 1;

        Ok(())
    }

    /**
    The claim Reward function after playing
    */
    #[access_control(user(&ctx.accounts.player_pool, &ctx.accounts.owner))]
    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        global_bump: u8,
        vault_bump: u8,
    ) -> ProgramResult {
        let mut player_pool = ctx.accounts.player_pool.load_mut()?;
        let reward = player_pool.game_data.reward_amount;
        require!(
            ctx.accounts.reward_vault.to_account_info().lamports() > reward,
            GameError::InsufficientRewardVault
        );
        if reward > 0 {
            // Transfer SOL to the winner from the PDA
            sol_transfer_with_signer(
                ctx.accounts.reward_vault.to_account_info(),
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                &[&[VAULT_AUTHORITY_SEED.as_ref(), &[vault_bump]]],
                reward,
            )?;
            player_pool.game_data.reward_amount = 0;
        }
        Ok(())
    }

    /**
    Withdraw function to withdraw SOL from the PDA with amount
    Args:
    amount: The sol amount to withdraw from this PDA
    Only Admin can withdraw SOL from this PDA
    */
    pub fn withdraw(
        ctx: Context<Withdraw>,
        global_bump: u8,
        vault_bump: u8,
        amount: u64,
    ) -> ProgramResult {
        let global_authority = &mut ctx.accounts.global_authority;
        require!(
            ctx.accounts.admin.key() == global_authority.super_admin,
            GameError::InvalidAdmin
        );
        sol_transfer_with_signer(
            ctx.accounts.reward_vault.to_account_info(),
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            &[&[VAULT_AUTHORITY_SEED.as_ref(), &[vault_bump]]],
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(global_bump: u8, vault_bump: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
        payer = admin
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(
        seeds = [VAULT_AUTHORITY_SEED.as_ref()],
        bump = vault_bump,
    )]
    pub reward_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializePlayerPool<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(zero)]
    pub player_pool: AccountLoader<'info, PlayerPool>,
}

#[derive(Accounts)]
#[instruction(
    global_bump: u8,
    vault_bump: u8,
)]
pub struct PlayRound<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub player_pool: AccountLoader<'info, PlayerPool>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED.as_ref()],
        bump = vault_bump,
    )]
    pub reward_vault: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet1.key() == TREASURY_WALLET1.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet1: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet2.key() == TREASURY_WALLET2.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet2: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet3.key() == TREASURY_WALLET3.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet3: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet4.key() == TREASURY_WALLET4.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet4: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet5.key() == TREASURY_WALLET5.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet5: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet6.key() == TREASURY_WALLET6.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet6: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet7.key() == TREASURY_WALLET7.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet7: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet8.key() == TREASURY_WALLET8.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet8: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet9.key() == TREASURY_WALLET9.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet9: AccountInfo<'info>,

    #[account(
        mut,
        constraint = treasury_wallet10.key() == TREASURY_WALLET10.parse::<Pubkey>().unwrap(),
    )]
    pub treasury_wallet10: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    global_bump: u8,
    vault_bump: u8,
)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub player_pool: AccountLoader<'info, PlayerPool>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED.as_ref()],
        bump = vault_bump,
    )]
    pub reward_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    global_bump: u8,
    vault_bump: u8,
)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED.as_ref()],
        bump = vault_bump,
    )]
    pub reward_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
// Access control modifiers
fn user(pool_loader: &AccountLoader<PlayerPool>, user: &AccountInfo) -> Result<()> {
    let user_pool = pool_loader.load()?;
    require!(user_pool.player == *user.key, GameError::InvalidPlayerPool);
    Ok(())
}
