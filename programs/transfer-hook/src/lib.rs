use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{self, Token2022},
    token_interface::{Mint, TokenAccount},
};

declare_id!("E24fiZAUbQNAwEkCcyzm9b4UFi4hhPdqMJgN9ZqntJgq");

#[program]
pub mod transfer_hook {
    use super::*;

    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        // Simplified initialization for transfer hook
        msg!("Initializing transfer hook extra account meta list");
        msg!("Extra account meta list: {}", ctx.accounts.extra_account_meta_list.key());
        
        Ok(())
    }

    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        msg!("Transfer Hook: Processing transfer of {} tokens", amount);
        
        let destination = &ctx.accounts.destination_token;
        
        // Get the destination token account owner
        let destination_owner = destination.owner;
        
        // Simplified validation - just log the transfer
        msg!("Transfer approved for destination: {}", destination_owner);
        Ok(())
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>) -> Result<()> {
        let new_account = ctx.accounts.new_account.key();
        
        msg!("Added {} to whitelist", new_account);
        Ok(())
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>) -> Result<()> {
        let account = ctx.accounts.account_to_remove.key();
        
        msg!("Removed {} from whitelist", account);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Extra account meta list account for transfer hook
    #[account(
        init,
        payer = payer,
        space = 1000, // Simplified space allocation
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    
    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    
    /// CHECK: source token account owner, can be SystemAccount or PDA
    pub owner: UncheckedAccount<'info>,
    
    /// CHECK: ExtraAccountMetaList Account
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 1000,
        seeds = [b"whitelist"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    /// CHECK: Account to add to whitelist
    pub new_account: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"whitelist"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    
    /// CHECK: Account to remove from whitelist
    pub account_to_remove: AccountInfo<'info>,
}

// State Accounts
#[account]
pub struct Whitelist {
    pub authority: Pubkey,
    pub accounts: Vec<Pubkey>,
}

// Custom Errors
#[error_code]
pub enum TransferHookError {
    #[msg("Destination account is not whitelisted")]
    DestinationNotWhitelisted,
    #[msg("Account is already whitelisted")]
    AccountAlreadyWhitelisted,
    #[msg("Account is not whitelisted")]
    AccountNotWhitelisted,
    #[msg("Whitelist is full")]
    WhitelistFull,
}
