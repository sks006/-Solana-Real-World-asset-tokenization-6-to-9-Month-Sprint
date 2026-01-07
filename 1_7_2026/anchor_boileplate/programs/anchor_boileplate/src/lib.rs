pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("E77kwj2dcDpYgxsubFRMLVjZGASVSe127gBTyGvJMfAy");

#[program]
pub mod anchor_boileplate {
    use super::*;

        // rules:this name deposit become the method name in typescript
    pub fn deposit(ctx:Context<DepositCollateral>,amount:u64)->Result<()>(
      instructions::initialize::handler(ctx, amount)
    )
}
