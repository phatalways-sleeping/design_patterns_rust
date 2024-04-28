pub trait Target {
    fn request(&self) -> String;
}

pub struct LinuxSoftware;

impl Target for LinuxSoftware {
    fn request(&self) -> String {
        "Linux Software".into()
    }
}