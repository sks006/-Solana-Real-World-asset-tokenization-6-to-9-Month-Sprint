// use solana_program::{
//     account_info::{next_account_info, AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey,
//     // Note: system_program contains the ID "11111111111111111111111111111111"
//     system_program,
// };
// entrypoint!(process_instruction);

// pub fn process_instruction(
//     program_id: &Pubkey,      // The ID of this specific program
//     accounts: &[AccountInfo],  // A slice containing all accounts for the tx
//     _instruction_data: &[u8],  // Data passed from the client (unused here)
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account= next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance = lamports as f64 / 1_000_000_000.0;
//     msg!("Target Account: {}", account_pubkey);
//     msg!("Balance: {} lamports ({:.9} SOL)", lamports, sol_balance);

//     if target_account.owner == &system_program::ID {
//        msg!("Verification: This is a standard user-owned account.");
//     }

//    Ok(())
// }

//------------------------------------------ 14 ----------------------------------

// use solana_program::{
//     account_info::{next_account_info,AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey,
//     system_program

// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// ) -> ProgramResult{
//     let account_info=&mut accounts.iter();
//     let target_account=next_account_info(account_info)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/ 1_000_000_000.0;
//     msg!("Target Account : {}",account_pubkey);
//     msg!("Balance :{} lamports ({:.9}SOL)", lamports, sol_balance);

//     if target_account.owner == &system_program::ID {
//        msg!("Verification: This is a standard user-owned account.");
//     }
//      Ok(())
// }

//------------------------------------------ 13 ----------------------------------
// use solana_program::{
//     account_info::{next_account_info,AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey,
//     system_program
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     msg!("Balance :{} lamports({:.9})SOL",lamports,sol_balance);
//     if target_account.owner == &system_program::ID{
//         msg!("Verification: this is a standard user-owned account");
//     }
//     Ok(())
// }
// ------------------------------------------ 12 ----------------------------------

// use solana_program::{
//     account_info::{next_account_info,AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey,
//     system_program
// };

// entrypoint!(process_instruction);
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// ) ->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as u64/1_000_000_000;
//     msg!("Balance :{} lamports ({:.9})SOL",lamports,sol_balance);
//     if target_account.owner==& system_program::ID{
//         msg!("varifiaction: this is  standard user-owned account")
//     }
//     Ok(())
// }

// ------------------------------------------ 11 ----------------------------------

// use solana_program::{
//     account_info::{next_account_info,AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey,
//     system_program,

// };

// entrypoint!{process_instruction}

// pub fn process_instruction(

//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]

// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     msg!("Target Account: {}",account_pubkey);
//     msg!("Balance: {} lamports ({:.9} SOL)", lamports, sol_balance);
//     if target_account.owner ==&system_program::ID{
//         msg!("Verification: This is a standard user-owned account.");
//     }
//     Ok(())
// }
// ------------------------------------------ 10 ----------------------------------

// use solana_program::{
//     account_info::{next_account_info,AccountInfo},
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance= lamports as f64 / 1_000_000_000.0;
//     msg!("Target Account: {}", account_pubkey);
//     msg!("Balance: {}lamports({:.9}SOL)",lamports,sol_balance);
//     if target_account.owner == &system_program::ID{
//                msg!("Verification: This is a standard user-owned account.");
//     }
//     Ok(())
// }

// ------------------------------------------ 9 ----------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info, AccountInfo
//     },
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey,
//     system_program
// };

// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     msg!("Target Account: {}", account_pubkey);
//     msg!("Balance: {} lamports ({:.9} SOL)", lamports, sol_balance);

//     if target_account.owner == &system_program::ID {
//        msg!("Verification: This is a standard user-owned account.");
//     }

//    Ok(())
// }

// ------------------------------------------ 8 ----------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     pubkey::Pubkey,
//     system_program
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//       let target_account= next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance= lamports as f64/1_000_000_000.0;
//     msg!("Targate Account: {}",account_pubkey);
//     msg!("Balance: {} lamports({:.9})",lamports,sol_balance);
//     if target_account.owner== &system_program::ID{
//              msg!("Verification: This is a standard user-owned account.");

//             }
//             Ok(())

// }

// -------------------------------------- 7 ----------------------------------
// use solana_program::{
//     account_info::{next_account_info,AccountInfo},
//     pubkey::Pubkey,
//     msg,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{

//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance= lamports as u64/1_000_000_000;
//     msg!("Targat Account: {}",account_pubkey);
//     msg!("Balance: {} lamports ({:.9}) SOL",lamports,sol_balance);

//     if target_account.owner == &system_program::ID {
//        msg!("Verification: This is a standard user-owned account.");
//     }
//    Ok(())
// }
// -------------------------------------- 6 ----------------------------------
// use solana_program::{
//     account_info::{next_account_info,AccountInfo},
//     msg,
//     entrypoint,
//     entrypoint::ProgramResult,
//     system_program,
//     pubkey::Pubkey
// };

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )-> ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance= lamports as u64/1_000_000_000;
//     msg!("Target Account: {}", account_pubkey);
//     msg!("Balance: {} lamports ({:.9} SOL)", lamports, sol_balance);
//     if target_account.owner== &system_program::ID{
//         msg!{
//             "verification: this is a standard user-owned account"
//         }
//     }
// Ok(())
// }
// -------------------------------------- 5 ----------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter= &mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance= lamports as f64/1_000_000_000.0;
//         msg!("Target Account: {}", account_pubkey);
//     msg!("Balance: {} lamports ({:.9} SOL)", lamports, sol_balance);
//     if target_account.owner ==&system_program::ID{
//               msg!("Verification: This is a standard user-owned account.")
//     }
//     Ok(())

