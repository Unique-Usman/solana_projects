use anchor_lang::prelude::*;

declare_id!("C9z85jfQZJ6pum5hPXwJ69L6FZJDx3zsUneLLLbdnkL3");

#[program]
pub mod pda_counter {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.bump = ctx.bumps.counter;
        Ok(())
    }

    pub fn update_counter(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!(
            "The value of the counter has been increased to: {}",
            counter.count
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(init, space = 8 + Counter::INIT_SPACE, payer = signer, seeds=[signer.key().as_ref(), b"counter"], bump)]
    counter: Account<'info, Counter>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    signer: Signer<'info>,
    #[account( seeds=[signer.key().as_ref(), b"counter"], bump)]
    counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    bump: u8,
    count: u64,
}
