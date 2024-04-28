use std::fmt::Debug;

use super::Service;

pub struct NotionTask;

impl Service for NotionTask {
    async fn create<T: Send + Sync + Debug>(&self, data: T) -> Result<(), &'static str> {
        println!("NotionTask: Sending request of {:?} creation", data);
        Ok(())
    }

    async fn remove<T: Send + Sync + Debug>(&self, id: T) -> Result<(), &'static str> {
        println!("NotionTask: Sending request of {:?} removal", id);
        Ok(())
    }
}
