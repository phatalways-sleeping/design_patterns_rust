use std::{cell::RefCell, rc::Rc};

use crate::behavioural::command::{
    application::Application, application::Orchestration, editor::Editor,
};

use super::{Executable, ExecutableResult, ExecutableType};

pub struct UndoCommand {
    pub application: Rc<RefCell<Application>>,
    pub editor: Rc<RefCell<Editor>>,
    pub backup: Option<String>,
}

impl Executable for UndoCommand {
    fn execute(&mut self) -> ExecutableResult {
        self.application.borrow_mut().undo();
        Ok(ExecutableType::Unchanged)
    }
}
