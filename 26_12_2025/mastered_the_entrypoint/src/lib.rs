// use solana_program::{
//    account_info::{next_account_info, AccountInfo},
//    entrypoint,
//    entrypoint::ProgramResult,
//    msg,
//    program_error::ProgramError,
//    pubkey::Pubkey,
//    system_program,
// };

// entrypoint!(process_instruction);

// pub fn process_instruction(
//    program_id: &Pubkey,
//    accounts: &[AccountInfo],
//    _instruction_data: &[u8],
// ) -> ProgramResult {
//    let accounts_iter = &mut accounts.iter();

//    // --- RULE 1: Sequence Validation ---
//    let target_account = next_account_info(accounts_iter)?;
//    let authority = next_account_info(accounts_iter)?;

//    // --- RULE 2: Security Gate - Ownership Check ---
//    // If the account isn't owned by the System Program, it might be a fake RWA.
//    if target_account.owner != &system_program::ID {
//        msg!("Error: Target account is not owned by the System Program");
//        return Err(ProgramError::IncorrectProgramId);
//    }

//    // --- RULE 3: Security Gate - Signer Check ---
//    // Only the authorized key can trigger logic.
//    if !authority.is_signer {
//        msg!("Error: Authority must sign this transaction");
//        return Err(ProgramError::MissingRequiredSignature);
//    }

//    // --- RULE 4: Precision Rule - Checked Math (NO FLOATS) ---
//    // HFT Rule: We stay in lamports (u64) to prevent non-deterministic rounding.
//    let lamports = target_account.lamports();

//    // Instead of balance / 10^9, we use checked division for safety
//    let sol_whole_part = lamports
//        .checked_div(1_000_000_000)
//        .ok_or(ProgramError::ArithmeticOverflow)?;

//    msg!("Simulation: Verified balance of {} SOL", sol_whole_part);

//    Ok(())
// }

// #[cfg(test)]
// mod test {
//    use super::*;
//    use solana_program::clock::Epoch;

//    #[test]
//    fn test_security_gates() {
//        let program_id = Pubkey::new_unique();

//        // --- Setup Target Account ---
//        let target_key = Pubkey::new_unique();
//        let mut target_lamports = 2_500_000_000; // 2.5 SOL
//        let mut target_data = vec![0; 0];

//        let target_info = AccountInfo::new(
//            &target_key,
//            false,
//            true,
//            &mut target_lamports,
//            &mut target_data,
//            &system_program::ID, // Matches Rule 2
//            false,
//            Epoch::default(),
//        );

//        // --- Setup Authority (Signer) ---
//        let auth_key = Pubkey::new_unique();
//        let mut auth_lamports = 0;
//        let mut auth_data = vec![0; 0];

//        let auth_info = AccountInfo::new(
//            &auth_key,
//            true, // Matches Rule 3 (is_signer = true)
//            false,
//            &mut auth_lamports,
//            &mut auth_data,
//            &system_program::ID,
//            false,
//            Epoch::default(),
//        );

//        let accounts = vec![target_info, auth_info];

//        // Execute
//        let result = process_instruction(&program_id, &accounts, &[]);
//        assert!(result.is_ok());
//    }
// }

//----------------------------------- 10 -----------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
//     system_program,
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     //rules:1 Sequence validation
//     let target_account=next_account_info(accounts_iter)?;
//     let authority=next_account_info(accounts_iter)?;

//     //rules:2 Sequrity gate- Ownership Check --------
//     // If the account isn't owned by the System Program, it might be a fake RWA.

//     if target_account.owner!=&system_program::ID{
//         msg!("Error: Target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId);
//     }

//     //rules:3 sequrity gate- Signer Check ---
//      // Only the authorized key can trigger logic.
//      if !authority.is_signer{
//         msg!{"Error :authority must sign this transaction"};
//         return Err(ProgramError::MissingRequiredSignature)
//      }

//     //rules:4 Precision Rule Checked Math
//      // HFT Rule: We stay in lamports (u64) to prevent non-deterministic rounding.

