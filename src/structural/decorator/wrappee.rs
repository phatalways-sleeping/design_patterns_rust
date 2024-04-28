use super::notifier::Notifier;

pub struct SMSNotifier;

impl Notifier for SMSNotifier {
    fn send(&self) {
        println!("SMS Notifier sent!");
    }
}
