// RUN MULTIPLE FUTURES AT ONCE
use std::time;
use tokio::time::{Duration, sleep};

async fn lazy_hi() {
    log::info!("Yaaaw!! 🥱");
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi!");
}

pub async fn demo() {
    let start = time::Instant::now();
    for i in 1..=5 {
        lazy_hi().await;
    }
    let end = time::Instant::now();
    log::info!("Took {:?} secs", end - start);
}
