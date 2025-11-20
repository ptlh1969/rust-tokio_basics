// RUN MULTIPLE FUTURES CONCURRENTLY USING TOKIO::JOIN!
use std::time;
use tokio::time::{Duration, sleep};

async fn lazy_hi() {
    log::info!("Yaaaw!! 🥱");
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi!");
}

pub async fn demo() {
    let start = time::Instant::now();
    tokio::join!(lazy_hi(), lazy_hi(), lazy_hi(), lazy_hi(), lazy_hi(),);
    let end = time::Instant::now();
    log::info!("Took {:?} secs", end - start);
}
