use crate::dependency_injection::services::{google_task::GoogleTask, notion_task::NotionTask};

use super::{consumer::Consumer, Controller, Injector};

pub struct GoogleTaskServiceInjector;

impl Injector for GoogleTaskServiceInjector {
    fn build(&self) -> impl Controller {
        Consumer::new(GoogleTask)
    }
}

pub struct NotionTaskServiceInjector;

impl Injector for NotionTaskServiceInjector {
    fn build(&self) -> impl Controller {
        Consumer::new(NotionTask)
    }
}