// }

// #[cfg(test)]
// mod test {
//    use super::*;
//    use solana_program::clock::Epoch;
//    #[test]
//    fn test_hello_balance() {
//        let program_id = Pubkey::new_unique();
//        let key = Pubkey::new_unique();
//        let mut lamports = 2_500_000_000; // Mock 2.5 SOL
//        let mut data = vec![0; 0];
//        let owner = system_program::ID;
//        // Mocking the Solana AccountInfo struct
//        let account = AccountInfo::new(
//            &key,
//            false, // is_signer
//            true,  // is_writable
//            &mut lamports,
//            &mut data,
//            &owner,
//            false, // executable
//            Epoch::default(),
//        );
//        let accounts = vec![account];
//        // This println! will only show if you run with --nocapture
//        println!("\n--- STARTING SIMULATION ---");
//        let result = process_instruction(&program_id, &accounts, &[]);
//        assert!(result.is_ok());
//        println!("--- SIMULATION SUCCESSFUL ---\n");
//    }
// }

// -------------------------------------- 4 ----------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     msg,
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance= lamports as f64 /1_000_000_000.0;
//     msg!("Target Account: {}", account_pubkey);
//     msg!("Balance: {} lamports ({:.9} SOL)", lamports, sol_balance);
//     if target_account.owner==&system_program::ID{
//     msg!("Verification: This is a standard user-owned account.")
//     }
//     Ok(())
// }

// #[cfg(test)]
// mod tast{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance(){
//         let program_id =
//         Pubkey::new_unique();
//         let key= Pubkey::new_unique();
//         let mut lamports= 2_500_000_000;
//         let mut data= vec![0;0];
//         let owner =system_program::ID;
//         let account= AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts=vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n")
//     }
// }

// -------------------------------------- 3 ----------------------------------

// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
//     pubkey::Pubkey,
// };

// entrypoint! {
//     process_instruction
// }

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8]
// ) -> ProgramResult {
//     let accounts_iter = &mut accounts.iter();
//     let target_account = next_account_info(accounts_iter)?;
//     let account_pubkey = target_account.key;
//     let lamports = target_account.lamports();
//     let sol_balance = (lamports as f64) / 1_000_000_000.0;
//     msg!("Target Account: {}", account_pubkey);
//     msg!("Balance: {} lamports ({:.9} SOL)", lamports, sol_balance);
//     if target_account.owner == &system_program::ID {
//         msg!("Verification: This is a standard user-owned account.");
//     }
//     Ok(())
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0; 0];
//         let owner = system_program::ID;
//         let account = AccountInfo::new(
//             &key,
//             false, // is_signer
//             true, // is_writable
//             &mut lamports,
//             &mut data,
//             &owner,
//             false, // executable
//             Epoch::default()
//         );
//         let accounts = vec![account];
//         // This println! will only show if you run with --nocapture
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n");
//     }
// }
// -------------------------------------- 2 ----------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account= next_account_info(accounts_iter)?;
//     let account_pubkey=target_account.key;
//     let lamports=target_account.lamports();
//     let sol_balance=lamports as f64/1_000_000_000.0;
//     if target_account.owner==&system_program::ID{
//         msg!("Verification: this is a standard user-owned account")
//     }
//     Ok(())

// }

// #[cfg(test)]
// mod tast{
//     use super::*;
//     use solana_program::clock::Epoch;
//     #[test]
//     fn test_hello_balance(){
//         let program_id =
//         Pubkey::new_unique();
//         let key= Pubkey::new_unique();
//         let mut lamports= 2_500_000_000;
//         let mut data= vec![0;0];
//         let owner =system_program::ID;
//         let account= AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default()
//         );
//         let accounts=vec![account];
//         println!("\n--- STARTING SIMULATION ---");
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//         println!("--- SIMULATION SUCCESSFUL ---\n")
//     }
// }

// -------------------------------------- 1 ----------------------------------

use solana_program::{
    account_info::{
        next_account_info,AccountInfo
    },
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    system_program,
    pubkey::Pubkey
};

entrypoint!{process_instruction}

pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    _instruction_data:&[u8]

)->ProgramResult{
    let accounts_iter=&mut accounts.iter();
    let target_account=next_account_info(accounts_iter)?;
    let account_pubkey=target_account.key;
    let lamports= target_account.lamports();
    let sol_balance= lamports as f64/1_000_000_000.0;

    msg!("Target Account: {}", account_pubkey);
    msg!("Balance: {} lamports ({:.9} SOL)", lamports, sol_balance);

    if target_account.owner==&system_program::ID{
                msg!("Verification: This is a standard user-owned account.");
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    #[test]
    fn test_hello_balance() {
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let mut lamports = 2_500_000_000;
        let mut data = vec![0; 0];
        let owner = system_program::ID;
        let account = AccountInfo::new(
            &key,
            false, // is_signer
            true, // is_writable
            &mut lamports,
            &mut data,
            &owner,
            false, // executable
            Epoch::default()
        );
        let accounts = vec![account];
        // This println! will only show if you run with --nocapture
        println!("\n--- STARTING SIMULATION ---");
        let result = process_instruction(&program_id, &accounts, &[]);
        assert!(result.is_ok());
        println!("--- SIMULATION SUCCESSFUL ---\n");
    }
}