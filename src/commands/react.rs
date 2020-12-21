use std::collections::HashSet;

use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::{Message, ReactionType::Unicode},
    prelude::*,
};

use serde::{Deserialize, Serialize};

#[group]
#[owners_only]
#[prefix("react")]
#[prefix("반응")]
#[description = "Reacts to the message by emoji"]
#[summary = "Reacts by emoji"]
#[default_command(react)]
#[commands(react_remove)]
pub struct React;

#[derive(Serialize, Deserialize)]
struct ToReact {
    bind: String,
    m_id: u64,
    reactions: Vec<String>,
}

#[command]
#[owners_only]
#[only_in(guilds)]
#[description = "Reacts to the message by emoji"]
#[required_permissions("ADMINISTRATOR")]
pub async fn react(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_react: ToReact = serde_json::from_str(&args.rest()).expect("Input JSON");

    let c_id: u64 = match &to_react.bind[..] {
        "default" => *msg.channel_id.as_u64(),
        other => String::from(&other[2..20]).parse::<u64>()?,
    };

    for reaction in to_react.reactions {
        ctx.http
            .create_reaction(c_id, to_react.m_id, &Unicode(reaction))
            .await?;
    }

    Ok(())
}

#[command]
#[owners_only]
#[only_in(guilds)]
#[aliases("remove", "해제")]
#[description = "Removes the reaction of the message"]
#[required_permissions("ADMINISTRATOR")]
pub async fn react_remove(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_react: ToReact = serde_json::from_str(&args.rest()).expect("Input JSON");

    let c_id: u64 = match &to_react.bind[..] {
        "default" => *msg.channel_id.as_u64(),
        other => String::from(&other[2..20]).parse::<u64>()?,
    };

    let (_owners, bot_id) = match ctx.http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match ctx.http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    for reaction in to_react.reactions {
        ctx.http
            .delete_reaction(
                c_id,
                to_react.m_id,
                Some(*bot_id.as_u64()),
                &Unicode(reaction),
            )
            .await?;
    }

    Ok(())
}
