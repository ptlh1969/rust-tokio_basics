// JoinSet with passing data to future
use std::time;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};

async fn lazy_hi(i: i32) {
    log::info!("Yaaaw {i} 🥱!");
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi {i}!");
}

pub async fn demo() {
    let start = time::Instant::now();
    let mut set = JoinSet::new();
    for i in 1..=5 {
        let future = lazy_hi(i);
        set.spawn(future);
    }
    while let Some(result) = set.join_next().await {
        result.expect("task panicked");
    }

    let end = time::Instant::now();
    log::info!("Took {:?} secs", end - start);
}
