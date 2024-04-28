use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::{application::Application, server::Server};

pub struct BenjProxy {
    application: Arc<Application>,
    rate_limiter: Arc<Mutex<HashMap<String, u32>>>,
    maximum_requests: u32,
}

impl From<Arc<Application>> for BenjProxy {
    fn from(application: Arc<Application>) -> Self {
        Self {
            application,
            rate_limiter: Arc::new(Mutex::new(HashMap::new())),
            maximum_requests: 30,
        }
    }
}

impl BenjProxy {
    fn invalid_request(&self, endpoint: &str) -> bool {
        *self
            .rate_limiter
            .lock()
            .unwrap()
            .entry(endpoint.into())
            .or_insert(1)
            > self.maximum_requests
    }
}

impl Server for BenjProxy {
    fn handle(&self, endpoint: &str) -> (u16, String) {
        if self.invalid_request(endpoint) {
            (400, "You have exceeded the request limit".into())
        } else {
            self.application.handle(endpoint)
        }
    }
}
