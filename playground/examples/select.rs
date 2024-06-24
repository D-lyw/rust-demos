#[tokio::main]
async fn main() {
    // select! usage example
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(format!("task1: {}", i)).await.unwrap();
        }
    });

    tokio::spawn(async move {
        for i in 0..10 {
            tx2.send(format!("task2: {}", i)).await.unwrap();
        }
    });

    loop {
        tokio::select! {
            Some(val) = rx.recv() => {
                println!("received 1: {}", val);
            }
            Some(val) = rx2.recv() => {
                println!("received 2: {}", val);
            }
            else => {
                break;
            }
        }
    }

    println!("Done");
}
