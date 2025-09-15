
#[derive(Clone)]
pub struct Document{
    pub name: String,
    pub content: String,
    pub is_modified: bool,

}

impl Document {
    pub fn new(name: String) -> Self {
        Self {
            name,
            content: String::new(),
            is_modified: false,
        }
    }

    pub fn open(name: String, content: String) -> Self {
        Self {
            name,
            content,
            is_modified: false,
        }
    }
}
