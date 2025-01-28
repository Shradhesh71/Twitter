use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    let comment = &mut ctx.accounts.comment;

    require!(
        comment_content.as_bytes().len() <= CONTENT_LENGTH,
        TwitterError::ContentTooLong
    );

    let mut comment_data = [0u8; CONTENT_LENGTH];
    comment_data[..comment_content.as_bytes().len()].copy_from_slice(comment_content.as_bytes());
    comment.content = comment_data;

    comment.comment_author = ctx.accounts.comment_author.key();
    comment.parent_tweet = ctx.accounts.tweet.key();
    comment.content_length = comment_content.as_bytes().len() as u16;

    comment.bump = ctx.bumps.comment;

    Ok(())
}
#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,
    #[account(
        init,
        payer = comment_author,
        space = 8 + Comment::LEN,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            {anchor_lang::solana_program::hash::hash(comment_content.as_bytes()).to_bytes().as_ref()},
            tweet.key().as_ref(),
            ],
        bump)]
    pub comment: Account<'info, Comment>,

    #[account(
        mut,
        seeds = [
            tweet.topic[..tweet.topic_length as usize].as_ref(),
            TWEET_SEED.as_bytes(),
            tweet.tweet_author.key().as_ref()
        ],
        bump = tweet.bump
    )]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}
