use crate::Queue;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Default)]
pub struct MemoryQueue<T> {
    phantom: std::marker::PhantomData<T>,
    queue: Vec<String>,
}

impl<T> MemoryQueue<T> {
    pub fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
            queue: Vec::new(),
        }
    }
}

impl<T: Serialize + DeserializeOwned> Queue<T> for MemoryQueue<T> {
    async fn push(&mut self, t: &T) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(t)?;
        self.queue.push(serialized);
        Ok(())
    }

    async fn pop(&mut self) -> anyhow::Result<T> {
        let serialized = self.queue.pop().ok_or(anyhow::anyhow!("Queue is empty"))?;
        let t = serde_json::from_str(&serialized)?;
        Ok(t)
    }
}
