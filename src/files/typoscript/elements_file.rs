use crate::element::element::Element;
use crate::file::SaveFile;

#[derive(Debug)]
pub struct ElementsFile<'a> {
    element: &'a Element
}

impl<'a> ElementsFile<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element
        }
    }
}

impl<'a> SaveFile for ElementsFile<'a> {
    fn content(&self) -> String {
        let plugin = r"@import 'EXT:${extension}/Configuration/TypoScript/ContentElement/*.typoscript'";
        plugin.replace("${extension}", &self.element.key.extension_key())
    }

    fn file(&self) -> String {
        format!("{}/Configuration/TypoScript/setup.typoscript", self.element.key.extension_key())
    }
}
