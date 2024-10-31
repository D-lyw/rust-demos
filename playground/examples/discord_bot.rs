use dotenv::dotenv;
use ractor::async_trait;
use serenity::{
    all::{ChannelId, Context, EventHandler, GatewayIntents, Message},
    Client,
};
use std::env;

/// Discord Bot | send messages to discord channel
///
/// Ceate: https://docs.rs/serenity/latest/serenity/

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let channel = ChannelId::new(1301454061324996648);
            if let Err(why) = channel
                .say(&ctx.http, format!("hello {}", chrono::Local::now()))
                .await
            {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let discord_token = env::var("DISCORD_TOKEN")?;

    let intent = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&discord_token, intent)
        .event_handler(Handler)
        .await?;

    if let Err(error) = client.start().await {
        println!("Client run error: {}", error);
    }

    Ok(())
}
