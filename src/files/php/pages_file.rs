use crate::file::SaveFile;
use crate::extension::extension::Extension;

#[derive(Debug)]
pub struct PagesFile<'a> {
    extension: &'a Extension
}

impl<'a> PagesFile<'a> {
    pub fn new(extension: &'a Extension) -> Self {
        Self {
            extension
        }
    }
}

impl<'a> SaveFile for PagesFile<'a> {
    fn content(&self) -> String {
        let config = r#"<?php

defined('TYPO3_MODE') || die();

call_user_func(function () {

    /**
     * Extension key
     */
    $extensionKey = '${extension}';

    \TYPO3\CMS\Core\Utility\ExtensionManagementUtility::registerPageTSConfigFile(
        $extensionKey,
        'Configuration/TsConfig/ts_config.typoscript',
        '${title}'
    );

});"#;
        config.replace("${extension}", &self.extension.key.extension_key())
            .replace("${title}", &self.extension.conf.title)
    }

    fn file(&self) -> String {
        format!("{}/Configuration/TCA/Overrides/pages.php", self.extension.key.extension_key())
    }
}
