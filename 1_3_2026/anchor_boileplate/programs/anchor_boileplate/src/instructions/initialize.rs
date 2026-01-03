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

// ----------------------- 15 --------------
use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode; // Ensure this import is here
#[derive(Accounts)]
pub struct DepositCollateral<info>{
    #[
        account(
            init_if_needed,
            payer=user,
            space=UserVault::LEN,
            seeds=[b"vault",user.key()],
            bump
        )
    ]
    pub vault_account:Account<info,UserVault>,
    #[account(mut)]
    pub user:Signer<info>,
    pub system_program:Program<info, System>,
}
pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
    let vault=&mut ctx.accounts.vault_account;
    if vault_account== Pubkey::default(){
        vault.owner=ctx.accounts.user.key();
        vault.bump= ctx.bumps.vault_account;
    }
    vault.collateral=vault.collateral.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;
    Ok(())
}
