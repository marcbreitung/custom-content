#[derive(Debug)]
pub struct Key {
    extension: String,
    plugin: String,
}

impl Key {
    pub fn new(extension: &str, plugin: &str) -> Self {
        Self {
            extension: extension.to_string(),
            plugin: plugin.to_string(),
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