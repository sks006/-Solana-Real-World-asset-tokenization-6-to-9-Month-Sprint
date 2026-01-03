// pub mod constants;
// pub mod error;
// pub mod instructions;
// pub mod state;

// use anchor_lang::prelude::*;

// pub use constants::*;
// pub use instructions::*;
// pub use state::*;

// declare_id!("EPxRVXRxpc8cAeskM2BL8XTe2XyMQwhU5AaEg9bd7WoZ");

// #[program]
// pub mod anchor_boileplate {
//     use super::*;

//    // rules:this name deposit become the method name in typescript
//     pub fn deposit(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
//         instructions::initialize::handler(ctx, amount)
//     }
// }
// --------------------- 15 ----------------
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EPxRVXRxpc8cAeskM2BL8XTe2XyMQwhU5AaEg9bd7WoZ");

#[program]
pub mod anchor_boileplate {
    use super::*;

   // rules:this name deposit become the method name in typescript
    pub fn deposit(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
        instructions::initialize::handler(ctx:amount)
    }
}