// pub mod constants;
// pub mod error;
// pub mod instructions;
// pub mod state;

// use anchor_lang::prelude::*;

// pub use constants::*;
// pub use instructions::*;
// pub use state::*;

// declare_id!("5aSWC4fsTg4tKrh2MoLLagNuxMRz5TxcTfNaqhc9ugWL");

// #[program]
// pub mod anchor_boileplate {
//     use super::*;

//    // rules:this name deposit become the method name in typescript
//     pub fn deposit(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
//         instructions::initialize::handler(ctx, amount)
//     }
// }
// -----------------------------15 --------------------

// pub mod constants;
// pub mod error;
// pub mod instructions;
// pub mod state;

// use anchor_lang::prelude::*;

// pub use constants::*;
// pub use instructions::*;
// pub use state::*;

// declare_id!("--------------")

// #[program]
// pub mod anchor_boileplate{\
//     use super::*;
//     //rules: this name deposit become the method name in typescript 
//     pub fn deposit(ctx:Context<DepositCollateral>,amount:u64)->Result<()>{
//         instructions::initialize::handler(ctx,amount)
//     }
// }

// -----------------------------14 --------------------
// pub  mod constants;
// pub mod error;
// pub mod instructions;
// pub mod state;

// use anchor_lang::prelude::*;
// pub use constants::*;
// pub use instructions::*;
// pub use state::*;

// declare_id!("---------------------")

// #[program]
// pub mod anchor_boileplate{
//     use super::*;
//     // this name deposit become the method name in tyscript
//      pub  fn deposit(ctx:Context<DepositCollateral>,amount:u64)Result<()>{
//         instructions::initialize::handler(ctx.account)
//      }
// }
// ----------------------------------- 13 ----------------------
// pub  mod constants;
// pub mod error;
// pub mod instructions;
// pub mod state;

// use anchor_lang::prelude::*;
// pub use constants::*;
// pub use instructions::*;
// pub use state::*;

// declare_id!("---------------------")

// #[program]
// pub mod anchor_boileplate{
//     use super::*;
//     // this name deposit become the method name in tyscript
//     pub fn deposit(ctx:Context<DepositCollateral>,account:u64)Result<()>{
//     instructions::initialize::handler(ctx.account)
//     }
// }
// ----------------------------------- 12 ----------------------
// pub  mod constants;
// pub mod error;
// pub mod instructions;
// pub mod state;

// use anchor_lang::prelude::*;
// pub use constants::*;
// pub use instructions::*;
// pub use state::*;

// declare_id!("-----------------")
// #[program]
// pub mod anchor_lang{
//     use super::*;
//     pub fn deposit(ctx:Context<DepositCollateral>,account:u64)Result<()>{
//         instructions::initialize::handler(ctx.account)
//     }
// }
// ----------------------------------- 11 ----------------------
// pub  mod constants;
// pub mod error;
// pub mod instructions;
// pub mod state;

// use anchor_lang::prelude::*;
// pub use constants::*;
// pub use instructions::*;
// pub use state::*;

// declare_id!("-----------------")
// #[program]
// pub mod anchor_lang{
//     use super::*;
//     pub fn deposit(ctx:Context<DepositCollateral>,account:u64)Result<()>{
//         instructions::initialize::handler(ctx.account)
//     }
// }
// ----------------------------------- 10 ----------------------
// pub  mod constants;
// pub mod error;
// pub mod instructions;
// pub mod state;

// use anchor_lang::prelude::*;
// pub use constants::*;
// pub use instructions::*;
// pub use state::*;

// declare_id!("-----------------")
// #[program]
// pub mod anchor_lang{
//     use super::*;
//     pub fn deposit(ctx:Context<DepositCollateral>,account:u64)->Result<()>{
//         instructions::initialize::handler(ctx.account)
//     }
// }
// ----------------------------------- 9 ----------------------
pub  mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("-----------------")
#[program]
pub mod anchor_lang{
    use super::*;
    pub fn deposit(ctx:Context<DepositCollateral>,account:u64)->Result<()>{
        instructions::initialize::handler(
            ctx.account
        )
    }
}