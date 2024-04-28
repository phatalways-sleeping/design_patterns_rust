pub trait Server {
    fn handle(&self, endpoint: &str) -> (u16, String);
}