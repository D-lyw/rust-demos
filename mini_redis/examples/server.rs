use log::info;
use mini_redis::{Connection, Frame};
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listenr = TcpListener::bind("127.0.0.1:9001").await?;

    loop {
        match listenr.accept().await {
            Ok((socket, addr)) => {
                info!("{}", addr);
                println!("{}", addr);
                handle_connection(socket).await?;
            }
            Err(e) => {
                println!("accept error: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}

async fn handle_connection(socket: TcpStream) -> io::Result<()> {
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        info!("Received frame: {:?}", frame);

        let response = Frame::Error("Unimplemented".into());
        connection.write_frame(&response).await?;
    }
    Ok(())
}
