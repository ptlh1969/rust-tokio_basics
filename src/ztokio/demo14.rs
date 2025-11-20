// JoinSet with blocking code with spawn_blocking
use std::time;
use tokio::task::JoinSet;
use tokio::task::spawn_blocking;
use tokio::time::{Duration, sleep};

fn blocking_code() {
    let mut result = 0;
    for i in 1..500_000_000 {
        result += 1;
    }
}

async fn lazy_hi(i: i32) {
    log::info!("Yaaaw {i} 🥱!");
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi {i}!");

    //blocking code
    if i == 3 {
        // blocking_code();
        spawn_blocking(move || blocking_code());
        log::info!("blocking code processed {i}!");
    }
}

pub async fn demo() {
    let start = time::Instant::now();
    let mut set = JoinSet::new();
    // for i in 1..=100 {
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
