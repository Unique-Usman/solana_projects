use anchor_lang::prelude::*;

declare_id!("F9fvqiSJkcGsN7HkxVCTiERjhGSPEMsf6RidoQn5qnzB");

#[program]
pub mod counter {
    use super::*;

    pub fn init_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        counter.count = 0;
        msg!("Initialize counter value to {}", counter.count);
        Ok(())
    }

    pub fn update_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        counter.count += 1;
        msg!("Increment counter value to {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init, payer = signer, space = 8 * 8)]
    counter: Account<'info, Counter>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(mut)]
    counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    count: u32,
}
