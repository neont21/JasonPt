use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{
        channel::Message,
        id::{ChannelId, MessageId},
    },
    prelude::*,
};

use serde::{Deserialize, Serialize};

#[group]
#[owners_only]
#[prefix("say")]
#[prefix("텍스트")]
#[description = "Sends the text to the channel"]
#[summary = "Sends the text"]
#[default_command(say)]
#[commands(say_modify)]
pub struct Say;

#[derive(Serialize, Deserialize)]
struct ToSay {
    content: String,
    bind: String,
}

#[derive(Serialize, Deserialize)]
struct ToEditSay {
    m_id: u64,
    content: String,
    bind: String,
}

#[command]
#[owners_only]
#[only_in(guilds)]
#[description = "Sends the text to the channel"]
#[required_permissions("ADMINISTRATOR")]
pub async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_say: ToSay = serde_json::from_str(&args.rest()).expect("Input JSON");

    let chan: ChannelId = match &to_say.bind[..] {
        "default" => msg.channel_id,
        other => ChannelId(String::from(&other[2..20]).parse::<u64>()?),
    };

    chan.send_message(&ctx.http, |m| {
        m.content(&to_say.content);

        m
    })
    .await?;

    Ok(())
}

#[command]
#[owners_only]
#[only_in(guilds)]
#[aliases("modify", "수정")]
#[description = "Edits the text on the channel"]
#[required_permissions("ADMINISTRATOR")]
pub async fn say_modify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_edit: ToEditSay = serde_json::from_str(&args.rest()).expect("Input JSON");

    let chan: ChannelId = match &to_edit.bind[..] {
        "default" => msg.channel_id,
        other => ChannelId(String::from(&other[2..20]).parse::<u64>()?),
    };

    chan.edit_message(&ctx.http, MessageId(to_edit.m_id), |m| {
        m.content(&to_edit.content);

        m
    })
    .await?;

    Ok(())
}
