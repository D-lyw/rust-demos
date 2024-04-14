use std::{collections::HashMap, sync::{Arc, Mutex}};

use env_logger::{Builder, Env};
use log::{error, info};
use mini_redis::{Command, Connection, Frame};
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

type DB = Arc<Mutex<HashMap<String, Vec<u8>>>>;

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let listenr = TcpListener::bind("127.0.0.1:9001").await?;

    // Refactor3: Global Mutex DB For All Connection Threads
    let db = Arc::new(Mutex::new(HashMap::new()));
    
    loop {
        let db = db.clone();
        match listenr.accept().await {
            Ok((socket, addr)) => {
                info!("{}", addr);
                println!("{}", addr);

                // Refactor1
                // handle_connection(socket).await?;

                // Refactor2: make new thread to handle connection
                // tokio::spawn(async move {
                //     if let Err(e) = handle_connection(socket).await {
                //       error!("Hanldle connection error: {:?}", e);
                //     }
                // });

                // Refactor3: Global Mutex DB For All Connection Threads
                tokio::spawn(async move {
                    if let Err(e) = handle_connection3(socket, db).await {
                        error!("Hanldle connection error: {:?}", e);
                    }
                });
            }
            Err(e) => {
                println!("accept error: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}

// async fn handle_connection(socket: TcpStream) -> io::Result<()> {
//     let mut connection = Connection::new(socket);

//     // Refactor2: store data by hashmap
//     let mut db = HashMap::new();

//     while let Some(frame) = connection.read_frame().await.unwrap() {
//         info!("Received frame: {:?}", frame);

//         let response = match Command::from_frame(frame).unwrap() {
//             Command::Set(cmd) => {
//                 db.insert(cmd.key().to_string(), cmd.value().to_vec());
//                 Frame::Simple("OK".to_string())
//             }
//             Command::Get(cmd) => {
//                 if let Some(value) = db.get(cmd.key()) {
//                    Frame::Bulk(value.clone().into())
//                 } else {
//                    Frame::Null
//                 }
//             }
//             cmd => panic!("Unknown command"),
//         };

//         // let response = Frame::Error("Unimplemented".into());
//         connection.write_frame(&response).await?;
//     };

//     Ok(())
// }


/// Refactor3: Global Mutex DB For All Connection Threads
async fn handle_connection3(socket: TcpStream, db: DB) -> io::Result<()> {
  let mut connection = Connection::new(socket);

  while let Some(frame) = connection.read_frame().await.unwrap() {
      info!("Received frame: {:?}", frame);

      let response = match Command::from_frame(frame).unwrap() {
          Command::Set(cmd) => {
              if let Ok(mut db) = db.lock() {
                  db.insert(cmd.key().to_string(), cmd.value().to_vec());
                  Frame::Simple("OK".to_string())
              } else {
                  Frame::Error("DB is locked".to_string())
              }
          }
          Command::Get(cmd) => {
              if let Ok(db) = db.lock() { 
                 if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                 } else {
                    Frame::Null
                 }
              } else {
                  Frame::Error("DB is locked".to_string())
              }
          }
          _ => panic!("Unknown command"),
      };

      // let response = Frame::Error("Unimplemented".into());
      connection.write_frame(&response).await?;
  };

  Ok(())
}