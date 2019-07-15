#[derive(Debug)]
pub struct Key {
    extension: String,
    plugin: String,
}

impl Key {
    pub fn new(extension: &str, plugin: &str) -> Self {
        Self {
            extension: extension.to_string().to_lowercase(),
            plugin: plugin.to_string().to_lowercase(),
        }
    }

    pub fn extension_key(&self) -> String {
        self.extension.clone()
    }

    pub fn plugin_key(&self) -> String {
        let extension = self.extension.clone()
            .replace("_", "")
            .replace("-", "");

        let plugin = self.plugin.clone()
            .replace("_", "")
            .replace("-", "");

        format!("{}_{}", extension, plugin)
    }

    pub fn template_name(&self) -> String {
        let plugin = self.plugin.clone()
            .replace("_", "")
            .replace("-", "");

        self.uppercase_first_letter(&plugin)
    }

    fn uppercase_first_letter(&self, text: &str) -> String {
        let mut c = text.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}