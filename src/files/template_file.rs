use crate::element::element::Element;
use crate::file::SaveFile;

pub struct TemplateFile<'a> {
    element: &'a Element
}

impl<'a> TemplateFile<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element
        }
    }
}

impl<'a> SaveFile for TemplateFile<'a> {
    fn content(&self) -> String {
        let template = r#"<html xmlns:f="http://typo3.org/ns/TYPO3/CMS/Fluid/ViewHelpers" data-namespace-typo3-fluid="true">
<f:layout name="Default" />
<f:section name="Main">
    {data}
</f:section>
</html>"#;

        template.to_string()
    }

    fn file(&self) -> String {
        format!("{}/Resources/Private/Templates/{}.html", self.element.key.extension_key(), self.element.key.plugin_key())
    }
}