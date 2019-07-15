use crate::file::SaveFile;
use crate::extension::extension::Extension;

#[derive(Debug)]
pub struct ConfFile<'a> {
    extension: &'a Extension
}

impl<'a> ConfFile<'a> {
    pub fn new(extension: &'a Extension) -> Self {
        Self {
            extension
        }
    }
}

impl<'a> SaveFile for ConfFile<'a> {
    fn content(&self) -> String {
        let config = r#"<?php
$EM_CONF[$_EXTKEY] = [
    'title' => '${title}',
    'description' => '${description}',
    'category' => 'fe',
    'state' => 'excludeFromUpdates',
    'clearCacheOnLoad' => true,
    'version' => '0.0.0',
    'constraints' => [
        'depends' => [
            'typo3' => '9.5.0-9.5.99',
        ],
    ]
];"#;
        config.replace("${title}", &self.extension.conf.title)
            .replace("${description}", &&self.extension.conf.description)
    }

    fn file(&self) -> String {
        format!("{}/ext_emconf.php", self.extension.key.extension_key())
    }
}
