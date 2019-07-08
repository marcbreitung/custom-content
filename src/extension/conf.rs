#[derive(Debug)]
pub struct Conf {
    pub key: String,
    pub title: String,
    pub description: String,
}

impl Conf {
    pub fn new() -> Self {
        Self {
            key: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
        }
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }
}