//      let lamports=target_account.lamports();
//      let sol_whole_part=lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;
//      msg!("Simulation: veryFied balance of {} SoL",sol_whole_part);

//     // Instead of balance / 10^9,we use checked division for safety
//     Ok(())
// }
//-------------------------------- 9 ---------------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
//     system_program
// };

// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     // rule 1: Sequence validation ---
//     let target_account=next_account_info(accounts_iter)?;
//     let authority=next_account_info(accounts_iter)?;

//     // rule 2: Sequrity gate ownership check
//     if target_account.owner!=&system_program::ID{
//         msg!{"Error: Target account is not owned by the system Programming"};
//         return Err(ProgramError::IncorrectProgramId);
//     }

//     // rule 3: Security Gate Signer Check--
//     if !authority.is_signer{
//         msg!{"Error : Authority must sign this transaction"};
//         return Err(ProgramError::MissingRequiredSignature)
//     }

//     // rule 4: Precision Rule - checked math
//      // HFT Rule: We stay in lamports (u64) to prevent non-deterministic rounding.
//      let lamports=target_account.lamports();
//      let sol_whole_part=lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;
//         msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
//    Ok(())
// }
//-------------------------------- 8 ---------------------------------------

// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//      // --- RULE 1: Sequence Validation ---
//     let target_account=next_account_info(accounts_iter)?;
//     let authority= next_account_info(accounts_iter)?;
//      // --- RULE 2: Security Gate - Ownership Check ---
//     if target_account.owner!=&system_program::ID{
//         msg!("Error: Target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId);
//     }

//      //RULE 3: Security gate signer check ---------
//      if !authority.is_signer{
//         msg!("Error: Authority must sign this transaction");
//         return Err(ProgramError::MissingRequiredSignature)

//      }

//      // RULE 4:Precision Rule -Checked Math (no floats) ---
//      let lamports= target_account.lamports();
//      let sol_whole_part=lamports.checked_div(1_000_000_000).ok_or(
//         ProgramError::ArithmeticOverflow
//      )?;
//        msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
//        Ok(())
// }
//-------------------------------- 7 ---------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//       // --- RULE 1: Sequence Validation ---
//       let target_account=next_account_info(accounts_iter)?;
//       let authority=next_account_info(accounts_iter)?;
//       // --- RULE 2: Security Gate - Ownership Check ---
//         if target_account.owner != &system_program::ID{
//             msg!("Error: Target account is not owned by the System Program");
//             return Err(ProgramError::IncorrectProgramId);
//             }

//       // --- RULE 3: Security Gate - Signer Check ---
//             if !authority.is_signer{
//                 msg!("Error: Authority must sign this transaction");
//                 return Err(ProgramError::MissingRequiredSignature)
//             }

//      // --- RULE 4: Precision Rule - Checked Math (NO FLOATS) ---
//      let lamports= target_account.lamports();
//      let sol_whole_part=lamports.checked_div(1_000_000_000).ok_or(
//         ProgramError::ArithmeticOverflow
//      )?;
//     msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
//     Ok(())
// }
//-------------------------------- 6 ---------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     //rule 1: sequence Validation
//     let target_account=next_account_info(accounts_iter)?;
//     let authority=next_account_info(accounts_iter)?;
//     //rule 2: Security gate Ownership check ---
//     if target_account.owner!= &system_program::ID{
//         msg!("Error: Target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId);
//     }
//     //rule 3: Security Gate - Signer Check ---
//     if !authority.is_signer{
//          msg!("Error: Authority must sign this transaction");
//          return Err(ProgramError::MissingRequiredSignature);
//     }
//     //rule 4: Precision ruke check math --
//     let lamports= target_account.lamports();
//     let sol_whole_part=lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;
//        msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
//    Ok(())
// }
//-------------------------------- 5 ---------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg,
//     program_error::ProgramError,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//       // --- RULE 1: Sequence Validation ---
//       let target_account=next_account_info(accounts_iter)?;
//       let authority=next_account_info(accounts_iter)?;
//     // --- RULE 2: Security Gate - Ownership Check ---
//     if target_account.owner!= &system_program::ID{
//         msg!("Error: Target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId)
//     }

