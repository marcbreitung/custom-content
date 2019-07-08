use std::fs;

use crate::element::element::Element;
use crate::extension::conf::Conf;
use crate::file::SaveFile;
use crate::files::conf_file::ConfFile;
use crate::files::plugin_file::PluginFile;
use crate::files::typoscript_file::TyposcriptFile;
use crate::files::wizard_file::WizardFile;
use crate::key::Key;
use crate::files::template_file::TemplateFile;

#[derive(Debug)]
pub struct Extension {
    pub key: Key,
    pub conf: Conf,
    pub elements: Vec<Element>,
}

impl Extension {
    pub fn new(conf: Conf) -> Self {
        let key = Key::new(&conf.key, "");
        Self {
            key,
            conf,
            elements: vec![],
        }
    }

    pub fn add_element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }

    pub fn build(&self) {
        let conf_file = ConfFile::new(&self);
        self.save_as_file(&conf_file);
        for element in &self.elements {
            let wizard_file = WizardFile::new(&element);
            self.save_as_file(&wizard_file);

            let typoscript_file = TyposcriptFile::new(&element);
            self.save_as_file(&typoscript_file);

            let plugin_file = PluginFile::new(&element);
            self.save_as_file(&plugin_file);

            let template_file = TemplateFile::new(&element);
            self.save_as_file(&template_file);
        }
    }

    pub fn save_as_file<T>(&self, element: &T) where T: SaveFile {
        let file = element.file().clone();
        let dirs: Vec<&str> = file.split("/").filter(|e| !e.contains(".")).collect();
        let mut path = "".to_owned();

        for dir in dirs {
            path = path + dir + "/";
            if let Ok(_) = fs::create_dir(&path) {
                println!("Make new dir: {:#?}", path);
            };
        }

        fs::write(element.file(), element.content()).expect("Unable to write file");
    }
}