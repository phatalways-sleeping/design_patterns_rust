use super::server::Server;

pub struct Application {
    data: Vec<String>,
}

impl Server for Application {
    fn handle(&self, endpoint: &str) -> (u16, String) {
        match endpoint {
            "/users" => (200, self.data.last().unwrap().into()),
            "/tasks" => (400, "Error: Failed to load".into()),
            _ => (500, "Internal Server Error".into()),
        }
    }
}
