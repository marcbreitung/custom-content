use crate::file::SaveFile;
use crate::extension::extension::Extension;

#[derive(Debug)]
pub struct SysTemplateFile<'a> {
    extension: &'a Extension
}

impl<'a> SysTemplateFile<'a> {
    pub fn new(extension: &'a Extension) -> Self {
        Self {
            extension
        }
    }
}

impl<'a> SaveFile for SysTemplateFile<'a> {
    fn content(&self) -> String {
        let config = r#"<?php

defined('TYPO3_MODE') || die();

call_user_func(function () {

    /**
     * Extension key
     */
    $extensionKey = '${extension}';

    /**
     * Default TypoScript
     */
    \TYPO3\CMS\Core\Utility\ExtensionManagementUtility::addStaticFile(
        $extensionKey,
        'Configuration/TypoScript',
        '${title}'
    );

});"#;
        config.replace("${extension}", &self.extension.key.extension_key())
            .replace("${title}", &self.extension.conf.title)
    }

    fn file(&self) -> String {
        format!("{}/Configuration/TCA/Overrides/sys_template.php", self.extension.key.extension_key())
    }
}