//     // --- RULE 3: Security Gate - Signer Check ---
//     if !authority.is_signer{
//         msg!("Error: Authority must sign this transaction");
//         return Err(ProgramError::MissingRequiredSignature);
//}

//     // --- RULE 4: Precision Rule - Checked Math (NO FLOATS) ---
//     let lamports=target_account.lamports();
//     let sol_whole_part=lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;
//     Ok(())

// }
//-------------------------------- 4 ---------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     program_error::ProgramError,
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
//     // rule 1: sequence validation
//     let target_account= next_account_info(accounts_iter)?;
//     let authority=next_account_info(accounts_iter)?;

//     // rule 2: sequrity gate Ownership
//     if target_account.owner!=&system_program::ID{
//         msg!("Error: Target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId);
//     }
//     // rule 3: secqurity gate - signer check
//     if !authority.is_signer{
//         msg!("Err authority must sign the transaction");
//         return Err(ProgramError::MissingRequiredSignature);
//     }
//     // rule 4: precicion check math
//     let lamports=target_account.lamports();
//     let sol_whole_part= lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;
//            msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
//    Ok(())
// }
//----------------------------- 3 ---------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info, AccountInfo
//     },
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg,
//     program_error::ProgramError,
//     system_program
// };
// entrypoint!(process_instruction);

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8]
// ) -> ProgramResult {
//     let accounts_iter = &mut accounts.iter();

//     // Rule 1: Sequence Validation
//     let target_account = next_account_info(accounts_iter)?;
//     let authorized = next_account_info(accounts_iter)?;

//     // Rule 2: Security gate Ownership check
//     if target_account.owner != &system_program::ID {
//         msg!("Error: target account is not owned by the system program");
//         return Err(ProgramError::IncorrectProgramId);
//     }

//     // Rule 3: Security Gate - Signer Check
//     // FIXED: Changed `authority` to `authorized` (variable name mismatch)
//     if !authorized.is_signer {
//         msg!("Error: Authority must sign this transaction");
//         // FIXED: Changed `ProgramResult::MissingRequiredSignature` to correct error
//         return Err(ProgramError::MissingRequiredSignature);
//     }

//     // Rule 4: Precision rule check math
//     // FIXED: Added `lamports` variable (was missing)
//     // Note: You need to get lamports from somewhere - typically from target_account
//     let lamports = target_account.lamports();
//     let sol_whole_part = lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;

//     msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
//     Ok(())
// }
//----------------------------- 2 ---------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
//     program_error::ProgramError
// };
// entrypoint!{process_instruction}
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     // --- RULE 1: Sequence Validation ---
//     let target_account=next_account_info(accounts_iter)?;
//     let authority=next_account_info(accounts_iter)?;

//     // --- RULE 2: Security Gate - Ownership Check ---
//     if target_account.owner !=&system_program::ID{
//         msg!("Error: Target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId)
//     }

//     // --- RULE 3: Security Gate - Signer Check ---
//     if !authority.is_signer{
//         msg!("Error: Authority must sign this transaction");
//         return Err(ProgramError::MissingRequiredSignature);
//     }
//     // --- RULE 4: Precision Rule - Checked Math (NO FLOATS) ---
//     let lamports=target_account.lamports();
//     let sol_whole_part=lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;
//     Ok(())
// }
//----------------------------- 1 ---------------------------------------
// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     system_program,
//     program_error::ProgramError,
// };
// entrypoint! {process_instruction}

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8]
// ) -> ProgramResult {
//     let accounts_iter = &mut accounts.iter();
//     // --- RULE 1: Sequence Validation ---
//     let target_account = next_account_info(accounts_iter)?;
//     let authority = next_account_info(accounts_iter)?;
//     // --- RULE 2: Security Gate - Ownership Check ---
//     if target_account.owner != &system_program::ID {
//         msg!("Error: Target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId);
//     }
//     // --- RULE 3: Security Gate - Signer Check ---
//     if !authority.is_signer {
//         msg!("Error: Authority must sign this transaction");
//         return Err(ProgramError::MissingRequiredSignature);
//     }

