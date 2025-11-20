// MOST BASIC ASYNC FUNCTION
// TOKIO SPAWN
use std::time;
use tokio::spawn;
use tokio::time::{Duration, sleep};

async fn lazy_hi() {
    log::info!("Yaaaw!! 🥱");
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi!");
}

pub async fn demo() {
    let start = time::Instant::now();
    let future = spawn(lazy_hi());
    future.await;
    let end = time::Instant::now();
    log::info!("Took {:?} secs", end - start);
}
