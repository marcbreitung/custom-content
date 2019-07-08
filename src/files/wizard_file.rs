use crate::element::element::Element;
use crate::file::SaveFile;

#[derive(Debug)]
pub struct WizardFile<'a> {
    element: &'a Element
}

impl<'a> WizardFile<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element
        }
    }
}

impl<'a> SaveFile for WizardFile<'a> {
    fn content(&self) -> String {
        let wizard = r#"mod.wizards.newContentElement.wizardItems.common {
   elements {
      ${pluginkey} {
         iconIdentifier = ${icon}
         title = LLL:EXT:${extension}/Resources/Private/Language/Tca.xlf:${pluginkey}.wizard.title
         description = LLL:EXT:${extension}/Resources/Private/Language/Tca.xlf:${pluginkey}.wizard.description
         tt_content_defValues {
            CType = ${pluginkey}
         }
      }
   }
   show := addToList(${pluginkey})
}"#;
        wizard.replace("${icon}", &self.element.icon)
            .replace("${pluginkey}", &self.element.key.plugin_key())
            .replace("${extension}", &self.element.key.extension_key())
    }

    fn file(&self) -> String {
        format!("{}/Configuration/TsConfig/Page/{}.typoscript", self.element.key.extension_key(), self.element.key.plugin_key())
    }
}
