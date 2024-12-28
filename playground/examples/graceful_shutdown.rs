use std::time::Duration;
use rand::Rng;
use tokio::time::sleep;
use tokio_util::task::TaskTracker;

/// Graceful shutdown by CancellationToken and TaskTracker
#[tokio::main]
async fn main() {
    let tracker = TaskTracker::new();

    for i in 0u32..10 {
        tracker.spawn(bg_task(i));
    }

    tracker.close();
    tracker.wait().await;
    println!("Shutdown service!")
}

/// a async task
async fn bg_task(task_id: u32) {
    let sleep_duration = rand::thread_rng().gen_range(3..=6);
    sleep(Duration::from_secs(sleep_duration)).await;
    println!("Task {} done in {}", task_id, sleep_duration);
}