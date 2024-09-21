use crate::{JobBox, Queue};
use anyhow::Context;
use sqlx::prelude::*;
use sqlx::SqliteConnection;
use uuid::Uuid;

pub struct SqliteQueue {
    db: SqliteConnection,
}

impl SqliteQueue {
    pub async fn new() -> anyhow::Result<Self> {
        let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
        let mut db = SqliteConnection::connect(&db_url).await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS jobs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_uuid TEXT NOT NULL,
                params TEXT NOT NULL,
                finished_at DATETIME DEFAULT NULL
            )",
        )
        .execute(&mut db)
        .await?;
        Ok(Self { db })
    }
}

impl Queue<JobBox> for SqliteQueue {
    async fn push(&mut self, job: &JobBox) -> anyhow::Result<()> {
        let serialized_params = serde_json::to_string(&job.params)?;
        sqlx::query("INSERT INTO jobs (job_uuid, params) VALUES (?, ?)")
            .bind(Uuid::to_string(&job.job_uuid))
            .bind(serialized_params)
            .execute(&mut self.db)
            .await?;
        Ok(())
    }

    async fn pop(&mut self) -> anyhow::Result<JobBox> {
        let job = sqlx::query(
            "SELECT job_uuid, params FROM jobs WHERE finished_at IS NULL ORDER BY id LIMIT 1",
        )
        .map(|row: sqlx::sqlite::SqliteRow| JobBox {
            job_uuid: Uuid::parse_str(&row.get::<String, _>("job_uuid")).unwrap(),
            params: serde_json::from_str(&row.get::<String, _>("params")).unwrap(),
        })
        .fetch_one(&mut self.db)
        .await?;
        sqlx::query("UPDATE jobs SET finished_at = CURRENT_TIMESTAMP WHERE job_uuid = ?")
            .bind(Uuid::to_string(&job.job_uuid))
            .execute(&mut self.db)
            .await?;
        Ok(job)
    }
}
