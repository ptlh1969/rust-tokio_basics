// Joinset return value from future
use std::time;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};

async fn lazy_hi(i: i32) -> i32 {
    if i % 20 == 0 {
        panic!("let me sleep {i}! 😴");
    }
    log::info!("Yaaaw {i}!! 🥱");
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi {i}!");
    i * i
}

pub async fn demo() {
    let start = time::Instant::now();
    let mut set = JoinSet::new();
    for i in 1..=5 {
        set.spawn(async move { lazy_hi(i).await });
    }
    while let Some(result) = set.join_next().await {
        match result {
            Ok(value) => log::info!("task executed fine: squared value {value}"),
            Err(err) => log::error!("task paniced: {err}"),
        }
    }
    let end = time::Instant::now();
    log::info!("Took {:?} secs", end - start);
}
