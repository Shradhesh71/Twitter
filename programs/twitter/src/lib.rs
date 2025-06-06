use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod states;

declare_id!("31utvwA13kZZaTWcDCcwT2S1H877uRfKGydvnaMFXy2J");

#[program]
pub mod twitter {

    use super::*;

    pub fn initialize(ctx: Context<InitializeTweet>, topic: String, content: String, image: Option<String>) -> Result<()> {
        initialize_tweet(ctx, topic, content, image)
    }
    pub fn like_tweet(ctx: Context<AddReactionContext>) -> Result<()> {
        add_reaction(ctx, states::ReactionType::Like)
    }
    pub fn dislike_tweet(ctx: Context<AddReactionContext>) -> Result<()> {
        add_reaction(ctx, states::ReactionType::Dislike)
    }
    pub fn reaction_remove(ctx: Context<RemoveReactionContext>) -> Result<()> {
        remove_reaction(ctx)
    }
    pub fn comment_tweet(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
        add_comment(ctx, comment_content)
    }
    pub fn comment_remove(ctx: Context<RemoveCommentContext>) -> Result<()> {
        remove_comment(ctx)
    }
}
