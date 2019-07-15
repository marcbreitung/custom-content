use crate::element::element::Element;
use crate::file::SaveFile;

#[derive(Debug)]
pub struct ElementFile<'a> {
    element: &'a Element
}

impl<'a> ElementFile<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element
        }
    }
}

impl<'a> SaveFile for ElementFile<'a> {
    fn content(&self) -> String {
        let plugin = r"tt_content {
   ${pluginkey} < lib.fluidContent
   ${pluginkey} {
      templateName = ${template}.html
   }
}";
        plugin.replace("${pluginkey}", &self.element.key.plugin_key())
            .replace("${template}", &self.element.get_template_name())
    }

    fn file(&self) -> String {
        format!("{}/Configuration/TypoScript/ContentElement/{}.typoscript", self.element.key.extension_key(), self.element.key.plugin_key())
    }
}
