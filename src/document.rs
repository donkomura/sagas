#[derive(Debug, Clone)]
pub struct Document {
    pub id: u32,
    pub content: String,
}

impl Document {
    pub fn new(id: u32, content: String) -> Self {
        Self { id, content }
    }
} 