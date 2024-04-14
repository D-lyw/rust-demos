use mini_redis::{client, Result};

/// mini redis client example
#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:9001").await?;

    client.set("k1", "hello".into()).await?;
    client.set("k2", "world".into()).await?;

    let res = client.get("k2").await?;
    match res {
        Some(v) => println!("Response: {}", String::from_utf8_lossy(&v)),
        None => println!("not found"),
    }
    Ok(())
}