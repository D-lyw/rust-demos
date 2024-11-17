use futures::StreamExt;
use redis::AsyncCommands;
use tokio::{
    self,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let redis_client = redis::Client::open("redis://127.0.0.1")?;

    let mut conn = redis_client.get_multiplexed_async_connection().await?;

    // command:ping
    let pong: String = redis::cmd("ping").query_async(&mut conn).await?;
    println!("Pong: {}", pong);

    // get value by key
    let res: String = conn.get("count").await?;
    println!("count key: {:?}", res);

    // subscribe channel
    let mut pubsub = redis_client.get_async_pubsub().await?;
    pubsub.subscribe("mq_channel").await?;
    tokio::spawn(async move {
        let mut stream = pubsub.on_message();
        while let Some(msg) = stream.next().await {
            let payload: String = msg.get_payload().unwrap();
            println!("收到消息: {}", payload);
        }
    });

    // publish message to channel
    for i in 1..5 {
        let _: () = conn
            .publish("mq_channel", format!("message count {}", i))
            .await?;
        sleep(Duration::from_millis(200)).await;
    }

    Ok(())
}
