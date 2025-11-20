// pass data with scope
use std::time;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};

async fn lazy_hi(i: i32, name: &str) {
    log::info!("Yaaaw {i} {name}!! 🥱");
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi {i} {name}!");
}

pub async fn demo() {
    let start = time::Instant::now();
    let mut set = JoinSet::new();
    let name = "Newton";
    for i in 1..=5 {
        let future = lazy_hi(i, name);
        set.spawn(future);
    }
    while let Some(result) = set.join_next().await {
        result.expect("task panicked");
    }

    log::info!("is {name} still valid ?");
    let end = time::Instant::now();
    log::info!("Took {:?} secs", end - start);
}
