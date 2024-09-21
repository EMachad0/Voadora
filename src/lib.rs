pub mod memory_queue;
pub mod sqlite_queue;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[allow(async_fn_in_trait)]
pub trait Queue<T: Serialize + DeserializeOwned> {
    async fn push(&mut self, job: &T) -> anyhow::Result<()>;
    async fn pop(&mut self) -> anyhow::Result<T>;
}

#[derive(Debug)]
pub struct Job {
    pub uuid: Uuid,
    pub perform: fn(&Vec<String>),
}

inventory::collect!(Job);

#[derive(Debug, Serialize, Deserialize)]
pub struct JobBox {
    job_uuid: Uuid,
    params: Vec<String>,
}

impl JobBox {
    pub fn new(job_uuid: Uuid, params: Vec<String>) -> Self {
        Self { job_uuid, params }
    }

    pub fn run(&self) {
        let job = inventory::iter::<Job>
            .into_iter()
            .find(|job| job.uuid == self.job_uuid)
            .expect("Job not found");
        (job.perform)(&self.params);
    }
}
