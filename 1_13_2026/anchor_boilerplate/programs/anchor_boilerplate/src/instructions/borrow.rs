use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount, Mint, Transfer };

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool_vault", mint.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = obligation.owner == user.key() @ crate::error::ErrorCode::Unauthorized
    )]
    pub obligation: Account<'info, crate::state::UserObligation>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    let obligation = &ctx.accounts.obligation;

    // Health Factor Calculation (80% collateral rule)
    let max_borrow = (obligation.deposited as u128)
        .checked_mul(80)
        .unwrap()
        .checked_div(100)
        .unwrap() as u64;

    let total_borrowable = max_borrow.saturating_sub(obligation.borrowed);

    if amount > total_borrowable {
        return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
    }

    // Store mint key to avoid temporary value
    let mint_key = ctx.accounts.mint.key();

    // PDA signing for vault transfer
    let seeds = &[b"pool_vault", mint_key.as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer
    );

    token::transfer(cpi_ctx, amount)?;

    ctx.accounts.obligation.borrowed = ctx.accounts.obligation.borrowed
        .checked_add(amount)
        .unwrap();

    Ok(())
}
