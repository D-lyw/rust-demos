use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:9001").await?;

    client.set("k1", "hello".into()).await?;
    let res = client.get("k1").await?;

    println!("Response: {:?}", res);
    Ok(())
}