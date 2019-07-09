extern crate regex;

use std::collections::HashMap;

use regex::Regex;

use crate::file::read_file;

#[derive(Debug)]
pub struct Conf {
    pub key: String,
    pub title: String,
    pub description: String,
}

pub const CONF_FILE_NAME: &str = "ext_emconf.php";

impl Conf {
    pub fn new() -> Self {
        Self {
            key: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
        }
    }

    pub fn new_from_file(path: &str) -> Self {
        let fields = read_from_file(path);

        let key = path.to_string();
        let mut title = "".to_string();
        let mut description = "".to_string();

        if let Some(t) = fields.get("title") {
            title = t.to_string();
        }

        if let Some(d) = fields.get("description") {
            description = d.to_string();
        }

        Self {
            key,
            title,
            description,
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

fn read_from_file(path: &str) -> HashMap<String, String> {
    let file = format!("{}/{}", path, CONF_FILE_NAME);
    let content = read_file(&file).expect("Failed to read file");
    let re = Regex::new(r"'(.*)' => '(.*)'").unwrap();
    let cap: Vec<(String, String)> = re.captures_iter(&content).map(|c| (c[1].to_string(), c[2].to_string())).collect();
    let fields: HashMap<String, String> = cap.into_iter().collect();
    fields
}