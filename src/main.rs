use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{channel::Message, gateway::Ready};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{command, group},
};

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is running!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("."))
        .group(&GENERAL_GROUP);

    let token = env::var("BOT_TOKEN").expect("Please provide a bot token.");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to instantiate the bot client.");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong! Welcome to something new and awesome.").await?;

    Ok(())
}
