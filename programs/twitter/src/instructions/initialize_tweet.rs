use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn initialize_tweet(
    ctx: Context<InitializeTweet>,
    topic: String,
    content: String,
    image: Option<String>,
) -> Result<()> {
    let initialized_tweet = &mut ctx.accounts.tweet;

    require!(
        topic.as_bytes().len() <= TOPIC_LENGTH,
        TwitterError::TopicTooLong
    );

    require!(
        content.as_bytes().len() <= CONTENT_LENGTH,
        TwitterError::ContentTooLong
    );
    

    let mut topic_data = [0u8; TOPIC_LENGTH];

    topic_data[..topic.as_bytes().len()].copy_from_slice(topic.as_bytes());
    initialized_tweet.topic = topic_data;

    let mut content_data = [0u8; CONTENT_LENGTH];
    content_data[..content.as_bytes().len()].copy_from_slice(content.as_bytes());
    initialized_tweet.content = content_data;

    initialized_tweet.topic_length = topic.as_bytes().len() as u8;
    initialized_tweet.tweet_author = ctx.accounts.tweet_authority.key();
    initialized_tweet.likes = 0;
    initialized_tweet.dislikes = 0;

    if let Some(image) = image {
        let mut image_data = [0u8; IMAGE_LENGTH];
        image_data[..image.as_bytes().len()].copy_from_slice(image.as_bytes());
        initialized_tweet.image = image_data;
    }

    initialized_tweet.bump = ctx.bumps.tweet;

    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)]
pub struct InitializeTweet<'info> {
    #[account(mut)]
    pub tweet_authority: Signer<'info>,
    #[account(
        init,
        payer = tweet_authority,
        space = 8 + Tweet::LEN,
        seeds = [
            topic.as_bytes(),
            TWEET_SEED.as_bytes(),
            tweet_authority.key().as_ref()
            ],
        bump)]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}
