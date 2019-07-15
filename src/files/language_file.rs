use crate::element::element::Element;
use crate::file::SaveFile;

pub struct LanguageFile<'a> {
    element: &'a Element
}

impl<'a> LanguageFile<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element
        }
    }
}

impl<'a> SaveFile for LanguageFile<'a> {
    fn content(&self) -> String {
        let language = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.0" xmlns:t3="http://typo3.org/schemas/xliff">
   <file source-language="en" datatype="plaintext" original="messages" date="${date}" product-name="${extension}">
      <header/>
      <body>
         <trans-unit id="${pluginkey}" xml:space="preserve">
            <source>${name}</source>
         </trans-unit>
         <trans-unit id="${pluginkey}.wizard.title" xml:space="preserve">
            <source>${title}</source>
         </trans-unit>
         <trans-unit id="${pluginkey}.wizard.description" xml:space="preserve">
            <source>${description}</source>
         </trans-unit>
      </body>
   </file>
</xliff>"#;

        language.replace("${extension}", &self.element.key.extension_key())
            .replace("${pluginkey}", &self.element.key.plugin_key())
            .replace("${name}", &self.element.get_name())
            .replace("${title}", &self.element.get_title())
            .replace("${description}", &self.element.get_description())

    }

    fn file(&self) -> String {
        format!("{}/Resources/Private/Language/{}.xlf", self.element.key.extension_key(), self.element.key.plugin_key())
    }
}