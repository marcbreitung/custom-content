#[derive(Debug)]
pub struct Key {
    pub extension: String,
    pub plugin: String,
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
}