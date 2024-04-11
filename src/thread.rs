use std::{sync::mpsc, thread};

/////////////////////
/// Rust threads
/////////////////////

/// Communication between threads by channel
pub fn thread_channel_example() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
       let a = String::from("Hello");
       tx1.send(a).unwrap();
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
       let b = String::from("World");
       tx2.send(b).unwrap();
    });

    drop(tx);
    
    // println!("Received: {:?}", rx.recv().unwrap());
    for v in rx {
      println!("Received: {:?}", v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        thread_channel_example();
    }
}