//     // --- RULE 4: Precision Rule - Checked Math (NO FLOATS) ---

//     let lamports = target_account.lamports();
//     let sol_whole_part = lamports
//         .checked_div(1_000_000_000)
//         .ok_or(ProgramError::ArithmeticOverflow)?;
//     Ok(())
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;

//     #[test]
//     fn test_security_gates() {
//         let program_id = Pubkey::new_unique();

//         // --- Setup Target Account ---
//         let target_key = Pubkey::new_unique();
//         let mut target_lamports = 2_500_000_000; // 2.5 SOL
//         let mut target_data = vec![0; 0];

//         let target_info = AccountInfo::new(
//             &target_key,
//             false,
//             true,
//             &mut target_lamports,
//             &mut target_data,
//             &system_program::ID, // Matches Rule 2
//             false,
//             Epoch::default()
//         );

//         // --- Setup Authority (Signer) ---
//         let auth_key = Pubkey::new_unique();
//         let mut auth_lamports = 0;
//         let mut auth_data = vec![0; 0];

//         let auth_info = AccountInfo::new(
//             &auth_key,
//             true, // Matches Rule 3 (is_signer = true)
//             false,
//             &mut auth_lamports,
//             &mut auth_data,
//             &system_program::ID,
//             false,
//             Epoch::default()
//         );

//         let accounts = vec![target_info, auth_info];

//         // Execute
//         let result = process_instruction(&program_id, &accounts, &[]);
//         assert!(result.is_ok());
//     }
// }
//----------------------------- 5 ---------------------------------------

// use solana_program::{
//     account_info::{ next_account_info, AccountInfo },
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg,
//     program_error::ProgramError,
//     system_program,
// };
// entrypoint! {process_instruction}

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8]
// ) -> ProgramResult {
//     let accounts_iter = &mut accounts.iter();
//     // --- RULE 1: Sequence Validation ---
//     let target_account = next_account_info(accounts_iter)?;
//     let authority = next_account_info(accounts_iter)?;

//     // --- RULE 2: Security Gate - Ownership Check ---
//     if target_account.owner != &system_program::ID {
//         msg!("Error: target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId);
//     }
//     // --- RULE 3: Security Gate - Signer Check ---
//     if !authority.is_signer {
//         msg!("Error: Authority must sign this transaction");
//         return Err(ProgramError::MissingRequiredSignature);
//     }
//     // --- RULE 4: Precision Rule - Checked Math (NO FLOATS) ---
//     let lamports = target_account.lamports();
//     let sol_whole_part = lamports
//         .checked_div(1_000_000_000)
//         .ok_or(ProgramError::ArithmeticOverflow)?;
//     Ok(())
// }

//----------------------------- 4 ---------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
//     system_program
// };
// entrypoint!{process_instruction}

// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     //sequence validation 
//     let target_account=next_account_info(accounts_iter)?;
//     let authority=next_account_info(accounts_iter)?;
//     //sequrity gate Ownership 
//     if target_account.owner !=&system_program::ID{
//                 msg!("Error: Target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId);
//     }
    
//     //sequrity gate - signer check
//         if !authority.is_signer{
//         msg!("Err authority must sign the transaction");
//         return Err(ProgramError::MissingRequiredSignature);
//     }
//     //precicion check math 
//         let lamports=target_account.lamports();
//     let sol_whole_part= lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;
//            msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
//    Ok(())
// }
//----------------------------- 3 ---------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info,AccountInfo
//     },
//     pubkey::Pubkey,
//     entrypoint,
//     entrypoint::ProgramResult,
//     msg,
//     program_error::ProgramError,
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
//     let authority=next_account_info(accounts_iter)?;

