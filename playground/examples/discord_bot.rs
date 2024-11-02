use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use dashmap::DashMap;
use dotenv::dotenv;
use ractor::async_trait;
use serenity::{
    all::{
        ChannelId, Context, CreateEmbed, CreateMessage, EventHandler, GatewayIntents, Message,
        MessageBuilder, Ready, UserId,
    },
    prelude::TypeMapKey,
    Client,
};
use std::{env, sync::Arc};

/// Discord Bot | send messages to discord channel
///
/// Ceate: https://docs.rs/serenity/latest/serenity/

struct Conversations;
impl TypeMapKey for Conversations {
    type Value = Arc<DashMap<UserId, Vec<ChatCompletionRequestMessage>>>;
}

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
        if msg.content.starts_with("!ask") {
            // chat with AI
            let conversations_lock = {
                let global_data = ctx.data.read().await;
                global_data.get::<Conversations>().unwrap().clone()
            };

            let mut conversation = match conversations_lock.get(&msg.author.id) {
                Some(messages) => messages.to_vec(),
                None => vec![ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a great AI assistant, answer user's questions!")
                    .build()
                    .unwrap()
                    .into()],
            };

            let question = msg.content.strip_prefix("!ask ").unwrap_or(&msg.content);
            println!("Q: {}", question);

            conversation.push(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(question)
                    .build()
                    .unwrap()
                    .into(),
            );

            println!("{:?}", conversation);
            let answer = chat_ai(conversation.clone())
                .await
                .expect("AI service error");

            if let Err(error) = msg.channel_id.say(&ctx.http, answer).await {
                println!("Error sending message: {error:?}");
            } else {
                //  persistent chat history context
                conversations_lock
                    .insert(msg.author.id, conversation)
                    .unwrap();
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn chat_ai(question: Vec<ChatCompletionRequestMessage>) -> anyhow::Result<String> {
    let openai_client = async_openai::Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(512u32)
        .messages(question)
        .build()?;

    let response = openai_client.chat().create(request).await?;

    if let Some(choice) = response.choices.last() {
        Ok(choice
            .message
            .content
            .clone()
            .unwrap_or(String::from("No content")))
    } else {
        Ok(String::from("Error"))
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

    {
        let mut data = client.data.write().await;
        data.insert::<Conversations>(Arc::new(DashMap::new()));
    }

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
