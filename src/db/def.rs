use serde::{Deserialize, Serialize};

use crate::slint_generatedAppWindow::TodoEntry as UITodoEntry;
pub const TODO_TABLE: &str = "todo";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TodoEntry {
    pub uuid: String,
    pub title: String,
    pub note: String,
    pub priority: i32,
    pub is_open: bool,
}

impl From<UITodoEntry> for TodoEntry {
    fn from(entry: UITodoEntry) -> Self {
        TodoEntry {
            uuid: entry.uuid.into(),
            title: entry.title.into(),
            note: entry.note.into(),
            priority: entry.priority,
            is_open: entry.is_open,
        }
    }
}

impl From<TodoEntry> for UITodoEntry {
    fn from(entry: TodoEntry) -> Self {
        UITodoEntry {
            uuid: entry.uuid.into(),
            title: entry.title.into(),
            note: entry.note.into(),
            priority: entry.priority,
            is_open: entry.is_open,
        }
    }
}
