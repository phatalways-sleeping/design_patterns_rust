use std::fmt::Debug;

use crate::dependency_injection::services::Service;

use super::Controller;

pub struct Consumer<S: Service> {
    service: S,
}

impl<S: Service> Controller for Consumer<S> {
    type Inner = S;

    fn new(service: Self::Inner) -> Self {
        Self { service }
    }

    async fn create<T: Send + Sync + Debug>(&self, data: T) -> Result<(), &'static str> {
        self.service.create(data).await
    }

    async fn remove<T: Send + Sync + Debug>(&self, id: T) -> Result<(), &'static str> {
        self.service.remove(id).await
    }
}
