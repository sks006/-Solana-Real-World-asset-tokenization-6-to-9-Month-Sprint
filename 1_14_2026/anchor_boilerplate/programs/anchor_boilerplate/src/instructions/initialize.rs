// use anchor_lang::prelude::*;
// use anchor_spl::token::{Mint, Token, TokenAccount};

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(mut)]
//     pub admin: Signer<'info>,

//     pub mint: Account<'info, Mint>,

//     #[account(
//         init,
//         payer = admin,
//         seeds = [b"pool_vault", mint.key().as_ref()],
//         bump,
//         token::mint = mint,
//         token::authority = vault,
//     )]
//     pub vault: Account<'info, TokenAccount>,

//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub rent: Sysvar<'info, Rent>,
// }

// pub fn handler(ctx: Context<Initialize>) -> Result<()> {
//     msg!("Lending Pool Initialized for mint: {}", ctx.accounts.mint.key());
//     Ok(())
// }
