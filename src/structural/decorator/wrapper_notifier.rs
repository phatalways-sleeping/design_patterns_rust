use super::notifier::Notifier;

pub trait WrapperNotifier: Notifier {
    fn wrappee(&self) -> &Box<dyn Notifier>;
}

pub struct FacebookNotifier {
    pub wrappee: Box<dyn Notifier>,
}

impl FacebookNotifier {
    pub const fn new(wrappee: Box<dyn Notifier>) -> Self {
        Self { wrappee }
    }
}

impl Notifier for FacebookNotifier {
    fn send(&self) {
        self.wrappee().send();
        println!("Facebook Notifier sent!");
    }
}

impl WrapperNotifier for FacebookNotifier {
    fn wrappee(&self) -> &Box<dyn Notifier> {
        &self.wrappee
    }
}
