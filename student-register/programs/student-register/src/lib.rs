use anchor_lang::prelude::*;
const Discriminator: usize = 8;
const MAX_NAME_LENGTH: usize = 20;
const MAX_MESSAGE_LENGTH: usize = 20;

#[error_code]
enum StudentRegisterError {
    #[msg("Student name length should be less than 20")]
    NameTooLong,
    #[msg("Message length should be less than 50")]
    MessageTooLong,
}

declare_id!("75s6KicQEAyDL7WnVb7riWKpdATU6fbjncfLRrvSWLTo");

#[program]
pub mod student_register {
    use super::*;

    pub fn create_student(
        ctx: Context<CreateStudent>,
        name: String,
        message: String,
    ) -> Result<()> {
        require!(
            name.len() <= MAX_NAME_LENGTH,
            StudentRegisterError::NameTooLong
        );
        require!(
            message.len() <= MAX_MESSAGE_LENGTH,
            StudentRegisterError::MessageTooLong
        );

        let student_account = &mut ctx.accounts.student_account;
        student_account.name = name;
        student_account.message = message;
        student_account.owner = ctx.accounts.student.key();

        msg!("Student Accounts Created on Chain");
        msg!("Initialize the student name to {}", student_account.name);
        msg!(
            "Initialize the student message to {}",
            student_account.message
        );
        Ok(())
    }

    pub fn update_student(
        ctx: Context<UpdateStudent>,
        name: String,
        message: String,
    ) -> Result<()> {
        require!(
            name.len() <= MAX_NAME_LENGTH,
            StudentRegisterError::NameTooLong
        );
        require!(
            message.len() <= MAX_MESSAGE_LENGTH,
            StudentRegisterError::MessageTooLong
        );

        let student_account = &mut ctx.accounts.student_account;
        student_account.name = name;
        student_account.message = message;
        student_account.owner = ctx.accounts.student.key();

        msg!("Student Accounts Created on Chain");
        msg!("Initialize the student name to {}", student_account.name);
        msg!(
            "Initialize the student message to {}",
            student_account.message
        );
        Ok(())
    }

    pub fn delete_student(_ctx: Context<DeleteStudent>, name: String) -> Result<()> {
        msg!(
            "The student with the name: {} account has been deleted ",
            name
        );
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String,)]
pub struct CreateStudent<'info> {
    #[account(init, seeds = [student.key.as_ref(), name.as_bytes()], payer = student, bump, space = Discriminator + Student::INIT_SPACE )]
    student_account: Account<'info, Student>,
    #[account(mut)]
    student: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String,)]
pub struct UpdateStudent<'info> {
    #[account(mut, realloc = Discriminator + Student::INIT_SPACE, realloc::payer = student, seeds = [student.key.as_ref(), name.as_bytes()],
    bump, realloc::zero=true)]
    student_account: Account<'info, Student>,
    #[account(mut)]
    student: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String,)]
pub struct DeleteStudent<'info> {
    #[account(mut, seeds = [student.key.as_ref(), name.as_bytes()],
    bump, close= student)]
    student_account: Account<'info, Student>,
    #[account(mut)]
    student: Signer<'info>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Student {
    owner: Pubkey,
    #[max_len(20)]
    name: String,
    #[max_len(50)]
    message: String,
}
