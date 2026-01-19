pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7XQHfwaJaBMtyfHS4zrWdN4Xjooo3M5ateNGLu8jAWDH");

#[program]
pub mod account_type {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}



// #[account(signer)]
/*
The signer Constraint:
This attribute ensures the is_signer field of the account is set to true. It is more flexible than the Signer type because it can be combined with other constraints and type checks, such as mut or has_one
*/
#[account(signer)]
#[account(signer @ <custom_error>)]

// #[account(mut)]
/*
Key Functions
Permission to Modify:
     It signals to the Solana runtime that the instruction intends to modify the account's data, balance (lamports), or other fields.

State Persistence:
     At the end of an instruction, Anchor automatically serializes any changes made to the account's data and saves them back to the blockchain. Without this constraint, changes will not be persisted, and the transaction might fail if modifications are attempted.

Parallel Execution:
     This attribute helps the runtime determine which transactions can be processed in parallel. If an account is not marked mutable, the runtime knows it can safely process other transactions reading from that same account simultaneously. 
*/

#[account(mut)]
#[account(mut @ <custom_error>)]


// #[account(init)]
/*
//OPTIMIZE  Core Functionality
Automatic Account Creation:
     Under the hood, Anchor performs a Cross-Program Invocation (CPI) to the Solana System Program to create the account.

Discriminator Setting:
     It automatically sets an 8-byte account discriminator, which is a unique identifier derived from the account's type name. This prevents one account type from being used as another.

Ownership Assignment:
     By default, it sets the owner of the newly created account to the program currently being executed. 

//OPTIMIZE   Required Parameters

When using init, you must provide the following additional constraints:
payer:
     Specifies which account will pay the SOL (rent) required for account creation. The payer must also be marked as mut.

space:
     Defines the number of bytes to allocate for the new account. This must include the 8 bytes for the Anchor discriminator plus the size of your data structure.

system_program:
     The instruction's accounts struct must include the System Program (of type Program<'info, System>) to facilitate the CPI for creation. 

*/
#[account(
    init,
    payer = <target_account>,
    space = <num_bytes>
)]


// #[account(init_if_needed)]
/*
//OPTIMIZE  Core Functionality
Conditional Creation:
     Unlike init, which fails if the target account already exists, init_if_needed allows the transaction to proceed even if the account is already initialized.
One-Step Workflow:
     It is primarily used to combine account initialization and state updates into a single transaction, simplifying the user experience.
Internal logic:
     If the account exists, Anchor skips the creation CPI; if it does not, Anchor creates the account, sets the discriminator, and assigns ownership.

//OPTIMIZE  Required Parameters
To use this constraint, the following must be provided:
payer:
     Specifies which account will pay the SOL rent for the new account if creation is necessary.
space:
     Defines the number of bytes to allocate for the new account.
Note:
     Space is not strictly required for certain account types (like Token accounts) where Anchor knows the fixed byte size.
system_program:
     The instruction's accounts struct must include the System Program (Program<'info, System>) to facilitate potential creation.
Cargo Feature:
     You must enable the init-if-needed feature in your anchor-lang dependency in Cargo.toml


Security Considerations

Re-initialization Attacks:
     Because the instruction succeeds even if the account exists, you must manually ensure that an attacker cannot "re-initialize" an existing account to reset its state unless that is your intended design.
Ownership Checks:
     If the account is a Program Derived Address (PDA), you must ensure proper ownership checks are in place to prevent attackers from providing their own malicious accounts.
*/

#[account(
    init_if_needed,
    payer = <target_account>
)]
 
#[account(
    init_if_needed,
    payer = <target_account>,
    space = <num_bytes>
)]


//#[account(seeds, bump)]
/*
//OPTIMIZE  Core Functionality
 PDA Verification:
    //CHANGED It ensures the provided account  address matches the one derived from the specified seeds the currently executing program_id, and a bump.

Off-Curve Security:
     The bump is a single-byte value (0–255) used to "bump" the derived address off the Ed25519 elliptic curve, //BUG ensuring it has no associated private key and can only be controlled by the program.

Canonical Bump:
     If bump is provided without a value, Anchor automatically finds and uses the canonical bump (the highest value that results in a valid PDA, starting from 255). 

Parameters and Syntax

seeds:
     An array of byte slices used as inputs for the derivation. Seeds can be static strings (e.g., b"vault") or dynamic data from other accounts (e.g., user.key().as_ref()).
bump:
    bump (empty):
         Tells Anchor to derive the canonical bump automatically.
    bump = <expr>:
         Allows you to specify a stored bump (e.g., bump = my_account.bump). This is a best practice for frequently accessed accounts as it saves compute units by avoiding re-derivation.

seeds::program = <expr> (Optional):
     Used when the PDA was derived by a different program than the one currently executing.
*/

