use crate::element::element::Element;
use crate::file::SaveFile;

#[derive(Debug)]
pub struct WizardsFile<'a> {
    element: &'a Element
}

impl<'a> WizardsFile<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element
        }
    }
}

impl<'a> SaveFile for WizardsFile<'a> {
    fn content(&self) -> String {
        let wizard = r#"@import 'EXT:${extension}/Configuration/TsConfig/Page/*.typoscript'"#;
        wizard.replace("${extension}", &self.element.key.extension_key())
    }

    fn file(&self) -> String {
        format!("{}/Configuration/TsConfig/ts_config.typoscript", self.element.key.extension_key())
    }
}
