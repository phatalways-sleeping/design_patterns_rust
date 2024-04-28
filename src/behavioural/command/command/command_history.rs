use super::Executable;

pub struct CommandHistory {
    commands: Vec<Box<dyn Executable>>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn push(&mut self, command: Box<dyn Executable>) {
        self.commands.push(command);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Executable>> {
        self.commands.pop()
    }
}
