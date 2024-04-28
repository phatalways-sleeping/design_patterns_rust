use std::fmt::Debug;

use super::Service;

pub struct GoogleTask;

impl Service for GoogleTask {
    async fn create<T: Send + Sync + Debug>(&self, data: T) -> Result<(), &'static str> {
        println!("GoogleTask: Sending request of {:?} creation", data);
        Ok(())
    }

    async fn remove<T: Send + Sync + Debug>(&self, id: T) -> Result<(), &'static str> {
        println!("GoogleTask: Sending request of {:?} removal", id);
        Ok(())
    }
}
