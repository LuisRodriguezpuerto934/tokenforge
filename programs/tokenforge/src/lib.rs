use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tokenforge {
    use super::*;

    // Launch a new token with one command
    pub fn launch_token(
        ctx: Context<LaunchToken>,
        name: String,
        symbol: String,
        supply: u64,
        decimals: u8,
    ) -> Result<()> {
        let token_data = &mut ctx.accounts.token_data;
        token_data.creator = ctx.accounts.creator.key();
        token_data.name = name;
        token_data.symbol = symbol;
        token_data.supply = supply;
        token_data.decimals = decimals;
        token_data.created_at = Clock::get()?.unix_timestamp;
        token_data.trading_enabled = false;
        
        // Mint initial supply to creator
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.creator_token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            supply,
        )?;
        
        msg!("Token launched: {} ({}) - Supply: {}", name, symbol, supply);
        Ok(())
    }
    
    // Enable trading (add liquidity)
    pub fn enable_trading(ctx: Context<EnableTrading>) -> Result<()> {
        let token_data = &mut ctx.accounts.token_data;
        require!(
            token_data.creator == ctx.accounts.creator.key(),
            ErrorCode::Unauthorized
        );
        
        token_data.trading_enabled = true;
        msg!("Trading enabled for token");
        Ok(())
    }
    
    // Distribute revenue to holders
    pub fn distribute_revenue(
        ctx: Context<DistributeRevenue>,
        amount: u64,
    ) -> Result<()> {
        // Revenue sharing logic
        msg!("Revenue distribution: {} tokens", amount);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, symbol: String)]
pub struct LaunchToken<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + TokenData::SIZE,
        seeds = [b"token_data", creator.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub token_data: Account<'info, TokenData>,
    
    #[account(
        init,
        payer = creator,
        mint::decimals = decimals,
        mint::authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = creator,
        token::mint = mint,
        token::authority = creator,
    )]
    pub creator_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Mint authority PDA
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct EnableTrading<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(mut, has_one = creator)]
    pub token_data: Account<'info, TokenData>,
}

#[derive(Accounts)]
pub struct DistributeRevenue<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    // Additional accounts for revenue distribution
}

#[account]
pub struct TokenData {
    pub creator: Pubkey,
    pub name: String,
    pub symbol: String,
    pub supply: u64,
    pub decimals: u8,
    pub created_at: i64,
    pub trading_enabled: bool,
    pub total_revenue_distributed: u64,
}

impl TokenData {
    pub const SIZE: usize = 32 + 4 + 50 + 4 + 10 + 8 + 1 + 8 + 1 + 8;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Trading already enabled")]
    TradingAlreadyEnabled,
    #[msg("Invalid supply amount")]
    InvalidSupply,
}
