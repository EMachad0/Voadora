mod common;

use std::sync::{Arc, Mutex};
use voadora::Queue;

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let exit = Arc::new(Mutex::new(false));

    let exit_clone = exit.clone();
    let worker_thread = tokio::spawn(async move {
        let mut queue = voadora::sqlite_queue::SqliteQueue::new()
            .await
            .expect("Failed to create queue");

        while !*exit_clone.lock().unwrap() {
            let job_box = queue.pop().await;
            if let Ok(job_box) = job_box {
                info!("Running job");
                job_box.run();
            } else {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    });

    let exit_clone_2 = exit.clone();
    let heartbeat_thread = tokio::spawn(async move {
        while !*exit_clone_2.lock().unwrap() {
            trace!("Heartbeat");
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });

    loop {
        let mut buffer = String::new();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                if buffer.trim() == "exit" {
                    *exit.lock().unwrap() = true;
                    break;
                } else {
                    println!("Type 'exit' to finish the program");
                }
                buffer.clear();
            }
            Err(_) => {
                eprintln!("Failed to read input");
            }
        }
    }

    worker_thread.await.expect("Worker thread panicked");
    heartbeat_thread.await.expect("Heartbeat thread panicked");
}
