use anchor_lang::prelude::*;

declare_id!("3kEnttTDQhPKDeaamz6KAfWHfzJshxtdiCLJXkoUCFU8");

const MIN_RATING: u8 = 1;
const MAX_RATING: u8 = 5;
const MAX_TITLE_LENGTH: usize = 20;
const MAX_DESCRIPTION_LENGTH: usize = 50;

#[program]
pub mod anchor_movie_review_program {
    use super::*;
    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        require!(
            rating >= MIN_RATING && rating <= MAX_RATING,
            MoviewReviewError::InvalidRating
        );

        require!(
            title.len() <= MAX_TITLE_LENGTH,
            MoviewReviewError::TitleTooLong
        );

        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            MoviewReviewError::DescriptionTooLong
        );

        msg!("Movie Review Accounts created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;
        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMoviewReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        require!(
            rating >= MIN_RATING && rating <= MAX_RATING,
            MoviewReviewError::InvalidRating
        );

        require!(
            title.len() <= MAX_TITLE_LENGTH,
            MoviewReviewError::TitleTooLong
        );

        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            MoviewReviewError::DescriptionTooLong
        );

        msg!("Movie Review Accounts updated");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;
        Ok(())
    }

    pub fn delete_moview_review(_ctx: Context<DeleteMoviewReview>, title: String) -> Result<()> {
        msg!("Moview Review for {} deleted", title);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct AddMovieReview<'info> {
    #[account(init, seeds=[title.as_bytes(), initializer.key.as_ref()], payer = initializer, bump, space = MoviewAccountState::INIT_SPACE + DISCRIMINATOR)]
    movie_review: Account<'info, MoviewAccountState>,
    #[account(mut)]
    initializer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateMoviewReview<'info> {
    #[account(mut, seeds=[title.as_bytes(), initializer.key.as_ref()], bump, realloc = MoviewAccountState::INIT_SPACE + DISCRIMINATOR,
            realloc::payer = initializer,
            realloc::zero = true,
        )]
    movie_review: Account<'info, MoviewAccountState>,
    #[account(mut)]
    initializer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMoviewReview<'info> {
    #[account(
        mut,
        seeds=[title.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub moview_review: Account<'info, MoviewAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct MoviewAccountState {
    reviewer: Pubkey,
    rating: u8,
    #[max_len(20)]
    title: String,
    #[max_len(50)]
    description: String,
}

const DISCRIMINATOR: usize = 8;

#[error_code]
enum MoviewReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
    #[msg("Moview Title too long")]
    TitleTooLong,
    #[msg("Movie Description too long")]
    DescriptionTooLong,
}
