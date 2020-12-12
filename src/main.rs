// for Environment Variable
use std::env;

// for Discord API
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

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
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    
    // It called when the bot is ready for the work.
    // It prints the name of the bot on the console.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
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
