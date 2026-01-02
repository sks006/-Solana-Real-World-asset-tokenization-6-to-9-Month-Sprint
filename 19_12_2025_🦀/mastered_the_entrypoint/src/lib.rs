use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    // Note: system_program contains the ID "11111111111111111111111111111111"
    system_program, 
};

// =============================================================================
// 1. THE ENTRYPOINT (The "Hook")
// =============================================================================
// Architectural Reason: The SVM requires a specific symbol to start execution.
// This macro maps the raw BPF memory into the Rust types below.
entrypoint!(process_instruction);

// =============================================================================
// 2. THE SIGNATURE (The "Kata") - Practice this 30 times!
// =============================================================================
pub fn process_instruction(
    program_id: &Pubkey,      // The ID of this specific program
    accounts: &[AccountInfo],  // A slice containing all accounts for the tx
    _instruction_data: &[u8],  // Data passed from the client (unused here)
) -> ProgramResult {          // Returns Ok(()) or ProgramError

    msg!("--- [KATA REP: Hello Balance] ---");
    msg!("Program ID: {}", program_id);

    // Step 1: Create an iterator over the accounts slice
    let accounts_iter = &mut accounts.iter();

    // Step 2: Extract the first account using next_account_info
    // The '?' operator returns an error if the account slice is empty.
    let target_account = next_account_info(accounts_iter)?;

    // Step 3: Access data (Pubkey and Lamports)
    let account_pubkey = target_account.key;
    let lamports = target_account.lamports();
    
    // Convert Lamports to SOL for the log
    // Formula: 1 SOL = 1,000,000,000 Lamports
    let sol_balance = lamports as f64 / 1_000_000_000.0;

    // Step 4: Output to the program logs
    msg!("Target Account: {}", account_pubkey);
    msg!("Balance: {} lamports ({:.9} SOL)", lamports, sol_balance);

    // Step 5: Security/Verification
    // We check if the account is owned by the System Program.
    if target_account.owner == &system_program::ID {
        msg!("Verification: This is a standard user-owned account.");
    }

    Ok(())
}

// =============================================================================
// 3. THE SIMULATION (How to get output)
// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_hello_balance() {
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let mut lamports = 2_500_000_000; // Mock 2.5 SOL
        let mut data = vec![0; 0];
        let owner = system_program::ID;

        // Mocking the Solana AccountInfo struct
        let account = AccountInfo::new(
            &key,
            false, // is_signer
            true,  // is_writable
            &mut lamports,
            &mut data,
            &owner,
            false, // executable
            Epoch::default(),
        );

        let accounts = vec![account];

        // This println! will only show if you run with --nocapture
        println!("\n--- STARTING SIMULATION ---");
        let result = process_instruction(&program_id, &accounts, &[]);
        assert!(result.is_ok());
        println!("--- SIMULATION SUCCESSFUL ---\n");
    }
}