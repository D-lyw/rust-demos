use std::future::Future;
use std::time::{Duration, Instant};
use std::pin::Pin;
use std::task::{Context, Poll};

struct SleepFuture {
    time: Duration,
    start: Option<Instant>,
}

impl Future for SleepFuture {
    type Output = Result<bool, String>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(start) = self.start {
            if start.elapsed() >= self.time {
                // 如果已经过了指定的时间，返回完成状态
                Poll::Ready(Ok(true))
            } else {
                // 如果时间还没到，重新安排一个唤醒
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        } else {
            // 第一次调用时，记录开始时间
            self.start = Some(Instant::now());
            // 安排一个唤醒
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn my_sleep(time: Duration) -> impl Future<Output = Result<bool, String>> {
    SleepFuture { time, start: None }
}

#[tokio::main]
async fn main() {
    let res = my_sleep(Duration::from_secs(1)).await;
    println!("{:?}", res);
}
