use std::{cell::RefCell, rc::Rc};

use crate::behavioural::command::{
    application::Application,
    editor::{Editor, Modifiable, Selectable},
};

use super::{Executable, ExecutableResult, ExecutableType};

pub struct CutCommand {
    pub application: Rc<RefCell<Application>>,
    pub editor: Rc<RefCell<Editor>>,
    pub backup: Option<String>,
}

impl Executable for CutCommand {
    fn execute(&mut self) -> ExecutableResult {
        self.save_backup();
        let selection = self.editor.borrow_mut().get_selection();
        self.editor.borrow_mut().delete_selection();
        self.application.borrow_mut().clipboard = selection;
        Ok(ExecutableType::Changed)
    }
}
