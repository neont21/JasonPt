use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[aliases("자기소개")]
#[description = "Sends the information about the bot"]
pub async fn about(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "안녕하세요, 제이슨입니다!")
        .await?;

    Ok(())
}
