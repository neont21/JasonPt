// for Environment Variable
use std::env;

// for Discord API
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};

struct Handler;

static RESTRICT_USER: bool = true;

#[async_trait]
impl EventHandler for Handler {
    // Whenever a new message is received, it called asynchronously.
    async fn message(&self, ctx: Context, msg: Message) {
        // Configure the client with your Discord bot id in the environment.
        let bot_id = env::var("DISCORD_BOT_ID")
            .expect("Expected a bot id in the environment");

        // send `Pong!` by `ping`
        // if it's fail, print error message.
        if msg.content == format!("<@!{}> ping", bot_id) {
            if RESTRICT_USER {
                only_user("Pong!", ctx, msg).await;
            } else {
                all_user("Pong!", ctx, msg).await;
            }
        }
    }
    
    // It called when the bot is ready for the work.
    // It prints the name of the bot on the console.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn all_user(s: &str, ctx: Context, msg: Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, s).await {
        println!("Error sending message: {:?}", why);
    }
}
    
async fn only_user(s: &str, ctx: Context, msg: Message) {
    let user_id = env::var("DISCORD_USER_ID")
        .expect("Expected a user id in the environment");
    let user_id: u64 = user_id.parse().unwrap();

    if msg.author.id == UserId(user_id) {
        if let Err(why) = msg.channel_id.say(&ctx.http, s).await {
            println!("Error sending message: {:?}", why);
        }
    } else {
        if let Err(why) = msg.channel_id.say(&ctx.http, format!("난 <@!{}> 말만 들을건데요", user_id)).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the client by the bot token.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // error handling
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
