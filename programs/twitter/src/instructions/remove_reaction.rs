use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &ctx.accounts.tweet_reaction;
    let reaction = &tweet_reaction.reaction;

    match reaction {
        ReactionType::Dislike => {
            tweet.dislikes = tweet
                .dislikes
                .checked_sub(1)
                .ok_or(TwitterError::MinDislikesReached)?;
        }
        ReactionType::Like => {
            tweet.likes = tweet
                .likes
                .checked_sub(1)
                .ok_or(TwitterError::MinLikesReached)?;
        }
    }

    Ok(())
}
#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        mut,
        close=reaction_author,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref(),
            ],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
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
}
