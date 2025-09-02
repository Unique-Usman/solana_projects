use anchor_lang::prelude::*;

declare_id!("BANTSJ6xxV7PVeSpWpyyreaqZf2C9UFJw7A8Wj4s1Gfj");

#[program]
pub mod first_anchor_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello Solana Network, I want to be a Solana Blockchain Developer");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
