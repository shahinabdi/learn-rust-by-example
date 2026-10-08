use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Debug)]
pub enum TodoError {
    Io(io::Error),
    Json(serde_json::Error),
    NotFound(u32),
    EmptyTitle,
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::Io(e) => write!(f, "I/O error: {e}"),
            TodoError::Json(e) => write!(f, "corrupt data file: {e}"),
            TodoError::NotFound(id) => write!(f, "no task with id {id}"),
            TodoError::EmptyTitle => write!(f, "title must not be empty"),
        }
    }
}

impl std::error::Error for TodoError {}

// `?` converts these errors automatically.
impl From<io::Error> for TodoError {
    fn from(e: io::Error) -> Self {
        TodoError::Io(e)
    }
}
impl From<serde_json::Error> for TodoError {
    fn from(e: serde_json::Error) -> Self {
        TodoError::Json(e)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn load(path: &Path) -> Result<Self, TodoError> {
        match fs::read_to_string(path) {
            Ok(text) => Ok(serde_json::from_str(&text)?),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), TodoError> {
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn add(&mut self, title: &str) -> Result<u32, TodoError> {
        let title = title.trim();
        if title.is_empty() {
            return Err(TodoError::EmptyTitle);
        }
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.tasks.push(Task { id, title: title.to_string(), done: false });
        Ok(id)
    }

    pub fn complete(&mut self, id: u32) -> Result<(), TodoError> {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .map(|t| t.done = true)
            .ok_or(TodoError::NotFound(id))
    }

    pub fn remove(&mut self, id: u32) -> Result<Task, TodoError> {
        let pos = self.tasks.iter().position(|t| t.id == id).ok_or(TodoError::NotFound(id))?;
        Ok(self.tasks.remove(pos))
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_complete_remove() {
        let mut l = TodoList::default();
        let a = l.add("write code").unwrap();
        let b = l.add("test code").unwrap();
        assert_eq!((a, b), (1, 2));
        l.complete(a).unwrap();
        assert!(l.tasks()[0].done);
        assert_eq!(l.remove(b).unwrap().title, "test code");
        assert!(matches!(l.complete(99), Err(TodoError::NotFound(99))));
        assert!(matches!(l.add("   "), Err(TodoError::EmptyTitle)));
    }

    #[test]
    fn ids_are_not_reused() {
        let mut l = TodoList::default();
        l.add("a").unwrap();
        let b = l.add("b").unwrap();
        l.remove(b).unwrap();
        l.add("c").unwrap();
        assert_eq!(l.tasks().last().unwrap().id, 2);
    }

    #[test]
    fn json_round_trip() {
        let path = std::env::temp_dir().join("todo_cli_test.json");
        let mut l = TodoList::default();
        l.add("persist me").unwrap();
        l.save(&path).unwrap();
        let loaded = TodoList::load(&path).unwrap();
        assert_eq!(loaded.tasks(), l.tasks());
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn missing_file_is_empty_list() {
        let l = TodoList::load(Path::new("no/such/todo.json")).unwrap();
        assert!(l.tasks().is_empty());
    }
}
