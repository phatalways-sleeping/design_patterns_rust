pub mod controllers;
pub mod services;
pub mod types;

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::{
        controllers::{
            injector::{GoogleTaskServiceInjector, NotionTaskServiceInjector},
            Controller, Injector,
        },
        types::Task,
    };

    #[tokio::test]
    async fn create_services() {
        let task = Task {
            created_at: std::time::Instant::now(),
            owner: "benjamin".to_owned(),
            title: "Exercise".to_owned(),
            description: "Perform daily workout".to_owned(),
            deadline: std::time::Instant::now()
                .checked_add(Duration::from_secs(1300))
                .unwrap(),
        };
        // Create using GoogleTask
        let service_controller = GoogleTaskServiceInjector.build();
        service_controller
            .create(task.clone())
            .await
            .expect("Failed to create google task");
        service_controller
            .remove("aishbsd-aschdsdhvs")
            .await
            .expect("Failed to remove google task");
        // Create using NotionTask
        let service_controller = NotionTaskServiceInjector.build();
        service_controller
            .create(task)
            .await
            .expect("Fail to create notion task");
        service_controller
            .remove("aishbsd-aschdsdhvs")
            .await
            .expect("Failed to remove notion task");
    }
}
