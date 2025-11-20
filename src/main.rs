#![allow(unused)]

mod ztokio;
use env_logger::Env;
use ztokio::*;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    demo14::demo().await;
}
