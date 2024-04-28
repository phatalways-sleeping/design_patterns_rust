use super::{
    command::{command_history::CommandHistory, Executable, ExecutableType},
    editor::Editor,
};

pub trait Orchestration {
    fn execute(&mut self, command: Box<dyn Executable>);
    fn undo(&mut self);
}

pub struct Application {
    pub editors: Vec<Editor>,
    pub clipboard: String,
    pub active_editor: Editor,
    pub history: CommandHistory,
}

impl Application {
    pub fn new() -> Self {
        Self {
            editors: vec![],
            clipboard: String::new(),
            active_editor: Editor::new(),
            history: CommandHistory::new(),
        }
    }
}

impl Orchestration for Application {
    fn execute(&mut self, mut command: Box<dyn Executable>) {
        if let Ok(ExecutableType::Changed) = command.execute() {
            self.history.push(command);
        }
    }

    fn undo(&mut self) {
        let prev_command = self.history.pop();
        if let Some(mut command) = prev_command {
            command.undo();
        }
    }
}
