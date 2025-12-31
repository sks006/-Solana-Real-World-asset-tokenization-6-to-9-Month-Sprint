// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode; // Ensure this import is here

// #[derive(Accounts)] // Rule: This macro generates the "Bumps" and "try_accounts" logic
// pub struct DepositCollateral<'info> {
//     #[account(
//         init_if_needed, 
//         payer = user, 
//         space = UserVault::LEN, 
//         seeds = [b"vault", user.key().as_ref()], 
//         bump
//     )]
//     pub vault_account: Account<'info, UserVault>,
    
//     #[account(mut)]
//     pub user: Signer<'info>,
    
//     pub system_program: Program<'info, System>,
// }

// // RULE: Only ONE handler per file in this modular structure
// pub fn handler(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
//     let vault = &mut ctx.accounts.vault_account;
    
//     if vault.owner == Pubkey::default() {
//         vault.owner = ctx.accounts.user.key();
//         vault.bump = ctx.bumps.vault_account;
//     }

//     // Precision Rule: Preventing HFT exploits
//     vault.collateral = vault.collateral
//         .checked_add(amount)
//         .ok_or(ErrorCode::MathOverflow)?;

//     Ok(())
// }
//------------------------19------------------------------------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode; // Ensure this import is here


// /*1. The Security Gate (The DepositCollateral Struct)
// Imagine a user walking up to a bank. Before they can touch any gold, they must pass through the Security Gate */

// #[derive(Accounts)] // Rule: This macro generates the "Bumps" and "try_accounts" logic
// pub struct DepositCollateral<info>{
//     /* 2. The PDA (The vault_account) 
//     Real-Life Example: The Safety Deposit Box. > * seeds: This is the Box Number. It’s not random. It's calculated based on the user's ID. This ensures the user can only ever find their specific box.

// init_if_needed: If the user doesn't have a box yet, the bank builds one for them right now.

// payer = user: Building a box isn't free. The user pays the "Construction Fee" (Rent/Lamports).

// space: The box is built to a specific size. You can't put a car in a jewelry box.
//     */
//     #[account(
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds=[b"vault",user.key().as_ref()],
//         bump
//     )]
//     pub vault_account:Account<info,UserVault>,
//     /* 3. The User (The user Signer)
//     Real-Life Example: The Biometric Scan. The Signer is the user standing there with their ID card and thumbprint. If the signature doesn't match the key, the Night Guard (Solana Runtime) stops the transaction.
//     */
//     #[account(mut)]
//     pub user:Signer<info>,
//     /*4. The System Program
//     Real-Life Example: The Construction Worker. If a new box needs to be built (init_if_needed), the program needs a worker who knows how to handle the bank's concrete. The System Program is the only one authorized to move "Rent" and create new accounts.
//     */
//     pub system_program:Program<info,System>,
// }

// /*
// 5. The Handler (The "Action")
// Real-Life Example: The Transaction Log. The Context is the "Approved File" the guard hands to the Banker. It contains the Box, the User, and the Worker.
//  */

// pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
//     let vault=&mut ctx.accounts.vault_account;
//     /*
//     Step A: Identity Burning
//     Real-Life Example: Engraving the Name. If the box is new, the first thing the banker does is engrave the user's name on it. From now on, this box belongs to them forever.
//     */

//     if vault.owner==Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bump=ctx.bumps.vault_account;
//     }
//     /**
//      * Step B: The Precision Rule (HFT Safety)
//      * Real-Life Example: The High-Speed Counter. In HFT, we move money so fast that we must use a digital counter that refuses to roll over. If the user tries to deposit so much that the box would explode (Overflow), the counter freezes and sounds an alarm (MathOverflow).
//      */
//     vault.collateral=vault.collateral
//         .checked_add(amount)
//         .ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// }
// ------------------------18------------------------------------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode; // Ensure this import is here
// #[derive(Accounts)] // Rule: This macro generates the "Bumps" and "try_accounts" logic
// pub struct DepositCollateral<info>{
//     #[account](
//         init_if_needed,
//         payer=uset,
//         space=UserVault::LEN,
//         seeds=[b"vault",user.key().as_ref()]
//     )
//     pub vault_account:Account<info,UserVault>,
//     #[account(mut)]
//     pub user:Signer<info>,
//     pub system_program:Program<info,System>,
    
