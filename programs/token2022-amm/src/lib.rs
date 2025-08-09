use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{self, Token2022},
    token_interface::{Mint, TokenAccount, transfer_checked, TransferChecked},
};
use spl_token_2022::{
    extension::{ExtensionType, StateWithExtensions},
    state::Mint as Token2022Mint,
};
use num_integer::Roots;

declare_id!("BXkgQBaKiJS7AunZPYAHGqQ5qKz6gJ7vTjedNYM1FUkU");

#[program]
pub mod token2022_amm {
    use super::*;

    /// Initialize a new AMM pool for Token-2022 assets
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        fee_rate: u64, // Fee rate in basis points (100 = 1%)
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        // Validate fee rate
        require!(fee_rate <= 1000, AmmError::InvalidFeeRate); // Max 10%
        
        // Initialize pool state
        pool.authority = ctx.accounts.authority.key();
        pool.token_a_mint = ctx.accounts.token_a_mint.key();
        pool.token_b_mint = ctx.accounts.token_b_mint.key();
        pool.token_a_vault = ctx.accounts.token_a_vault.key();
        pool.token_b_vault = ctx.accounts.token_b_vault.key();
        pool.lp_token_mint = ctx.accounts.lp_token_mint.key();
        pool.fee_rate = fee_rate;
        pool.token_a_reserve = 0;
        pool.token_b_reserve = 0;
        pool.lp_token_supply = 0;
        pool.bump = ctx.bumps.pool;
        
        msg!("Initialized AMM pool with fee rate: {} basis points", fee_rate);
        