#[account(
    seeds = <seeds>,
    bump
)]
 
#[account(
    seeds = <seeds>,
    bump,
    seeds::program = <expr>
)]
 
#[account(
    seeds = <seeds>,
    bump = <expr>
)]
 
#[account(
    seeds = <seeds>,
    bump = <expr>,
    seeds::program = <expr>
)]


//#[account(has_one = target)]
/*
//OPTIMIZE  Core Functionality

Field Matching:
      It ensures that a field within the primary account's data matches the public key of the specified target account in the Accounts struct.

Ownership/Access Control:
      It is most commonly used for authorization, such as verifying that the user calling an instruction is the authorized owner or administrator stored in the account's state.

Automatic Generation:
      Behind the scenes, Anchor generates a check equivalent to account.target == target.key().

Required Syntax and Naming

For the constraint to work, the following naming conventions must be met:

Primary Account:
      The account being validated must have a field named target in its data structure.
Target Account:
      An account or signer with the name target must exist in the same Accounts struct. 
*/

#[account(
    has_one = <target_account>
)]
 
#[account(
    has_one = <target_account> @ <custom_error>
)]

//#[account(address = expr)]
/*
//OPTIMIZE  Core Functionality
Exact Match:
      It verifies that account.key() == expr. If the provided account does not match the expression, the transaction will fail.

Hardcoding Values:
      It is most commonly used to restrict an instruction to a specific, well-known account, such as a global configuration account or a specific mint address (e.g., the USDC mint).

Access Control:
      This constraint provides a simple way to implement a "whitelist" or "admin-only" check by comparing a signer's address against a known public key.
Common Use Cases

Global Configs:
      Ensuring an instruction can only interact with the one true "Settings" account of your program.

Specific Minting:
      Restricting an instruction to only accept a specific SPL Token mint.

Developer Keys:
      Restricting administrative functions to a specific developer's wallet address.

*/

#[account(address = <expr>)]
#[account(address = <expr> @ <custom_error>)]



//#[account(owner = expr)]

/*
//OPTIMIZE  Core Functionality
Ownership Validation:
      It ensures that the owner field in the account's metadata matches the public key specified in <expr>.

Security Layer:
      By default, Anchor's Account<'info, T> type already checks that the account is owned by the currently executing program. The owner constraint is used when you need to verify that an account is owned by a different program (e.g., the Token Program or a specific third-party program). 

Common Use Cases

SPL Token Accounts:
           Verifying that a token account is owned by the SPL Token Program or Token-2022 Program.

External Program Data:
      Checking that a data account passed into your program was actually created and is managed by a specific external protocol.
*/
#[account(owner = <expr>)]
#[account(owner = <expr> @ <custom_error>)]


//#[account(executable)]
/*
//OPTIMIZE  Core Functionality
Executable Flag Check:
      It validates that the executable field in the account's metadata is set to true. On Solana, only accounts that hold compiled program code (BPF/SBF) have this flag enabled.

Security Validation:
      It ensures that an account passed as a program is actually an executable entity and not a malicious data account mimicking a program's address. 

Common Use Cases

Instruction Verification:
      When your program needs to interact with or verify another program, ensuring that the account provided is indeed an executable program.

CPI Safety:
      While Anchor's Program<'info, T> type automatically performs this check, you might use the executable constraint on an AccountInfo or UncheckedAccount if you are performing low-level manual Cross-Program Invocations (CPI)
      
.*/ 

//#[account(constraint = expr)]
/*
//OPTIMIZE //OPTIMIZE  Core Functionality
Custom Logic:
      It allows you to implement any check that isn't covered by standard built-in constraints (like signer or mut). This is often used for business logic validation, such as checking if a value is within a certain range or if an account's state matches specific criteria.

Boolean Evaluation:
      The expression <expr> must resolve to a boolean true or false. If it  evaluates to false, the transaction fails with a ConstraintRaw error (error code 2003) by default.

Context Access:
      Inside the expression, you can access other accounts in the same Accounts struct and fields within those accounts' data. 

Custom Errors
     You can provide a specific error message or code to be returned if the constraint is violated by using the @ symbol. 
Syntax: #[account(constraint = <expr> @ MyError::CustomErrorCode)]
*/

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(
        mut,
        // Custom constraint: data must be greater than 100
        constraint = my_account.data > 100 @ MyError::ValueTooLow
    )]
    pub my_account: Account<'info, MyData>,
    pub signer: Signer<'info>,
}

