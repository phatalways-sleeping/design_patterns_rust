pub mod command_history;
pub mod copy_command;
pub mod cut_command;
pub mod paste_command;
pub mod undo_command;

pub enum ExecutableError {
    CopyError,
    CutError,
    PasteError,
    UndoError,
    UndefinedError,
}

pub enum ExecutableType {
    Changed,
    Unchanged,
}

pub type ExecutableResult = Result<ExecutableType, ExecutableError>;

pub trait Executable {
    fn execute(&mut self) -> ExecutableResult;
    fn save_backup(&mut self) {}
    fn undo(&mut self) {}
}
