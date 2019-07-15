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

    pub fn get_name(&self) -> String {
        let plugin = self.key.plugin.clone()
            .replace("_", "")
            .replace("-", "");

        self.uppercase_first_letter(&plugin)
    }

    pub fn get_title(&self) -> String {
        self.get_name()
    }

    pub fn get_description(&self) -> String {
        self.get_name()
    }

    pub fn get_template_name(&self) -> String {
        self.get_name()
    }

    fn uppercase_first_letter(&self, text: &str) -> String {
        let mut c = text.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
