use anchor_lang::prelude::*;
#[account]
pub struct UserVault{
    pub owner:Pubkey,
    pub collateral:u64,
    pub borrowed:u64,
    pub bump:u8,
}
impl UserVault{
    //Manual space calculation(8 byte discriminator + fields)
    pub const LEN:usize=8+32+8+8+1;
}