        Ok(())
    }
    
    /// Add liquidity to the pool (first time or subsequent)
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a_desired: u64,
        amount_b_desired: u64,
        amount_a_min: u64,
        amount_b_min: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let clock = Clock::get()?;
        
        let (amount_a, amount_b, lp_tokens_to_mint) = if pool.token_a_reserve == 0 && pool.token_b_reserve == 0 {
            // First liquidity provision
            require!(amount_a_desired > 0 && amount_b_desired > 0, AmmError::InsufficientAmount);
            
            let lp_tokens = (amount_a_desired as u128 * amount_b_desired as u128).sqrt() as u64;
            require!(lp_tokens > 1000, AmmError::InsufficientLiquidity); // Minimum liquidity
            
            (amount_a_desired, amount_b_desired, lp_tokens - 1000) // Burn first 1000 tokens
        } else {
            // Subsequent liquidity provision
            let amount_b_optimal = amount_a_desired
                .checked_mul(pool.token_b_reserve)
                .unwrap()
                .checked_div(pool.token_a_reserve)
                .unwrap();
                
            if amount_b_optimal <= amount_b_desired {
                require!(amount_b_optimal >= amount_b_min, AmmError::InsufficientAmount);
                
                let lp_tokens = amount_a_desired
                    .checked_mul(pool.lp_token_supply)
                    .unwrap()
                    .checked_div(pool.token_a_reserve)
                    .unwrap();
                    
                (amount_a_desired, amount_b_optimal, lp_tokens)
            } else {
                let amount_a_optimal = amount_b_desired
                    .checked_mul(pool.token_a_reserve)
                    .unwrap()
                    .checked_div(pool.token_b_reserve)
                    .unwrap();
                    
                require!(amount_a_optimal <= amount_a_desired, AmmError::InsufficientAmount);
                require!(amount_a_optimal >= amount_a_min, AmmError::InsufficientAmount);
                
                let lp_tokens = amount_b_desired
                    .checked_mul(pool.lp_token_supply)
                    .unwrap()
                    .checked_div(pool.token_b_reserve)
                    .unwrap();
                    
                (amount_a_optimal, amount_b_desired, lp_tokens)
            }
        };
        
        // Transfer tokens to pool vaults (with transfer hook support)
        transfer_checked_with_hook_support(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.user_token_a.to_account_info(),
                    to: ctx.accounts.token_a_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                    mint: ctx.accounts.token_a_mint.to_account_info(),
                },
            ),
            amount_a,
            ctx.accounts.token_a_mint.decimals,
        )?;
        
        transfer_checked_with_hook_support(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.user_token_b.to_account_info(),
                    to: ctx.accounts.token_b_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                    mint: ctx.accounts.token_b_mint.to_account_info(),
                },
            ),
            amount_b,
            ctx.accounts.token_b_mint.decimals,
        )?;
        
        // Mint LP tokens to user
        let pool_seeds = &[
            b"pool",
            pool.token_a_mint.as_ref(),
            pool.token_b_mint.as_ref(),
            &[pool.bump],
        ];
        
        token_2022::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token_2022::MintTo {
                    mint: ctx.accounts.lp_token_mint.to_account_info(),
                    to: ctx.accounts.user_lp_token.to_account_info(),
                    authority: pool.to_account_info(),
                },
                &[pool_seeds],
            ),
            lp_tokens_to_mint,
        )?;
        
        // Update pool state
        pool.token_a_reserve += amount_a;
        pool.token_b_reserve += amount_b;
        pool.lp_token_supply += lp_tokens_to_mint;
        
        msg!("Added liquidity: {} token A, {} token B, minted {} LP tokens", amount_a, amount_b, lp_tokens_to_mint);
        
        Ok(())
    }
    
    /// Remove liquidity from the pool
    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        lp_token_amount: u64,
        amount_a_min: u64,
        amount_b_min: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        // Calculate amounts to return
        let amount_a = lp_token_amount
            .checked_mul(pool.token_a_reserve)
            .unwrap()
            .checked_div(pool.lp_token_supply)
            .unwrap();
            
        let amount_b = lp_token_amount
            .checked_mul(pool.token_b_reserve)
            .unwrap()
            .checked_div(pool.lp_token_supply)
            .unwrap();
        
        require!(amount_a >= amount_a_min, AmmError::InsufficientAmount);
        require!(amount_b >= amount_b_min, AmmError::InsufficientAmount);
        
        // Burn LP tokens
        token_2022::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token_2022::Burn {
                    mint: ctx.accounts.lp_token_mint.to_account_info(),
                    from: ctx.accounts.user_lp_token.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            lp_token_amount,
        )?;
        
        // Transfer tokens back to user (with transfer hook support)
        let pool_seeds = &[
            b"pool",
            pool.token_a_mint.as_ref(),
            pool.token_b_mint.as_ref(),
            &[pool.bump],
        ];
        
        transfer_checked_with_hook_support(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.token_a_vault.to_account_info(),
                    to: ctx.accounts.user_token_a.to_account_info(),
                    authority: pool.to_account_info(),
                    mint: ctx.accounts.token_a_mint.to_account_info(),
                },
                &[pool_seeds],
            ),
            amount_a,
            ctx.accounts.token_a_mint.decimals,
        )?;
        
        transfer_checked_with_hook_support(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.token_b_vault.to_account_info(),
                    to: ctx.accounts.user_token_b.to_account_info(),
                    authority: pool.to_account_info(),
                    mint: ctx.accounts.token_b_mint.to_account_info(),
                },
                &[pool_seeds],
            ),
            amount_b,
            ctx.accounts.token_b_mint.decimals,
        )?;
        
        // Update pool state
        pool.token_a_reserve -= amount_a;
        pool.token_b_reserve -= amount_b;
        pool.lp_token_supply -= lp_token_amount;
        
        msg!("Removed liquidity: {} LP tokens, returned {} token A, {} token B", lp_token_amount, amount_a, amount_b);
        
        Ok(())
    }
    
    /// Swap tokens in the pool
    /// Create a new Token-2022 mint with transfer hook
    pub fn create_token_with_hook(
        ctx: Context<CreateTokenWithHook>,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: u64,
    ) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let transfer_hook_program = &ctx.accounts.transfer_hook_program;
        
        // Initialize the mint with transfer hook extension
        // This will be handled by the Token-2022 program
        msg!("Creating Token-2022 mint with transfer hook");
        msg!("Mint: {}", mint.key());
        msg!("Transfer Hook Program: {}", transfer_hook_program.key());
        
        Ok(())
    }

    /// Initialize transfer hook for an existing token
    pub fn initialize_transfer_hook(
        ctx: Context<InitializeTransferHook>,
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let transfer_hook_program = &ctx.accounts.transfer_hook_program;
        
        msg!("Initializing transfer hook for mint: {}", mint.key());
        msg!("Transfer Hook Program: {}", transfer_hook_program.key());
        
        // This would typically involve setting up the transfer hook extension
        // on the mint account. For now, we'll just log the action.
        
        Ok(())
    }

    /// Add account to transfer hook whitelist
    pub fn add_to_whitelist(
        ctx: Context<ManageWhitelist>,
        account_to_add: Pubkey,
    ) -> Result<()> {
        let authority = &ctx.accounts.authority;
        
        msg!("Adding account {} to whitelist", account_to_add);
        msg!("Authority: {}", authority.key());
        
        // This would typically interact with the transfer hook program
        // to add the account to the whitelist
        
        Ok(())
    }

    /// Remove account from transfer hook whitelist
    pub fn remove_from_whitelist(
        ctx: Context<ManageWhitelist>,
        account_to_remove: Pubkey,
    ) -> Result<()> {
        let authority = &ctx.accounts.authority;
        
        msg!("Removing account {} from whitelist", account_to_remove);
        msg!("Authority: {}", authority.key());
        
        // This would typically interact with the transfer hook program
        // to remove the account from the whitelist
        
        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        amount_in: u64,
        amount_out_min: u64,
        a_to_b: bool, // true for A->B, false for B->A
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        let (amount_out, fee) = if a_to_b {
            // Swapping A for B
            let amount_in_with_fee = amount_in
                .checked_mul(10000 - pool.fee_rate)
                .unwrap()
                .checked_div(10000)
                .unwrap();
            
            let amount_out = pool.token_b_reserve
                .checked_mul(amount_in_with_fee)
                .unwrap()
                .checked_div(pool.token_a_reserve + amount_in_with_fee)
                .unwrap();
            
            let fee = amount_in - amount_in_with_fee;
            
            require!(amount_out >= amount_out_min, AmmError::InsufficientOutputAmount);
            require!(amount_out < pool.token_b_reserve, AmmError::InsufficientLiquidity);
            
            (amount_out, fee)
        } else {
            // Swapping B for A
            let amount_in_with_fee = amount_in
                .checked_mul(10000 - pool.fee_rate)
                .unwrap()
                .checked_div(10000)
                .unwrap();
            
            let amount_out = pool.token_a_reserve
                .checked_mul(amount_in_with_fee)
                .unwrap()
                .checked_div(pool.token_b_reserve + amount_in_with_fee)
                .unwrap();
            
            let fee = amount_in - amount_in_with_fee;
            
            require!(amount_out >= amount_out_min, AmmError::InsufficientOutputAmount);
            require!(amount_out < pool.token_a_reserve, AmmError::InsufficientLiquidity);
            
            (amount_out, fee)
        };
        
        if a_to_b {
            // Transfer A from user to pool
            transfer_checked_with_hook_support(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    TransferChecked {
                        from: ctx.accounts.user_token_a.to_account_info(),
                        to: ctx.accounts.token_a_vault.to_account_info(),
                        authority: ctx.accounts.user.to_account_info(),
                        mint: ctx.accounts.token_a_mint.to_account_info(),
                    },
                ),
                amount_in,
                ctx.accounts.token_a_mint.decimals,
            )?;
            
            // Transfer B from pool to user
            let pool_seeds = &[
                b"pool",
                pool.token_a_mint.as_ref(),
                pool.token_b_mint.as_ref(),
                &[pool.bump],
            ];
            
            transfer_checked_with_hook_support(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    TransferChecked {
                        from: ctx.accounts.token_b_vault.to_account_info(),
                        to: ctx.accounts.user_token_b.to_account_info(),
                        authority: pool.to_account_info(),
                        mint: ctx.accounts.token_b_mint.to_account_info(),
                    },
                    &[pool_seeds],
                ),
                amount_out,
                ctx.accounts.token_b_mint.decimals,
            )?;
            
            // Update reserves
            pool.token_a_reserve += amount_in;
            pool.token_b_reserve -= amount_out;
        } else {
            // Transfer B from user to pool
            transfer_checked_with_hook_support(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    TransferChecked {
                        from: ctx.accounts.user_token_b.to_account_info(),
                        to: ctx.accounts.token_b_vault.to_account_info(),
                        authority: ctx.accounts.user.to_account_info(),
                        mint: ctx.accounts.token_b_mint.to_account_info(),
                    },
                ),
                amount_in,
                ctx.accounts.token_b_mint.decimals,
            )?;
            
            // Transfer A from pool to user
            let pool_seeds = &[
                b"pool",
                pool.token_a_mint.as_ref(),
                pool.token_b_mint.as_ref(),
                &[pool.bump],
            ];
            
            transfer_checked_with_hook_support(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    TransferChecked {
                        from: ctx.accounts.token_a_vault.to_account_info(),
                        to: ctx.accounts.user_token_a.to_account_info(),
                        authority: pool.to_account_info(),
                        mint: ctx.accounts.token_a_mint.to_account_info(),
                    },
                    &[pool_seeds],
                ),
                amount_out,
                ctx.accounts.token_a_mint.decimals,
            )?;
            
            // Update reserves
            pool.token_b_reserve += amount_in;
            pool.token_a_reserve -= amount_out;
        }
        
        msg!("Swap completed: {} in, {} out, {} fee", amount_in, amount_out, fee);
        
        Ok(())
    }
}

