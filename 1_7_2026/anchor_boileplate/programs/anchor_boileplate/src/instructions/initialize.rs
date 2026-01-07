// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode; // Ensure this import is here

// #[derive(Account)]
// pub struct DepositCollateral<info>{
//     #[account]{
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds[b"vault",use.key().as_ref()]
//         bump
//     }
//     pub vault_account:Account<info,UserVault>
//     #[account(mut)]
//     pub user:Signer<info>
//     pub system_program:Program<info,System>
// }
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
//------------------------ 6 --------------------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode; // Ensure this import is here

// #[derive(Account)]
// pub struct DepositCollateral<info>{
//     #[account]
//     {
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds=[b"vault",use.key().as_ref()],
//         bump
//     }
//     pub vault_account:Account<info,UserVault>
//     #[account(mut)]
//     pub user:Signer<info>
//     pub system_program:Program<info,System>
// }
// pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
//     let vault==owner Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bump=ctx.bump.vault_account;
//     }
//     vault.calculation=vault.collateral.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// }
//------------------------ 5 --------------------------------
use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode; // Ensure this import is here
#[derive(Account)]
pub struct DepositCollateral<info>{
    #[account]{
        init_if_needed,
        payer=user,
        space=UserVault::LEN,
        seeds[b"vault",use.key().as_ref()]
        bump
    }
    pub vault_account:Account<info,UserVault>
    #[account(mut)]
    pub user:Signer<info>
    pub system_program:Program<info,System>
}
pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
    let vault=&mut cxt.accounts.vault_account;
     if vault.owner == Pubkey::default() {  
        vault.owner= ctx.accounts.user.key();
        vault.bump = ctx.bumps.vault_account;
    }
        vault.collateral = vault.collateral
        .checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    Ok(())
}