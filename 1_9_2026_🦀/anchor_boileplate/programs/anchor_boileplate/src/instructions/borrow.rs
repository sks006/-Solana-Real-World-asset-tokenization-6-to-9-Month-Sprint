use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub pool: Account<'info, LendingPool>,

    #[account(
        mut,
        seeds = [b"pool_vault", pool.key().as_ref()],
        bump // Rule: Program-owned vault
    )]
    pub pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        // WEEK 6 RULE: has_one ensures the signer IS the owner of this data.
        // seeds ensures this is a legitimate program-derived obligation.
        has_one = owner @ ErrorCode::Unauthorized,
        seeds = [b"obligation", owner.key().as_ref()],
        bump = obligation.bump,
    )]
    pub obligation: Account<'info, UserObligation>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub owner: Signer<'info>, // Rule: Must sign to prove intent
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    // 1. MATH RULE: Use checked math to prevent overflows/underflows
    let obligation = &mut ctx.accounts.obligation;
    
    // WEEK 6 SECURITY: Calculate Health Factor / LTV
    let max_borrow = obligation.deposited_amount
        .checked_mul(80).unwrap()
        .checked_div(100).unwrap();

    require!(
        obligation.borrowed_amount.checked_add(amount).unwrap() <= max_borrow,
        ErrorCode::InsufficientCollateral
    );

    // 2. WEEK 7 RULE: CPI with Signer Seeds (PDA hand-off)
    let pool_key = ctx.accounts.pool.key();
    let seeds = &[
        b"pool_vault",
        pool_key.as_ref(),
        &[ctx.bumps.pool_vault],
    ];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pool_vault.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.pool_vault.to_account_info(),
        },
        signer,
    );

    token::transfer(cpi_ctx, amount)?;

    // 3. Update State
    obligation.borrowed_amount = obligation.borrowed_amount.checked_add(amount).unwrap();
    
    Ok(())
}