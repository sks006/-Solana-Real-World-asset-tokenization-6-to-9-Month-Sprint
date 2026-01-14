// use anchor_lang::prelude::*;
// use anchor_spl::token::{ self, Token, TokenAccount, Mint, Transfer };

// #[derive(Accounts)]
// pub struct Borrow<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,

//     #[account(
//         mut,
//         token::mint = mint,
//         token::authority = user,
//     )]
//     pub user_token_account: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         seeds = [b"pool_vault", mint.key().as_ref()],
//         bump,
//     )]
//     pub vault: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         constraint = obligation.owner == user.key() @ crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation: Account<'info, crate::state::UserObligation>,

//     pub mint: Account<'info, Mint>,
//     pub token_program: Program<'info, Token>,
// }

// pub fn handler(ctx:Context<Borrow>, amount:u64) -> Result<()> {
//     // 1. IMMUTABLE REFERENCE: Borrow the state from the context. 
//     // Invariant: This data was already validated by the #[account] constraints.
//     let obligation = &ctx.accounts.obligation;

//     // 2. LTV CALCULATION (80%): Use u128 for intermediate calculation.
//     // Rule: Promote to larger types to prevent overflow during multiplication before division.
//     let max_borrow = (obligation.deposited as u128)
//         .checked_mul(80)
//         .unwrap()
//         .checked_div(100)
//         .unwrap() as u64;

//     // 3. COLLATERAL GUARD: Calculate the remaining headroom in the user's position.
//     // Note: saturating_sub prevents a crash if borrowed > max_borrow (e.g., due to price drops).
//     let total_borrowable = max_borrow.saturating_sub(obligation.borrowed);

//     // 4. BUSINESS LOGIC VALIDATION: Ensure the requested amount doesn't exceed the LTV cap.
//     if amount > total_borrowable {
//         return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
//     }

//     // 5. SIGNER SEEDS: Reconstruct the seeds for the Vault PDA.
//     // Invariant: The seeds must match the derivation in the #[derive(Accounts)] struct exactly.
//     let mint_key = ctx.accounts.mint.key();
//     let seeds = &[
//         b"pool_vault", 
//         mint_key.as_ref(), 
//         &[ctx.bumps.vault] // The 'bump' ensures we hit the valid PDA off the curve.
//     ];
//     let signer = &[&seeds[..]];

//     // 6. CPI SETUP: Prepare the transfer from Protocol Vault -> User.
//     // Rule: Authority is the vault itself (a PDA), hence we need CpiContext::new_with_signer.
//     let cpi_ctx = CpiContext::new_with_signer(
//         ctx.accounts.token_program.to_account_info(),
//         Transfer {
//             from: ctx.accounts.vault.to_account_info(),
//             to: ctx.accounts.user_token_account.to_account_info(),
//             authority: ctx.accounts.vault.to_account_info(),
//         },
//         signer
//     );

//     // 7. EXTERNAL CALL: Execute the SPL Token transfer.
//     token::transfer(cpi_ctx, amount)?;

//     // 8. STATE MUTATION: Update the ledger to reflect the new debt.
//     // Invariant: This must use checked arithmetic to prevent protocol insolvency.
//     ctx.accounts.obligation.borrowed = ctx.accounts.obligation.borrowed
//         .checked_add(amount)
//         .unwrap();

//     Ok(())
// }
//--------------------------19------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
// #[derive(Accounts)]
// pub struct Borrow<'info>{
//     #[account(mut)]
//     pub user:Signer<'info>,
//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,
//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,
// }
// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//         .checked_mul(80)
//         .unwrap()
//         .checked_div(100)
//         .unwrap() as u64;
//     let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
//     if amount>total_borrowable{
//         return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
//     }
//     let mint_key=ctx.accounts.mint.key();
//     let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
//     let signer=&[&seeds[..]];
//     let cpi_ctx=CpiContext::new_with_signer(
//         ctx.accounts.token_program.to_account_info(),
//         Transfer{
//             from:ctx.accounts.vault.to_account_info(),
//             to:ctx.accounts.user_token_account.to_account_info(),
//             authority:ctx.accounts.vault.to_account_info(),
//         },
//         signer
//     );
//     token::transfer(cpi_ctx,amount)?;
//     ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
//         .checked_add(amount)
//         .unwrap();
//     Ok(())
// }
//--------------------------18------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
// #[derive(Accounts)]
// pub struct Borrow<'info>{
//     #[account(mut)]
//     pub user:Signer<info>,
//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,
//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,
// }

// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
//     let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
//     if amount>total_borrowable{
//         return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
//     }
//     let mint_key=ctx.accounts.mint.key();
//     let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
//     let signer=&[&seeds[..]];
//     let cpi_ctx=CpiContext::new_with_signer(
//         ctx.accounts.token_program.to_account_info(),
//         Transfer{
//             from:ctx.accounts.vault.to_account_info(),
//             to:ctx.accounts.user_token_account.to_account_info(),
//             authority:ctx.accounts.vault.to_account_info(),
//         },
//         signer
//     );
//     token::transfer(cpi_ctx,amount)?;
//     ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
//     .checked_add(amount)
//     .unwrap();
//     Ok(());
// }
//--------------------------17------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
// #[derive(Accounts)] 
// pub struct Borrow<info>{
//     #[account(mut)]
//     pub user:Signer<info>,
//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<info,TokenAccount>,
//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<info,TokenAccount>,
//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<info,crate::state::UserObligation>,
//     pub mint:Account<info,Mint>,
//     pub token_program:Program<info,Token>,  
// }

// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
// let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
// if amount>total_borrowable{
//     return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
// }
// let mint_key=ctx.accounts.mint.key();
// let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
// let signer=&[&seeds[..]];
// let cpi_ctx=CpiContext::new_with_signer(
//     ctx.accounts.token_program.to_account_info(),
//     Transfer{
//         from:ctx.accounts.vault.to_account_info(),
//         to:ctx.accounts.user_token_account.to_account_info(),
//         authority:ctx.accounts.vault.to_account_info(),
//     },
//     signer
// );
// token::transfer(cpi_ctx,amount)?;
// ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
// .checked_add(amount)
// .unwrap();
// Ok(())
// }
//--------------------------16------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};

// #[derive(Accounts)]
// pub struct Borrow<info>{
//     #[account(mut)]
//     pub user:Signer<'info>,

//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,

//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,

//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,
//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,
// }

// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
// let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
// if amount>total_borrowable{
//     return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
// }       
// let mint_key=ctx.accounts.mint.key();
// let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
// let signer=&[&seeds[..]];
// let cpi_ctx=CpiContext::new_with_signer(
//     ctx.accounts.token_program.to_account_info(),
//     Transfer{
//         from:ctx.accounts.vault.to_account_info(),
//         to:ctx.accounts.user_token_account.to_account_info(),
//         authority:ctx.accounts.vault.to_account_info(),
//     },
//     signer
// );
// token::transfer(cpi_ctx,amount)?;
// ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
// .checked_add(amount)
// .unwrap();
// Ok(())
// }
//--------------------------15------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
// #[derive(Accounts)]
// pub struct Borrow<'info>{
//     #[account(mut)]
//     pub user:Signer<'info>,

//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,

//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,

//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,

//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,
// }
// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
// let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
// if amount>total_borrowable{
//     return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
// }
// let mint_key=ctx.accounts.mint.key();
// let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
// let signer=&[&seeds[..]];
// let cpi_ctx=CpiContext::new_with_signer(
//     ctx.accounts.token_program.to_account_info(),
//     Transfer{
//         from:ctx.accounts.vault.to_account_info(),          
//         to:ctx.accounts.user_token_account.to_account_info(),
//         authority:ctx.accounts.vault.to_account_info(),
//     },
//     signer
// );
// token::transfer(cpi_ctx,amount)?;
// ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
// .checked_add(amount)
// .unwrap();
// Ok(())
// }
//--------------------------14------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
// #[derive(Accounts)]
// pub struct Borrow<'info>{
//     #[account(mut)]
//     pub user:Signer<'info>,
//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,
//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,

