use crate::key::Key;

#[derive(Debug)]
pub struct Element {
    pub key: Key,
    pub icon: String,
}

impl Element {
    pub fn new(key: Key) -> Self {
        Self {
            key,
            icon: "".to_string(),
        }
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = icon.to_string();
        self
    }
}