#[error_code]
pub enum MyError {
    #[msg("The provided value is too low.")]
    ValueTooLow,
}


//#[account(realloc)]
/*
//OPTIMIZE //OPTIMIZE  Core Functionality
Dynamic Resizing:
      It allows an account to either increase or decrease in size.

Automatic Rent Adjustment:
      When increasing size, the realloc::payer is automatically charged for the additional SOL required for rent exemption. When decreasing size, lamports are returned to the realloc::payer.

Execution Timing:
      The reallocation occurs at the beginning of the instruction, ensuring the updated space is available for use immediately. 

//OPTIMIZE Required Parameters

When using realloc, the following attributes must be specified together: 

realloc = <space>:
      The new total size of the account in bytes.

realloc::payer = <target>:
      The account responsible for paying additional rent or receiving the rent refund.

realloc::zero = <bool>:
      Determines if the new memory should be zero-initialized. Use true if you need the new space to be empty/predictable.
      
mut:
      The account being resized must also be marked as mutable.

system_program:
      The System Program must be included in the Accounts struct to facilitate the reallocation CPI. 
*/

#[derive(Accounts)]
#[instruction(new_size: u16)]
pub struct UpdateAccount<'info> {
    #[account(
        mut,
        realloc = 8 + new_size as usize, // 8 bytes for Anchor discriminator
        realloc::payer = signer,
        realloc::zero = false,
    )]
    pub data_account: Account<'info, MyData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


//BUG SPL Constraints
//#[account(token::*)]
/*
//OPTIMIZE //OPTIMIZE  Core Functionality
Validation:
      When used on an existing account, it verifies that the token account’s data (mint and authority) matches the specified accounts in the Accounts struct.

Initialization:
      When used with the init constraint, it automatically creates the token account via a CPI to the token program and sets its mint and authority. 
*/


