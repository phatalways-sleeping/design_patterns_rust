use super::{specific_target::WindowSoftware, target::Target};

pub struct TargetAdapter(pub WindowSoftware);

impl Target for TargetAdapter {
    fn request(&self) -> String {
        format!("Window application: {}", self.0.name)
    }
}