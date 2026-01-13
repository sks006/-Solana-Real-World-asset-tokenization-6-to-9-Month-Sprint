use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount, Mint, Transfer };
use crate::error::ErrorCode as LendingError;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

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
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 8 + 1,
        seeds = [b"obligation", user.key().as_ref()],
        bump
    )]
    pub obligation: Account<'info, crate::state::UserObligation>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Allow zero-amount deposits as a no-op (tests expect this behavior).
    if amount == 0 {
        return Ok(());
    }

    // Transfer tokens from user to vault
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    });
    token::transfer(cpi_ctx, amount)?;
    // Update or initialize the obligation account. Anchor's `init_if_needed`
    // will create the account when missing, so we simply set or update fields.
    let obligation = &mut ctx.accounts.obligation;

    // If the obligation owner is the default Pubkey, treat as uninitialized
    if obligation.owner == Pubkey::default() {
        obligation.owner = ctx.accounts.user.key();
        obligation.deposited = amount;
        obligation.borrowed = 0;
        let (_pda, bump) = Pubkey::find_program_address(
            &[b"obligation", ctx.accounts.user.key().as_ref()],
            ctx.program_id
        );
        obligation.bump = bump;
    } else {
        require!(obligation.owner == ctx.accounts.user.key(), LendingError::Unauthorized);

        obligation.deposited = obligation.deposited
            .checked_add(amount)
            .ok_or(LendingError::Overflow)?;
    }

    Ok(())
}
