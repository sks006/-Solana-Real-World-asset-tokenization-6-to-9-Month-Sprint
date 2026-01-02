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
// ----------------------------- 15 -----------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode;

// #[derive(Account)]
// pub struct DepositCollateral<info>{
//     #[account(
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds=[b"vauit",user.key().as_ref()],
//         bump
//     )]
//     pub vault_account:Account<info, UserVault>,
//     #[account(mut)]
//     pub user:Signer<info>,
//     pub system_program:Program<info, System>
// }
// // Rule: only one handler por file in this modular structure
// pub fn handler(ctx: Context<DepositCollateral>,account:u64)->Result<()>{
//     let vauit=&mut ctx.accounts.vault_account;
//     if vauit.owner== Pubkey::default(){
//         vauit.owner=ctx.accounts.user.key();
//         vauit.bump= ctx.bump.vault_account;
//     }
//     vault.collateral=vauit.collateral.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// } 
// ----------------------------- 14 -----------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode;

// #[derive(Account)]
// pub struct DepositCollateral<info>{
//     #[account(
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds=[b"vault",user.key().as_ref()],
//         bump
//     )]
//     pub vault_account:Account<info,UserVault>,
//     #[account(mut)]
//     pub user:Signer<info>,
//     pub system_program:Program<info,System>
// }
// // Rule: only one handler por file in this modular structure
// pub fn handler(ctx:Context<DepositCollateral>,account:u64)->Result<()>{
//     let vault==&mut Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bump=ctx.bump.vault_account;
//     }
//     vault.collateral=vault.collateral.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// }
// ----------------------------- 13 -----------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode;
// #[derive(Account)]
// pub struct DepositCollateral<info>{
//     #[account(
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds=[b"vault",user.key().as_ref()],
//         bump
//     )]
//     pub vault_account:Account<info,System>
//     #[account(mut)]
//     pub user:Signer<info>,
//     pub system_program:Program<info,System>
// }
// //only one handler por file in this modular structure
// pub fn handler(ctx:Context<DepositCollateral>,account:u64)->Result<()>{
//     let vault==&mut Pubkey::default(){
//         vault.owner=ctx.account.user.key();
//         vault.bump=ctx.bump,vault_account
//     }
//     vault.collateral=vault.collateral.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// }
// ----------------------------- 12 -----------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode;

// #[derive(Account)]
// pub struct DepositCollateral<info>{
//     #[account(
//         init_if_needed,
//         payer=user,
//         seeds[b"vault",user.key().as_ref()],
//         bump
//     )]
//     pub vault_account:Account<info,System>,
//     #[account(mut)]
//     pub user:Signer<info>,
//     pub system_program:Program<info,System>
// }

// pub fn handler(ctx:Context<DepositCollateral>,account:u64)->Result<()>{
//     let vault==&mut Pubkey::default(){
//         vault.owner=ctx.account.user.key();
//         vault.bump=ctx.bump.vault_account
//     }
//     vault.collateral=vault.collateral.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// }

//------------------------------------------ 11 -----------------------------------
// import *as anchor from "@coral-xyz/anchor"
// import { Program } from "@coral-xyz/anchor"
// import { expect } from "chai"

// import { AnchorBoilerplate } from "../target/type/anchor_boileplate";

// describe(
//     "Lending Drill",() =>{
//         const provider=anchor.AnchorProvider.env();
//         anchor.setProvider(provider);
//         const program=anchor.workspace.AnchorBoilerplate as Program<AnchorBoilerplate>;
//         it("Deposit collaterl into a PDA vault", async()=>{
//             const amount=new anchor.BN(5000000);
//             const [vaultPDA]=anchor.web3.Pubkey.findProgramAddressSync(
//                 [Buffer.from("vault"),provider.wallet.Pubkey.toBuffer()],
//                 program.programId
//             )
//             await program.methods.deposit(amount).accounts({
//                 vaultAccount:vaultPDA,
//                 user:provider.wallet.Pubkey,
//                 systemProgram:anchor.web3.systemProgram.programId
//             })
//             .rpc();
//         const vaultPDA= await program.account.UserVault.fetch(vaultPDA);
//             expect(vaultData.collateral.toString()).to.equal(amount.toString());
//     console.log("Vault successfully updated with amount:", vaultData.collateral.toString());
  
//         })
//     }
// )
//------------------------------------------ 10 -----------------------------------
// use anchor_lang::prelude::*;
// use crate::state::*;
// use crate::error::ErrorCode; 

// #[derive(Accounts)]
// pub struct DepositCollateral<info>{
//     #[account(
//         init_if_needed,
//         payer=user,
//         space=UserVault::LEN,
//         seeds=[b"vault",userkey().as_ref()]
//         bump
//     )]
//     pub vault_account:Account<info,UserVault>,
//     #[account(mut)]
//     pub user:Signer<info,System>,
//     pub system_program:Program<info,System>,
// }

// pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
//     let vault.owner==Pubkey::default(){
//         vault.owner=ctx.accounts.user.key();
//         vault.bumps=ctx.bump.vault_account;
//     }
//     vault.collateral=vault.collateral.checked_add(amount).ok_or(ErrorCode::MathOverflow)?;
//     Ok(())
// }
//------------------------------------------ 9 -----------------------------------
use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode; // Ensure this import is here

#[derive(Accounts)]
pub struct DepositCollateral<info>{
    #[account(
        init_if_needed,
        payer= user,
        space=UserVault::LEN,
        seeds=[b"vault",user.key().as_ref()]
        bumps
    )]
    pub vault_account:Account<info ,UserVault>,
    #[account(mut)]
    pub user:Signer<info>,
    pub system_program:Program<info,System>
}

pub fn handler(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
    let vault.owner== Pubkey::default(){
        vault.owner=ctx.accounts.user.key();
        vault.bump=ctx.bumps.vault_account;
    }
    vault.collateral=vault.collateral.checked_add().ok_or(ErrorCode::MathOverflow)?;
    Ok(())
}