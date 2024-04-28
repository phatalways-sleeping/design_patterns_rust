use std::fmt::Debug;

use super::services::Service;

pub mod consumer;
pub mod injector;

pub trait Controller {
    type Inner: Service;

    fn new(service: Self::Inner) -> Self;

    fn create<T: Send + Sync + Debug>(
        &self,
        data: T,
    ) -> impl std::future::Future<Output = Result<(), &'static str>> + Send;

    fn remove<T: Send + Sync + Debug>(
        &self,
        id: T,
    ) -> impl std::future::Future<Output = Result<(), &'static str>> + Send;
}

pub trait Injector {
    fn build(&self) -> impl Controller;
}
