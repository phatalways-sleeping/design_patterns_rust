pub mod application;
pub mod command;
pub mod editor;

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::{
        application::Application, application::Orchestration, command::copy_command::CopyCommand,
        editor::Editor,
    };

    #[test]
    fn create_application_with_commands() {
        let app = Rc::new(RefCell::new(Application::new()));
        let editor = Rc::new(RefCell::new(Editor::new()));
        let copy_command = CopyCommand {
            application: app.clone(),
            editor: editor.clone(),
            backup: None,
        };
        app.borrow_mut().execute(Box::new(copy_command));
    }
}
