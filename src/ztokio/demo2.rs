// MOST BASIC ASYNC FUNCTION
// LOG THE TIME IT TAKES TO RUN THE ASYNC FUNCTION
use std::time;
use tokio::time::{Duration, sleep};

async fn lazy_hi() {
    log::info!("Yaaaw!! 🥱");
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi!");
}

pub async fn demo() {
    let future = lazy_hi();
    let start = time::Instant::now();
    future.await;
    let end = time::Instant::now();
    log::info!("Took {:?} secs", end - start);
}