/* 
//OPTIMIZE "Available Sub-Constraints"
The following parameters can be specified within the token:: attribute:

token::mint = <target>: Checks that the token account belongs to the specified mint address.
token::authority = <target>: Checks that the token account's owner (authority) matches the specified account's public key.
token::token_program = <target> (Optional): Specifies whether to use the original SPL Token Program or the Token-2022 (Token Extensions) Program. If omitted, it typically defaults to the original Token Program
*/
#[derive(Accounts)]
pub struct ValidateToken<'info> {
    #[account(
        token::mint = mint_account,
        token::authority = user_signer,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub mint_account: Account<'info, Mint>,
    pub user_signer: Signer<'info>,
}
//CHANGED Initializing a new token account (PDA)
#[derive(Accounts)]
pub struct InitTokenPDA<'info> {
    #[account(
        init,
        payer = user,
        seeds = [b"vault", mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vault_pda_authority,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub vault_pda_authority: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

// #[account(mint::*)]

/*
//OPTIMIZE  Core Functionality

//NOTE Create or validate New accounts with specified parameters.
Initialization:
      When combined with the init constraint, it allows you to create a new mint account in a single step, setting the decimals and the authority accounts.
Validation:
      When used on an existing account, it verifies that the mint's decimals and authorities match the specified values in your Accounts struct.

//OPTIMIZE Available Sub-Constraints

The following parameters can be specified within the mint:: attribute:
mint::decimals = <expr>:
     Sets or validates the number of decimal places for the token.

mint::authority = <target>:
      Sets or validates the public key allowed to mint new tokens.

mint::freeze_authority = <target> (Optional):
      Sets or validates the public key allowed to freeze token accounts.

token_program = <target> (Optional):
      Specifies whether the mint belongs to the original Token Program or the Token-2022 (Token Extensions) Program.


*/

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub new_mint: Account<'info, Mint>,
    
    /// CHECK: This is the PDA that will control the mint
    pub mint_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/* 
//OPTIMIZE Important Considerations for 2026

Token-2022 Compatibility:
      For mints requiring advanced features (like transfer hooks or metadata), ensure you use the token_program = token_2022_program constraint and the InterfaceAccount<'info, Mint> type to remain compatible with the latest Solana standards.

Authority Management:
      If you want a mint to be "fixed" (non-mintable) after creation, you can set the mint::authority to a None value in the instruction logic, though the constraint usually requires a valid Pubkey during the initial init call.

Space Requirements:
      When using init with mint::*, Anchor automatically calculates the correct space for the mint account, so you do not need to provide a manual space constraint.
*/

//#[account(associated_token::*)]
/*
//OPTIMIZE Core Feature: Deterministic Derivation
The primary feature of an ATA is that its address is a Program Derived Address (PDA) generated deterministically using the wallet's address and the token's mint address as seeds. 
Unique mapping:
      For any given wallet and mint combination, there is exactly one valid ATA address.

Eliminates discovery issues:
      Instead of searching for where a user stores their USDC, programs can calculate the exact address based on the user's public key and the USDC mint. 

//NOTE Use Cases
Automatic Initialization:
      When combined with init, Anchor automatically makes a CPI to the Associated Token Program to create and initialize the account for the user.

Validation:
      It verifies that a provided account is the legitimate ATA for the specified mint and authority, preventing attackers from passing in random token accounts they control.

Convenience:
      It simplifies instruction logic by handling the complex CPI calls and address verification internally. 

//OPTIMIZE Available Sub-Constraints
To use this attribute, you typically provide the following parameters: 
associated_token::mint = <target>:
      Specifies the mint account the ATA must belong to.

associated_token::authority = <target>:
      Specifies the owner (wallet) of the ATA.

associated_token::token_program = <target> (Optional):
      Specifies whether to use the original Token Program or the newer Token-2022 (Token Extensions) Program. 
*/

#[derive(Accounts)]
pub struct EnsureAta<'info> {
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_ata: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

//TODO Token Extensions Constraints

//#[account(extensions::transfer_hook::*)]

/*
//OPTIMIZE Core Functionality

This attribute is used exclusively on mint accounts to set or validate the parameters required for implementing custom logic that must run before any tokens from that mint are transferred.
When a transfer hook is active, any attempt to move tokens triggers a CPI to a predefined transfer_hook_program_id, allowing that program to execute custom checks or logic before allowing the transfer to finalize.
Available Sub-Constraints

The primary parameters you can define within this constraint are:

extensions::transfer_hook::authority = <target>: 
     Sets or validates the public key that has the permission to update or disable the transfer hook mechanism for the mint account.

extensions::transfer_hook::program_id = <target>:
      Sets or validates the public key of the external program that will be called when a transfer occurs.

//OPTIMIZE Use Cases
Compliance/AML Checks:
      The most common use case is to route all transfers through a compliance program that checks wallet statuses (e.g., if an address is blacklisted).

Conditional Transfers:
      Implementing logic that only allows transfers to specific types of accounts or after certain conditions are met (e.g., KYC completed).

*/

#[derive(Accounts)]
pub struct InitializeTransferHookMint<'info> {
    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = mint_authority,
        // Set the transfer hook program ID and authority ID
        extensions::transfer_hook::program_id = hook_program,
        extensions::transfer_hook::authority = hook_authority,
        // Must explicitly target the Token-2022 program
        mint::token_program = token_2022_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// CHECK: PDA authority for both minting and the hook
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: PDA authority specifically for the hook config
    pub hook_authority: AccountInfo<'info>,
    /// CHECK: The external program that handles the hook logic
    pub hook_program: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_2022_program: Program<'info, Token2022>,
}


//#[account(extensions::transfer_hook::*)]
/*
//OPTIMIZE Core Functionality

It enables developers to define logic that must execute before any tokens associated with a specific mint account can be transferred. When a transfer occurs, the Token-2022 program automatically performs a Cross-Program Invocation (CPI) to an external program (the "hook program") for validation.
Required Parameters (Sub-Constraints)
When using this attribute, you must specify the following parameters to fully configure the hook:

extensions::transfer_hook::program_id = <target>:
      Specifies the Pubkey of the external program that holds the custom pre-transfer logic. All transfers will CPI into this program.

extensions::transfer_hook::authority = <target>:
      Specifies the Pubkey of the account that has permission to update or disable the transfer hook mechanism in the future.

//OPTIMIZE Use Cases

Compliance Checks:
      Enforcing Anti-Money Laundering (AML) or Know-Your-Customer (KYC) rules by checking transfer participants against blacklists or whitelists.

Conditional Logic:
      Ensuring specific conditions are met before allowing a transfer to complete.
*/

#[derive(Accounts)]
pub struct InitializeTransferHookMint<'info> {
    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = mint_authority,
        // *** The Transfer Hook Configuration ***
        extensions::transfer_hook::program_id = my_hook_program_id,
        extensions::transfer_hook::authority = hook_authority,
        // Must specify the Token-2022 program explicitly
        mint::token_program = token_2022_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Authority for minting
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: Authority specifically for modifying the hook config
    pub hook_authority: AccountInfo<'info>,
    /// CHECK: The external program that handles the hook logic
    pub my_hook_program_id: AccountInfo<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_2022_program: Program<'info, Token2022>,
}
