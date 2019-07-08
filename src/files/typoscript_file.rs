use crate::element::element::Element;
use crate::file::SaveFile;

#[derive(Debug)]
pub struct TyposcriptFile<'a> {
    element: &'a Element
}

impl<'a> TyposcriptFile<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element
        }
    }
}

impl<'a> SaveFile for TyposcriptFile<'a> {
    fn content(&self) -> String {
        let plugin = r"tt_content {
   ${pluginkey} < lib.fluidContent
   ${pluginkey} {
      templateName = NewContentElement.html
   }
}";
        plugin.replace("${pluginkey}", &self.element.key.plugin_key())
    }

    fn file(&self) -> String {
        format!("{}/Configuration/TypoScript/ContentElement/{}.typoscript", self.element.key.extension_key(), self.element.key.plugin_key())
    }
}
