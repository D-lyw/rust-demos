use std::task::{Context, Poll, Waker};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

struct MyFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

fn main() {
    let shared_state = Arc::new(Mutex::new(SharedState {
        completed: false,
        waker: None,
    }));

    let my_future = MyFuture {
        shared_state: shared_state.clone(),
    };

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        let mut shared_state = shared_state.lock().unwrap();
        shared_state.completed = true;
        if let Some(waker) = shared_state.waker.take() {
            waker.wake();
        }
    });

    futures::executor::block_on(my_future);
}
