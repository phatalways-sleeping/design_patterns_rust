use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Task {
    pub created_at: Instant,
    pub owner: String,
    pub title: String,
    pub description: String,
    pub deadline: Instant,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}
