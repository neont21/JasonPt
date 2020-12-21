use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{
        channel::Message,
        id::{ChannelId, MessageId},
    },
    prelude::*,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ToDelete {
    bind: String,
    m_id: u64,
}

#[command]
#[aliases("삭제")]
#[description = "Deletes the message"]
pub async fn delete(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_delete: ToDelete = serde_json::from_str(&args.rest()).expect("Input JSON");

    let chan: ChannelId = match &to_delete.bind[..] {
        "default" => msg.channel_id,
        other => ChannelId(String::from(&other[2..20]).parse::<u64>()?),
    };

    chan.delete_message(&ctx.http, MessageId(to_delete.m_id))
        .await?;

    Ok(())
}
