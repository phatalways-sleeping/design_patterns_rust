use std::{cell::RefCell, rc::Rc};

use crate::behavioural::command::{
    application::Application,
    editor::{Editor, Selectable},
};

use super::{Executable, ExecutableResult, ExecutableType};

pub struct CopyCommand {
    pub application: Rc<RefCell<Application>>,
    pub editor: Rc<RefCell<Editor>>,
    pub backup: Option<String>,
}

impl Executable for CopyCommand {
    fn execute(&mut self) -> ExecutableResult {
        let selection = self.editor.borrow_mut().get_selection();
        self.application.borrow_mut().clipboard = selection;
        Ok(ExecutableType::Unchanged)
    }
}
