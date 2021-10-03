use tokio::time::{Duration, sleep};
use tokio;
use std::env;
use dotenv::dotenv;
use reqwest::{get};

#[tokio::main]
async fn main() {
    // Load env vars from dotenv
    dotenv().expect("Failed to read env vars");
    async fn poll_endpoint () {
        println!("Endpoint polled!")
    }

    // This is hideous
    let poll_duration = env::var("POLL_DURATION").unwrap().parse::<u64>().unwrap();
    println!("poll duration is {:?}!", poll_duration);
    loop {
        sleep(Duration::from_secs(poll_duration)).await;
        poll_endpoint().await;
    }

}

