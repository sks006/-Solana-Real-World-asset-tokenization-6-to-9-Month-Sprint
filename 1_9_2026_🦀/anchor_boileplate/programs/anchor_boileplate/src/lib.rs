pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("E2YhugaBwQJYmrregFiL5wyonac4emX36PuydXxAiCvW");

#[program]
pub mod anchor_boileplate {
    use super::*;

     pub fn deposit(ctx:Context<DepositCollateral>,amount:u64)->Result<()>(
      instructions::initialize::handler(ctx, amount)
    )
}