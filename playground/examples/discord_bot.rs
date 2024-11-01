use dotenv::dotenv;
use ractor::async_trait;
use serenity::{
    all::{
        ChannelId, Context, CreateEmbed, CreateMessage, EventHandler, GatewayIntents, Message,
        MessageBuilder, Ready,
    },
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
        match msg.content.as_str() {
            "!ping" => {
                let channel = ChannelId::new(1301454061324996648);
                let msg = MessageBuilder::new()
                    .push_line(format!(
                        "[now is {}](https://google.com)",
                        chrono::Local::now()
                    ))
                    .build();
                // send message to special channel
                if let Err(error) = channel.say(&ctx.http, msg).await {
                    println!("Error sending message: {error:?}");
                }
            }
            "!dm" => {
                let embed = CreateEmbed::new()
                    .title("This is an embed")
                    .description("With a description");
                let echo_message = CreateMessage::new().content("Hello, World!").embed(embed);
                // send back direct message to sender
                if let Err(error) = msg.author.dm(&ctx, echo_message).await {
                    println!("Error sending message: {error:?}");
                }
            }
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
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

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        shard_manager.shutdown_all().await;
        println!("Bot is shotdown...")
    });

    if let Err(error) = client.start().await {
        println!("Client run error: {}", error);
    }

    Ok(())
}
