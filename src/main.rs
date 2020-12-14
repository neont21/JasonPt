// fr Environment Variable
use std::{collections::HashSet, env};

// for Discord API
use serenity::{
    async_trait,
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup, DispatchError, HelpOptions, help_commands, Reason, StandardFramework, macros::{
            command, group, help, check, hook,
        },
    },
    http::Http,
    model::{
        channel::{
            Message, ReactionType::Unicode,
        },
        gateway::Ready, id::{
            UserId, ChannelId, MessageId,
        },
    },
    utils::Colour,
    prelude::*,
};

// for JSON parsing
use serde::{Deserialize, Serialize};

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    // It called when the bot is ready for the work.
    // It prints the name of the bot on the console.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(about)]
struct General;

#[group]
#[owners_only]
#[only_in(guilds)]
#[commands(ping)]
struct Owner;

// something response to `help` command
#[help]
#[individual_command_tip="안녕하세요, 제이슨입니다!\n\n\
각각의 명령어의 기능이 알고 싶다면 인자로 전달하면 됩니다."]
#[command_not_found_text = "`{}`라는 명령어는 없는데요?"]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;

    Ok(())
}

// whenever before responding the command
#[hook]
async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Got command '{}' by user '{}'", command_name, msg.author.name);
     true
}

// whenever after responding the command
#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

// whenever the unknown command sent
#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

// whenever the normal message sent
#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}

// whenever the response of the message failed
#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(duration) = error {
        let _ = msg
            .channel_id
            .say(&ctx.http, &format!("Try this again in {} seconds.", duration.as_secs()))
            .await;
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .on_mention(Some(bot_id))
                   .prefix(&format!("<@!{}>", bot_id)[..])
                   .delimiters(vec![", ", ","])
                   .owners(owners)
                   )
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&SAY_GROUP)
        .group(&SEND_GROUP)
        .group(&REACT_GROUP)
        .group(&OWNER_GROUP);

    // Create a new instance of the client by the bot token.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    // Start the client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
#[description = "Sends the information about the bot"]
async fn about(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "안녕하세요, 제이슨입니다!").await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[checks(Owner)]
#[description = "Just a ping-pong"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong! :)").await?;

    Ok(())
}

#[group]
#[prefix("send")]
#[description = "Sends the embed to the channel"]
#[summary = "Sends the embed"]
#[default_command(send)]
#[commands(send_modify)]
struct Send;

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
#[aliases("modify")]
#[description = "Edits the embed to the channel"]
#[required_permissions("ADMINISTRATOR")]
async fn send_modify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_edit: ToEditEmbed = serde_json::from_str(&args.rest())
        .expect("Input JSON");

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
    }).await?;

    Ok(())
}

#[command]
#[description = "Sends the embed to the channel"]
#[required_permissions("ADMINISTRATOR")]
async fn send(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_embed: ToEmbed = serde_json::from_str(&args.rest())
        .expect("Input JSON");

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
    }).await?;

    Ok(())
}

#[group]
#[prefix("say")]
#[description = "Sends the text to the channel"]
#[summary = "Sends the text"]
#[default_command(say)]
#[commands(say_modify)]
struct Say;

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
#[aliases("modify")]
#[description = "Edits the text on the channel"]
#[required_permissions("ADMINISTRATOR")]
async fn say_modify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_edit: ToEditSay = serde_json::from_str(&args.rest())
        .expect("Input JSON");

    let chan: ChannelId = match &to_edit.bind[..] {
        "default" => msg.channel_id,
        other => ChannelId(String::from(&other[2..20]).parse::<u64>()?),
    };

    chan.edit_message(&ctx.http, MessageId(to_edit.m_id), |m| {
        m.content(&to_edit.content);

        m
    }).await?;

    Ok(())
}

#[command]
#[owners_only]
#[only_in(guilds)]
#[description = "Sends the text to the channel"]
#[required_permissions("ADMINISTRATOR")]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let to_say: ToSay = serde_json::from_str(&args.rest())
        .expect("Input JSON");

    let chan: ChannelId = match &to_say.bind[..] {
        "default" => msg.channel_id,
        other => ChannelId(String::from(&other[2..20]).parse::<u64>()?),
    };

    chan.send_message(&ctx.http, |m| {
        m.content(&to_say.content);

        m
    }).await?;

    Ok(())
}

#[group]
#[prefix("react")]
#[description = "Reacts to the message by emoji"]
#[summary = "Reacts by emoji"]
#[default_command(react)]
#[commands(react_remove)]
struct React;

#[derive(Serialize, Deserialize)]
struct ToReact {
    c_id: u64,
    m_id: u64,
    reactions: Vec<String>,
}

#[command]
#[owners_only]
#[only_in(guilds)]
#[aliases("remove")]
#[description = "Removes the reaction of the message"]
#[required_permissions("ADMINISTRATOR")]
async fn react_remove(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let to_react: ToReact = serde_json::from_str(&args.rest())
        .expect("Input JSON");

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
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    for reaction in to_react.reactions {
        ctx.http.delete_reaction(to_react.c_id, to_react.m_id, Some(*bot_id.as_u64()), &Unicode(reaction)).await?;
    }

    Ok(())
}

#[command]
#[owners_only]
#[only_in(guilds)]
#[description = "Reacts to the message by emoji"]
#[required_permissions("ADMINISTRATOR")]
async fn react(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let to_react: ToReact = serde_json::from_str(&args.rest())
        .expect("Input JSON");

    for reaction in to_react.reactions {
        ctx.http.create_reaction(to_react.c_id, to_react.m_id, &Unicode(reaction)).await?;
    }

    Ok(())
}

#[check]
#[name = "Owner"]
async fn owner_check(_: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> Result<(), Reason> {
    if msg.author.id != 488978655620366358 {
        return Err(Reason::User("Lacked owner permission".to_string()));
    }

    Ok(())
}
