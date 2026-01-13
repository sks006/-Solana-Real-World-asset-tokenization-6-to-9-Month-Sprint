use anchor_lang::prelude::*;

#[account]
pub struct UserObligation {
    pub owner: Pubkey,
    pub deposited: u64,
    pub borrowed: u64,
    pub bump: u8,
}

impl UserObligation {
    pub fn is_initialized(&self) -> bool {
        self.owner != Pubkey::default()
    }
}
