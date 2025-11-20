// MOST BASIC ASYNC FUNCTION
use tokio::time::{Duration, sleep};

async fn lazy_hi() {
    log::info!("Yaaaw!! 🥱");
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi!");
}

pub async fn demo() {
    let future = lazy_hi();
    future.await;
}
