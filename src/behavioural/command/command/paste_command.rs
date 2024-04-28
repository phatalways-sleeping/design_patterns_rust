use std::{cell::RefCell, rc::Rc};

use crate::behavioural::command::{
    application::Application,
    editor::{Editor, Modifiable},
};

use super::{Executable, ExecutableResult, ExecutableType};

pub struct PasteCommand {
    pub application: Rc<RefCell<Application>>,
    pub editor: Rc<RefCell<Editor>>,
    pub backup: Option<String>,
}

impl Executable for PasteCommand {
    fn execute(&mut self) -> ExecutableResult {
        self.save_backup();
        let clipboard = self.application.borrow().clipboard.clone();
        self.editor.borrow_mut().replace_selection_with(clipboard);
        Ok(ExecutableType::Changed)
    }
}
