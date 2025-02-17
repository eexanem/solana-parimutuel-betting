#![allow(unexpected_cfgs)]  
#![allow(unused_imports)]
use anchor_lang::prelude::*;

use mpl_token_metadata::instructions::CreateMetadataAccountV3;

declare_id!("8hLP8PTe7JqorACvs8oPeu7Uop8y6j2Fn9RQVp8ZBzxH"); // Update after anchor build

#[program]
pub mod betting_program {
    use super::*;

    // Initialize a betting pool with PDA
    pub fn initialize_pool(ctx: Context<InitializePool>, pool_id: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.pool_id = pool_id;
        pool.admin = *ctx.accounts.admin.key;
        Ok(())
    }

    // Place a bet and mint NFT ticket (simplified)
    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
        // Transfer SOL to pool (simulate betting)
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.pool.to_account_info().try_borrow_mut_lamports()? += amount;

        // TODO: Add Metaplex NFT minting logic
        Ok(())
    }
}

// PDA-managed pool account
#[account]
pub struct Pool {
    pub pool_id: u64,
    pub admin: Pubkey,
    pub total_bets: u64,
}

// Accounts for initialize_pool (PDA-based)
#[derive(Accounts)]
#[instruction(pool_id: u64)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 8 + 32 + 8, // Anchor discriminator + pool_id + admin + total_bets
        seeds = [b"pool", admin.key.as_ref(), &pool_id.to_le_bytes()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Accounts for place_bet (simplified)
#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut, has_one = admin)] // Security: Validate pool admin
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub admin: Signer<'info>, 
    // TODO: Add Metaplex accounts (mint, metadata, etc.)
}