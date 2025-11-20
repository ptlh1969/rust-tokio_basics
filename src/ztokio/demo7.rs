// std threads vs tokio threads
// tokio threads
//        - lightweight
//        - reuse os thread id
//        - does not guarantee same thread.
//        - Millions of threads - try spawning 100k
// std threads (OS Threads)
//        - expensive to spawn
//        - cannot be more than max capacity.
//        - different os thread id for each thread.
//        - Breaks above 1000
use std::thread::spawn;
use std::time;
use tokio::time::{Duration, sleep};

async fn lazy_hi() {
    log::info!("Yaaaw 🥱!");
    log::info!("Thread Id {:?}", std::thread::current().id());
    sleep(Duration::from_secs(5)).await;
    log::info!("lazy hi!");
}

pub async fn demo() {
    log::info!(
        "Total {:?} os threads available",
        std::thread::available_parallelism()
    );
    let start = time::Instant::now();
    let runtime = tokio::runtime::Handle::current();
    let mut task_handlers = Vec::new();

    for i in 1..=5000 {
        let rt_handle = runtime.clone();
        task_handlers.push(spawn(move || {
            rt_handle.block_on(lazy_hi());
        }));
    }

    for handler in task_handlers {
        handler.join();
    }

    let end = time::Instant::now();
    log::info!("Took {:?} secs", end - start);
}