// }
// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
// let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
// if amount>total_borrowable{
//     return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
// }
// let mint_key=ctx.accounts.mint.key();
// let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
// let signer=&[&seeds[..]];
// let cpi_ctx=CpiContext::new_with_signer(
//     ctx.accounts.token_program.to_account_info(),
//     Transfer{
//         from:ctx.accounts.vault.to_account_info(),
//         to:ctx.accounts.user_token_account.to_account_info(),
//         authority:ctx.accounts.vault.to_account_info(),
//     },
//     signer
// );
// token::transfer(cpi_ctx,amount)?;
// ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
// .checked_add(amount)
// .unwrap();
// Ok(())
    
// }
//--------------------------13------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
// #[derive(Accounts)]
// pub struct Borrow<'info>{
//     #[account(mut)]
//     pub user:Signer<info>,
//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,
//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,
// }
// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
// let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
// if amount>total_borrowable{
//     return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
// }
// let mint_key=ctx.accounts.mint.key();
// let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
// let signer=&[&seeds[..]];
// let cpi_ctx=CpiContext::new_with_signer(
//     ctx.accounts.token_program.to_account_info(),
//     Transfer{
//         from:ctx.accounts.vault.to_account_info(),
//         to:ctx.accounts.user_token_account.to_account_info(),
//         authority:ctx.accounts.vault.to_account_info(),
//     },
//     signer
// );
// token::transfer(cpi_ctx,amount)?;
// ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed       
// .checked_add(amount)
// .unwrap();
// Ok(())  
// }
//--------------------------12------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
// #[derive(Accounts)]
// pub struct Borrow<'info>{
//     #[account(mut)]
//     pub user:Signer<'info>,
//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,
//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,
// }
// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
// let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
// if amount>total_borrowable{
//     return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
// }
// let mint_key=ctx.accounts.mint.key();
// let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
// let signer=&[&seeds[..]];
// let cpi_ctx=CpiContext::new_with_signer(
//     ctx.accounts.token_program.to_account_info(),
//     Transfer{
//         from:ctx.accounts.vault.to_account_info(),
//         to:ctx.accounts.user_token_account.to_account_info(),
//         authority:ctx.accounts.vault.to_account_info(),
//     },
//     signer
// );
// token::transfer(cpi_ctx,amount)?;
// ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
// .checked_add(amount)
// .unwrap();
// Ok(())
// }
//--------------------------11------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
// #[derive(Accounts)]
// pub struct Borrow<'info>{
//     #[account(mut)]
//     pub user:Signer<'info>,
//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,
//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,
// }

// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
// let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
// if amount>total_borrowable{
//     return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
// }
// let mint_key=ctx.accounts.mint.key();
// let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
// let signer=&[&seeds[..]];
// let cpi_ctx=CpiContext::new_with_signer(
//     ctx.accounts.token_program.to_account_info(),
//     Transfer{
//         from:ctx.accounts.vault.to_account_info(),
//         to:ctx.accounts.user_token_account.to_account_info(),
//         authority:ctx.accounts.vault.to_account_info(),
//     },
//     signer
// );
// token::transfer(cpi_ctx,amount)?;       
// ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
// .checked_add(amount)
// .unwrap();
// Ok(())
// }
//--------------------------10------------------------
// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};

// #[derive(Accounts)]
// pub struct Borrow<'info>{
//     #[account(mut)]
//     pub user:Signer<'info>,
//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,
//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,
// }

// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
// let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
// if amount>total_borrowable{
//     return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
// }
// let mint_key=ctx.accounts.mint.key();
// let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
// let signer=&[&seeds[..]];
// let cpi_ctx=CpiContext::new_with_signer(
//     ctx.accounts.token_program.to_account_info(),
//     Transfer{
//         from:ctx.accounts.vault.to_account_info(),
//         to:ctx.accounts.user_token_account.to_account_info(),
//         authority:ctx.accounts.vault.to_account_info(),
//     },
//     signer
// );
// token::transfer(cpi_ctx,amount)?;
// ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
// .checked_add(amount)
// .unwrap();
// Ok(())
// }
//--------------------------9------------------------

// use anchor_lang::prelude::*;
// use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
// #[derive(Accounts)]
// pub struct Borrow<info>{
//     #[account(mut)]
//     pub user:Signer<'info>,
//     #[account(
//         mut,
//         token::mint=mint,
//         token::authority=user,
//     )]
//     pub user_token_account:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         seeds=[b"pool_vault",mint.key().as_ref()],
//         bump,
//     )]
//     pub vault:Account<'info,TokenAccount>,
//     #[account(
//         mut,
//         constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
//     )]
//     pub obligation:Account<'info,crate::state::UserObligation>,
//     pub mint:Account<'info,Mint>,
//     pub token_program:Program<'info,Token>,
// }

// pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
//     let obligation=&ctx.accounts.obligation;
//     let max_borrow=(obligation.deposited as u128)
//     .checked_mul(80)
//     .unwrap()
//     .checked_div(100)
//     .unwrap() as u64;
// let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
// if amount>total_borrowable{
//     return Err(error!(crate::error::ErrorCode::InsufficientCollateral));    

// }
// let mint_key=ctx.accounts.mint.key();
// let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
// let signer=&[&seeds[..]];
// let cpi_ctx=CpiContext::new_with_signer(
//     ctx.accounts.token_program.to_account_info(),
//     Transfer{
//         from:ctx.accounts.vault.to_account_info(),
//         to:ctx.accounts.user_token_account.to_account_info(),
//         authority:ctx.accounts.vault.to_account_info(),
//     },
//     signer
// );
// token::transfer(cpi_ctx,amount)?;
// ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
// .checked_add(amount)
// .unwrap();
// Ok(())
// }
//--------------------------7------------------------
use anchor_lang::prelude::*;
use anchor_spl::token::{self,Token,TokenAccount,Mint,Transfer};
#[derive(Accounts)]
pub struct Borrow<info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        mut,
        token::mint=mint,
        token::authority=user,
    )]
    pub user_token_account:Account<'info,TokenAccount>,
    #[account(
        mut,
        seeds=[b"pool_vault",mint.key().as_ref()],
        bump,
    )]
    pub vault:Account<'info,TokenAccount>,
    #[account(
        mut,
        constraint=obligation.owner==user.key()@crate::error::ErrorCode::Unauthorized
    )]
    pub obligation:Account<'info,crate::state::UserObligation>,
    pub mint:Account<'info,Mint>,
    pub token_program:Program<'info,Token>,
}
pub fn handler(ctx:Context<Borrow>,amount:u64)->Result<()>{
    let obligation=&ctx.accounts.obligation;
    let max_borrow=(obligation.deposited as u128)
    .checked_mul(80)
    .unwrap()
    .checked_div(100)
    .unwrap() as u64;
let total_borrowable=max_borrow.saturating_sub(obligation.borrowed);
if amount>total_borrowable{
    return Err(error!(crate::error::ErrorCode::InsufficientCollateral));
}
let mint_key=ctx.accounts.mint.key();
let seeds=&[b"pool_vault",mint_key.as_ref(),&[ctx.bumps.vault]];
let signer=&[&seeds[..]];       
let cpi_ctx=CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    Transfer{
        from:ctx.accounts.vault.to_account_info(),
        to:ctx.accounts.user_token_account.to_account_info(),
        authority:ctx.accounts.vault.to_account_info(),
    },
    signer
);
token::transfer(cpi_ctx,amount)?;
ctx.accounts.obligation.borrowed=ctx.accounts.obligation.borrowed
.checked_add(amount)
.unwrap();
Ok(())
}