//     if target_account.owner!=&system_program::ID{
//         msg!("Error: Target account is not owned by the System Program");
//         return Err(ProgramError::IncorrectProgramId);
//     }
//         if !authority.is_signer{
//         msg!("Err authority must sign the transaction");
//         return Err(ProgramError::MissingRequiredSignature);
//     };
//     let lamports=target_account.lamports();
//         let sol_whole_part= lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;
//            msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
//    Ok(())
// }
//----------------------------- 2 ---------------------------------------
// use solana_program::{
//     account_info::{
//         next_account_info, AccountInfo
//     },
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg,
//     program_error::ProgramError,
//     system_program
// };
// entrypoint!(process_instruction);
// pub fn process_instruction(
//     program_id:&Pubkey,
//     accounts:&[AccountInfo],
//     _instruction_data:&[u8]
// )->ProgramResult{
//     let accounts_iter=&mut accounts.iter();
//     let target_account=next_account_info(accounts_iter)?;
//     let authority=next_account_info(accounts_iter)?;
//     if target_account.owner!=&system_program::ID{
//         msg!("Error: target account is not owned by the system program");
//          return Err(ProgramError::IncorrectProgramId);

//     }
//     if !authority.is_signer {
//         msg!("Error: Authority must sign this transaction");
//         // FIXED: Changed `ProgramResult::MissingRequiredSignature` to correct error
//         return Err(ProgramError::MissingRequiredSignature);
//     }
//         let lamports = target_account.lamports();
//     let sol_whole_part = lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;

//     msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
//     Ok(())
// }

//----------------------------- 1 ---------------------------------------
use solana_program::{
    account_info::{
        next_account_info,AccountInfo
    },
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    system_program,
    program_error::ProgramError
};

entrypoint!{process_instruction}

pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    _instruction_data:&[u8]
)->ProgramResult{
     let accounts_iter = &mut accounts.iter();

    // Rule 1: Sequence Validation
    let target_account = next_account_info(accounts_iter)?;
    let authorized = next_account_info(accounts_iter)?;

    // Rule 2: Security gate Ownership check
    if target_account.owner != &system_program::ID {
        msg!("Error: target account is not owned by the system program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Rule 3: Security Gate - Signer Check
    // FIXED: Changed `authority` to `authorized` (variable name mismatch)
    if !authorized.is_signer {
        msg!("Error: Authority must sign this transaction");
        // FIXED: Changed `ProgramResult::MissingRequiredSignature` to correct error
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Rule 4: Precision rule check math
    // FIXED: Added `lamports` variable (was missing)
    // Note: You need to get lamports from somewhere - typically from target_account
    let lamports = target_account.lamports();
    let sol_whole_part = lamports.checked_div(1_000_000_000).ok_or(ProgramError::ArithmeticOverflow)?;

    msg!("Simulation: Verified balance of {} SOL", sol_whole_part);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_security_gates() {
        let program_id = Pubkey::new_unique();

        // --- Setup Target Account ---
        let target_key = Pubkey::new_unique();
        let mut target_lamports = 2_500_000_000; // 2.5 SOL
        let mut target_data = vec![0; 0];

        let target_info = AccountInfo::new(
            &target_key,
            false,
            true,
            &mut target_lamports,
            &mut target_data,
            &system_program::ID, // Matches Rule 2
            false,
            Epoch::default()
        );

        // --- Setup Authority (Signer) ---
        let auth_key = Pubkey::new_unique();
        let mut auth_lamports = 0;
        let mut auth_data = vec![0; 0];

        let auth_info = AccountInfo::new(
            &auth_key,
            true, // Matches Rule 3 (is_signer = true)
            false,
            &mut auth_lamports,
            &mut auth_data,
            &system_program::ID,
            false,
            Epoch::default()
        );

        let accounts = vec![target_info, auth_info];

        // Execute
        let result = process_instruction(&program_id, &accounts, &[]);
        assert!(result.is_ok());
    }
}

















