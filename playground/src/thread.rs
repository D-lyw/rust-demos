use std::sync::atomic::{self, AtomicU64};
use std::time::Instant;
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

pub fn thread_atomic_example() {
    static  R: AtomicU64 = AtomicU64::new(0);

    const N_TIMES: u64 = 1000000;
    const N_THREADS: u64 = 10;

    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS as usize);

    for i in 0..N_THREADS {
        threads.push(thread::spawn(move || {
            for _ in 0..N_TIMES {
                R.fetch_add(1, atomic::Ordering::SeqCst);
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_THREADS * N_TIMES, R.load(atomic::Ordering::SeqCst));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        thread_channel_example();
        thread_atomic_example();
    }
}
