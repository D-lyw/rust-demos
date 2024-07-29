use anyhow;
use crossbeam_channel::{after, select, tick};
use std::{
    thread,
    time::{Duration, Instant},
};

/// crossbeam-channel create is a alternative to std::sync::mpsc with more features and better performance.
/// Multi-producer multi-consumer channels for message passing.
fn main() -> anyhow::Result<()> {
    // usage1();
    // usage2()?;
    usage3()?;
    Ok(())
}

fn usage1() {
    let (tx, rx) = crossbeam_channel::unbounded::<i32>();
    let sender = std::thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });
    let receiver = std::thread::spawn(move || {
        for received in rx {
            println!("received: {}", received);
        }
    });
    sender.join().unwrap();
    receiver.join().unwrap();
}

// sharing channels
fn usage2() -> anyhow::Result<()> {
    let (tx1, rx1) = crossbeam_channel::unbounded();
    let (tx2, rx2) = (tx1.clone(), rx1.clone());

    // thread::spawn(move || {
    //     tx2.send(32)?;

    //     println!("received in rx2: {}", rx2.recv()?);
    //     Ok::<(), anyhow::Error>(())
    // });

    // tx1.send(21)?;
    // println!("received: {}", rx1.recv()?);

    thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(100));
            tx1.send(i)?;
        }
        Ok::<(), anyhow::Error>(())
    });

    let rx3 = rx1.clone();
    let rx4 = rx1.clone();
    let rx5 = rx1.clone();
    let rx6 = rx1.clone();

    let start = Instant::now();

    let delay = after(Duration::from_millis(3000));
    let ticker = tick(Duration::from_millis(100));

    loop {
        select! {
            recv(rx3) -> msg => println!("3: {:?}", msg),
            recv(rx4) -> msg => println!("4: {:?}", msg),
            recv(rx5) -> msg => println!("5: {:?}", msg),
            recv(rx6) -> msg => println!("6: {:?}", msg),
            recv(ticker) -> _ => println!("Ticker: {:?}", start.elapsed()),
            recv(delay) -> _ => break,
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct Msg {
    data: String,
    tx: oneshot::Sender<String>,
}

impl Msg {
    pub fn new(data: String) -> (Self, oneshot::Receiver<String>) {
        let (tx, rx) = oneshot::channel();
        (Self { data, tx }, rx)
    }
}

fn usage3() -> anyhow::Result<()> {
    let (tx, rx) = crossbeam_channel::unbounded::<Msg>();

    // send crossbeam_channel messages
    thread::spawn(move || {
        for i in 0..10 {
            let msg_str = format!("Task: {}", i);
            let (msg, rx_oneshot) = Msg::new(msg_str);
            tx.send(msg).unwrap();

            match rx_oneshot.recv_timeout(Duration::from_millis(1000)) {
                Ok(result) => println!("{}", result),
                Err(oneshot::RecvTimeoutError::Timeout) => eprintln!("Processor was too slow"),
                Err(oneshot::RecvTimeoutError::Disconnected) => panic!("Processor exited"),
            }
        }
        Ok::<(), anyhow::Error>(())
    });

    // receive crossbeam_channel messages
    let processor = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("Received: {}", msg.data);

            // add async task here using tokio runtime
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                // CUP cosumeing task
                let _ = tokio::time::sleep(Duration::from_millis(500)).await;
                // send oneshot message
                msg.tx.send(format!("{}, Done!", msg.data)).unwrap();
            });

            // runtime.spawn(async move {
            //     // CUP cosumeing task
            //     let _ = tokio::time::sleep(Duration::from_millis(500)).await;
            //     // send oneshot message
            //     msg.tx.send(format!("{}, Done!", msg.data)).unwrap();
            // });

            // simulate async task
            // thread::sleep(Duration::from_millis(100));

            // send oneshot message
            // msg.tx.send(format!("{}, Done!", msg.data)).unwrap();
        }
    });

    processor.join().unwrap();

    Ok(())
}