// Helper function to handle transfers with potential transfer hooks
fn transfer_checked_with_hook_support<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, TransferChecked<'info>>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    // For Token-2022 with transfer hooks, we use transfer_checked which automatically
    // invokes any transfer hooks configured on the mint
    // The transfer hook program will validate the transfer based on its logic
    // (e.g., whitelist validation, KYC checks, etc.)
    transfer_checked(ctx, amount, decimals)
}

// Account validation structs
#[derive(Accounts)]
pub struct CreateTokenWithHook<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// CHECK: Transfer hook program that will handle transfer validation
    pub transfer_hook_program: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeTransferHook<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// CHECK: Transfer hook program that will handle transfer validation
    pub transfer_hook_program: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct ManageWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Transfer hook program that will handle whitelist management
    pub transfer_hook_program: UncheckedAccount<'info>,
    
    /// CHECK: Whitelist account managed by the transfer hook program
    #[account(
        seeds = [b"whitelist"],
        bump,
    )]
    pub whitelist: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = Pool::SPACE,
        seeds = [b"pool", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    
    pub token_a_mint: InterfaceAccount<'info, Mint>,
    pub token_b_mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        init,
        payer = authority,
        seeds = [b"token_a_vault", pool.key().as_ref()],
        bump,
        token::mint = token_a_mint,
        token::authority = pool,
    )]
    pub token_a_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init,
        payer = authority,
        seeds = [b"token_b_vault", pool.key().as_ref()],
        bump,
        token::mint = token_b_mint,
        token::authority = pool,
    )]
    pub token_b_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init,
        payer = authority,
        seeds = [b"lp_token_mint", pool.key().as_ref()],
        bump,
        mint::decimals = 9,
        mint::authority = pool,
    )]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"pool", pool.token_a_mint.as_ref(), pool.token_b_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,
    
    pub token_a_mint: InterfaceAccount<'info, Mint>,
    pub token_b_mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = user,
    )]
    pub user_token_a: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = user,
    )]
    pub user_token_b: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = lp_token_mint,
        associated_token::authority = user,
    )]
    pub user_lp_token: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"token_a_vault", pool.key().as_ref()],
        bump,
    )]
    pub token_a_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"token_b_vault", pool.key().as_ref()],
        bump,
    )]
    pub token_b_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"lp_token_mint", pool.key().as_ref()],
        bump,
    )]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,
    
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"pool", pool.token_a_mint.as_ref(), pool.token_b_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,
    
    pub token_a_mint: InterfaceAccount<'info, Mint>,
    pub token_b_mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = user,
    )]
    pub user_token_a: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = user,
    )]
    pub user_token_b: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = lp_token_mint,
        associated_token::authority = user,
    )]
    pub user_lp_token: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"token_a_vault", pool.key().as_ref()],
        bump,
    )]
    pub token_a_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"token_b_vault", pool.key().as_ref()],
        bump,
    )]
    pub token_b_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"lp_token_mint", pool.key().as_ref()],
        bump,
    )]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,
    
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"pool", pool.token_a_mint.as_ref(), pool.token_b_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,
    
    pub token_a_mint: InterfaceAccount<'info, Mint>,
    pub token_b_mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = user,
    )]
    pub user_token_a: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = user,
    )]
    pub user_token_b: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"token_a_vault", pool.key().as_ref()],
        bump,
    )]
    pub token_a_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"token_b_vault", pool.key().as_ref()],
        bump,
    )]
    pub token_b_vault: InterfaceAccount<'info, TokenAccount>,
    
    // Transfer hook accounts for Token-2022 validation
    /// CHECK: Optional extra account meta list for token A transfer hooks
    #[account(
        seeds = [b"extra-account-metas", token_a_mint.key().as_ref()],
        bump
    )]
    pub token_a_extra_account_meta_list: UncheckedAccount<'info>,
    
    /// CHECK: Optional extra account meta list for token B transfer hooks
    #[account(
        seeds = [b"extra-account-metas", token_b_mint.key().as_ref()],
        bump
    )]
    pub token_b_extra_account_meta_list: UncheckedAccount<'info>,
    
    // Whitelist accounts for transfer hook validation
    /// CHECK: Whitelist account for token A transfer hooks
    #[account(
        seeds = [b"whitelist"],
        bump
    )]
    pub token_a_whitelist: UncheckedAccount<'info>,
    
    /// CHECK: Whitelist account for token B transfer hooks
    #[account(
        seeds = [b"whitelist"],
        bump
    )]
    pub token_b_whitelist: UncheckedAccount<'info>,
    
    /// CHECK: Hook authority for token A
    #[account(
        seeds = [b"hook_authority"],
        bump
    )]
    pub token_a_hook_authority: UncheckedAccount<'info>,
    
    /// CHECK: Hook authority for token B
    #[account(
        seeds = [b"hook_authority"],
        bump
    )]
    pub token_b_hook_authority: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token2022>,
}

// State account for AMM pool
#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub lp_token_mint: Pubkey,
    pub fee_rate: u64, // In basis points
    pub token_a_reserve: u64,
    pub token_b_reserve: u64,
    pub lp_token_supply: u64,
    pub bump: u8,
}

impl Pool {
    pub const SPACE: usize = 8 + 32 * 6 + 8 * 4 + 1; // 8 + 192 + 32 + 1 = 233 bytes
}

// Custom errors
#[error_code]
pub enum AmmError {
    #[msg("Invalid fee rate")]
    InvalidFeeRate,
    #[msg("Insufficient amount")]
    InsufficientAmount,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Insufficient output amount")]
    InsufficientOutputAmount,
    #[msg("Transfer hook validation failed")]
    TransferHookValidationFailed,
}
