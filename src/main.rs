// fr Environment Variable
use std::{collections::{HashMap, HashSet}, env, sync::Arc};

// for Discord API
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup, DispatchError, HelpOptions, help_commands, Reason, StandardFramework, macros::{
            command, group, help, check, hook,
        },
    },
    http::Http,
    model::{
        channel::Message, gateway::Ready, id::UserId
    },
    utils::{
        content_safe, ContentSafeOptions,
    },
    prelude::*,
};

// for using data across the Shards
use::tokio::sync::Mutex;


struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;
impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

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
#[commands(about, send)]
struct General;

#[group]
#[owners_only]
#[only_in(guilds)]
#[commands(bind, ping)]
struct Owner;

// something response to `help` command
#[help]
#[individual_command_tip="안녕하세요, 제이슨입니다!\n\n\
각각의 명령어의 기능이 알고 싶다면 인자로 전달하면 됩니다."]
#[command_not_found_text = "`{}`라는 명령어는 없는데요?"]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
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
async fn bind(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "**TODO** 미구현 함수").await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[checks(Owner)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong! :)").await?;

    Ok(())
}

#[command]
async fn send(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
        ContentSafeOptions::default()
            .clean_channel(false)
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings).await;

    // send to CURRENT channel
    msg.channel_id.say(&ctx.http, &content).await?;

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
