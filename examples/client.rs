mod common;

use common::jobs::ex_job::JOB_ID;
use voadora::{JobBox, Queue};

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let args = std::env::args().collect::<Vec<String>>();
    let job_box = JobBox::new(JOB_ID, args);

    let mut queue = voadora::sqlite_queue::SqliteQueue::new()
        .await
        .expect("Failed to create queue");
    queue.push(&job_box).await.expect("Failed to push job");

    info!("Job pushed to the queue");
}
