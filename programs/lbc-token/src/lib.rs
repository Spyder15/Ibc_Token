use anchor_lang::prelude::*;
pub mod instructions;

pub use instructions::*;

declare_id!("5jHQBa4z9vXqMuHrRKH6LrhutziAf26ujKFGkZmfrfvk");

#[program]
pub mod lbc_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
