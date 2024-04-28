use std::{fmt::Debug, future::Future};

pub mod google_task;
pub mod notion_task;

pub trait Service: Send + Sync {
    fn create<T: Send + Sync + Debug>(
        &self,
        data: T,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;
    fn remove<T: Send + Sync + Debug>(
        &self,
        id: T,
    ) -> impl Future<Output = Result<(), &'static str>> + Send;
}
