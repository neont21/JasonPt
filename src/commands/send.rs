use serde::{Deserialize, Serialize};
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
    utils::Colour,
};

#[group]
#[owners_only]
#[prefix("send")]
// #[prefix("임베드")]
#[description = "Sends the embed to the channel"]
// #[summary = "Sends the embed"]
#[default_command(send)]
#[commands(send_modify)]
pub struct Send;

#[derive(Serialize, Deserialize)]
struct ToEmbed {
    content: String,
    title: String,
    description: String,
    colour: u32,
    fields: Vec<(String, String, bool)>,
    bind: String,
}

#[derive(Serialize, Deserialize)]
struct ToEditEmbed {
    m_id: u64,
    content: String,
    title: String,
    description: String,
    colour: u32,
    fields: Vec<(String, String, bool)>,
    bind: String,
}

#[command]
#[description = "Sends the embed to the channel"]
#[required_permissions("ADMINISTRATOR")]
pub async fn send(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_embed: ToEmbed = serde_json::from_str(&args.rest()).expect("Input JSON");

    let chan = match &to_embed.bind[..] {
        "default" => msg.channel_id,
        other => ChannelId(String::from(&other[2..20]).parse::<u64>()?),
    };

    chan.send_message(&ctx.http, |m| {
        m.content(&to_embed.content);
        m.embed(|e| {
            e.title(&to_embed.title);
            e.description(&to_embed.description);
            e.fields(to_embed.fields);
            e.colour(Colour::new(to_embed.colour));

            e
        });

        m
    })
    .await?;

    Ok(())
}

#[command]
#[aliases("modify", "수정")]
#[description = "Edits the embed to the channel"]
#[required_permissions("ADMINISTRATOR")]
pub async fn send_modify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_edit: ToEditEmbed = serde_json::from_str(&args.rest()).expect("Input JSON");

    let chan = match &to_edit.bind[..] {
        "default" => msg.channel_id,
        other => ChannelId(String::from(&other[2..20]).parse::<u64>()?),
    };

    chan.edit_message(&ctx.http, MessageId(to_edit.m_id), |m| {
        m.content(&to_edit.content);
        m.embed(|e| {
            e.title(&to_edit.title);
            e.description(&to_edit.description);
            e.fields(to_edit.fields);
            e.colour(Colour::new(to_edit.colour));

            e
        });

        m
    })
    .await?;

    Ok(())
}
