use std::fs;

use crate::element::element::Element;
use crate::extension::conf::Conf;
use crate::file::SaveFile;
use crate::files::php::conf_file::ConfFile;
use crate::files::php::plugin_file::PluginFile;
use crate::files::php::sys_template_file::SysTemplateFile;
use crate::files::php::pages_file::PagesFile;
use crate::files::template_file::TemplateFile;
use crate::files::typoscript::element_file::ElementFile;
use crate::files::typoscript::elements_file::ElementsFile;
use crate::files::typoscript::wizard_file::WizardFile;
use crate::files::typoscript::wizards_file::WizardsFile;
use crate::key::Key;
use crate::files::language_file::LanguageFile;

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

        let sys_template_file = SysTemplateFile::new(&self);
        self.save_as_file(&sys_template_file);

        let pages_file = PagesFile::new(&self);
        self.save_as_file(&pages_file);

        for element in &self.elements {
            let wizard_file = WizardFile::new(&element);
            self.save_as_file(&wizard_file);

            let wizards_file = WizardsFile::new(&element);
            self.save_as_file(&wizards_file);

            let typoscript_elements_file = ElementsFile::new(&element);
            self.save_as_file(&typoscript_elements_file);

            let typoscript_element_file = ElementFile::new(&element);
            self.save_as_file(&typoscript_element_file);

            let plugin_file = PluginFile::new(&element);
            self.save_as_file(&plugin_file);

            let template_file = TemplateFile::new(&element);
            self.save_as_file(&template_file);

            let language_file = LanguageFile::new(&element);
            self.save_as_file(&language_file);
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