// }
// pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
//     let vault=&mut ctx.accounts.vault_account;
//     if vault.owner==Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bump=ctx.bumps.vault_account;
//     }
//     vault.collateral=vault.collateral
//         .checked_add(amount)
//         .ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// }
//------------------------17------------------------------------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode; // Ensure this import is here
// #[derive(Accounts)] // Rule: This macro generates the "Bumps" and "try
// pub struct DepositCollateral<info>{
//     #[account](
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds=[b"vault",user.key().as_ref(),
//         ],
//         bump
//         )
//         pub vault_account:Account<info,UserVault>,
//         #[account(mut)]
//         pub user:Signer<info>,
//         pub system_program:Program<info,System>,

// }

// pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
//     let vault=&mut ctx.accounts.vault_account;
//     if vault.owner==Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bump=ctx.bumps.vault_account;
//     }
//     vault.collateral=vault.collateral
//         .checked_add(amount)
//         .ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// }
//------------------------16------------------------------------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode; // Ensure this import is here
// #[derive(Accounts)] // Rule: This macro generates the "Bumps" and "try_accounts" logic
// pub struct DepositCollateral<'info>{
//     #[account](
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds=[b"vault",user.key().as_ref()],
//         bump
//     )
//     pub vault_account:Account<info,UserVault>,
//     #[account(mut)]
//     pub user:Signer<info>
//     pub system_program:Program<info,System>,
// }
// pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
//     let vault=&mut ctx.accounts.vault_account;
//     if vault.owner==Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bump=ctx.bumps.vault_account;
//     }
//     vault.collateral=vault.collateral.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// }
//------------------------15------------------------------------------------

// #[derive(Accounts)]
// pub struct DepositCollateral<info>{
//     #[account](
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds=[b"vault",user.key().as_ref()],
//         bump
//     )
//     pub vault_account:Account<info,UserVault>,
//     #[account(mut)]
//     pub user:Signer<info>,
//     pub system_program:Program<info,System>,
// }
// pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
//     let vault=&mut ctx.accounts.vault_account;
//     if vault.owner==Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bump=ctx.bumps.vault_account;
//     }
//     vault.collateral=vault.collateral.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;
//     Ok(());
// }
//---------------------------------------- 14 -------------------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode;
// #[derive(Accounts)]
// pub struct DepositCollateral<info>{
//     #[account](
//         init_if_needed,
//         payer=user,
//         seeds=[b"vault",user.key().as_ref()],
//         bump
//     )
//     pub vault_account:Account<info,UserVault>,
//     #[account(mut)]
//     pub user:Signer<info>,
//     pub system_program:Program<info,System>
// }
// pub fn handler(ctx:Context<DepositCollateral>,account:u64)->Result<()>{
//     let vault=&mut ctx.accounts.vault_account;
//     if vault.owner==Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bump=ctx.bumps.vault_account;
//    }
//    vault.collateral=vault.collateral.checked_add(account).ok_or(ErrorCode::MathOverflow)?;
//    Ok(())
// }
// ------------------------------------------ 13 ---------------------------------- 
// use anchor_lang::prelude::*;
// use crate::error::ErrorCode;
// #[derive(Accounts)]
// pub struct DepositCollateral<info>{
//     #[account](
//         init_if_needed,
//         payer=user,
//         seeds=[b"vault",user.key().as_ref()],
//         bump
//     )
//     pub vault_account:Account<info,UserVault>,
//     #[account(mut)]
//     pub user:system_program:program<info,System>
// }
// pub fn handle(ctx:Context<DepositCollateral>,account:u64)->Result<()>{
//     let vault=&mut ctx.accounts.vault_account;
//     if vault.owner==Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bump=ctx.bumps.vault_account;
//     }
//     vault.collateral=vault.collateral.checked_add(account).ok_or(ErrorCode::MathOverflow)?;
//     ok(())
// }
// ------------------------------------------ 12 ----------------------------------

use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::error::ErrorCode;
#[derive(Account)]
pub struct DepositCollateral<info>{
    #[account](
        init_if_needed,
        payer=user,
        seeds=[b"vault",user.key().as_ref()],
        bump
    )
    
       pub vault_account:Account<info,UserVault>,
    #[account(mut)]
    pub user:system_program:program<info,System>
}
pub fn handle(ctx:Context<DepositCollateral>,account:u64)->Result<()>{
    let vault=&mut ctx.accounts.vault_account;
    if vault.owner==Pubkey::default(){
        vault.owner=ctx.accounts.user.key();
        vault.bump=ctx.bumps.vault_account;
    }
    vault.collateral=vault.collateral.checked_add(account).ok_or(ErrorCode::MathOverflow)?;
    ok(())
}