use bytes::Bytes;
use env_logger::{Builder, Env};
use log::info;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

enum Command {
  Set {
    key: String,
    val: Bytes
  },
  Get {
    key: String,
    resp: oneshot::Sender<Bytes>
  },
}

#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let (tx, mut rx) = mpsc::channel(32);

    let tx1 = tx.clone();

    tx.send(Command::Set { key: "k1".to_string(), val: "hello".into() }).await.unwrap();

    // tx1.send(Command::Get { key: "k1".to_string() }).await.unwrap();
    let task1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get { key: "k1".to_string(), resp: resp_tx, };
        if let Ok(_) = tx1.send(cmd).await {
            let resp = resp_rx.await.unwrap();
            info!("Command response: {:?}", resp);
        }
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("localhost:9001").await.unwrap();

        while let Some(cmd) = rx.recv().await {
          match cmd {
            Command::Set { key, val } => {
              client.set(&key, val).await.unwrap();
            }
            Command::Get { key, resp } => {
              if let Some(res) = client.get(&key).await.unwrap() {
                println!("Response: {}", String::from_utf8_lossy(&res));
                resp.send(res).unwrap();
              }
            }
          }
        };
    });
    task1.await.unwrap();
    manager.await.unwrap();
}