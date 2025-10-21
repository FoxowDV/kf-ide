use std::{path::PathBuf};

#[derive(Clone)]
pub struct Document{
    pub name: String,
    pub content: String,
    pub is_modified: bool,
    pub file_path: Option<PathBuf>,
}

impl Document {
    pub fn new(name: String) -> Self {
        Self {
            name,
            content: String::new(),
            is_modified: false,
            file_path: None,
        }
    }

    pub fn _open(name: String, content: String) -> Self {
        Self {
            name,
            content,
            is_modified: false,
            file_path: None,
        }
    }
}
