use anchor_lang::prelude::*;

#[account]
pub struct UserObligation {
    pub owner: Pubkey,        // 32 bytes
    pub deposited_amount: u64, // 8 bytes
    pub borrowed_amount: u64,  // 8 bytes
    pub bump: u8,             // 1 byte
}

impl UserObligation {
    // Rule: 8 (Discriminator) + 32 + 8 + 8 + 1
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1;
}