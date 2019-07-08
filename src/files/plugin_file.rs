use crate::element::element::Element;
use crate::file::SaveFile;

pub struct PluginFile<'a> {
    element: &'a Element
}

impl<'a> PluginFile<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element
        }
    }
}

impl<'a> SaveFile for PluginFile<'a> {
    fn content(&self) -> String {
        let plugin = r"<?php
\TYPO3\CMS\Core\Utility\ExtensionManagementUtility::addPlugin(
   array(
      'LLL:EXT:${extension}/Resources/Private/Language/Tca.xlf:${pluginkey}',
      '${pluginkey}',
      'EXT:${extension}/Resources/Public/Icons/ContentElements/${pluginkey}.gif'
   ),
   'CType',
   '${extension}'
);

$GLOBALS['TCA']['tt_content']['types']['${pluginkey}'] = [
   'showitem' => '
         --palette--;LLL:EXT:frontend/Resources/Private/Language/locallang_ttc.xml:palette.general;general,
         --palette--;LLL:EXT:frontend/Resources/Private/Language/locallang_ttc.xml:palette.header;header,
      --div--;LLL:EXT:frontend/Resources/Private/Language/locallang_ttc.xml:tabs.appearance,
         --palette--;LLL:EXT:frontend/Resources/Private/Language/locallang_ttc.xml:palette.frames;frames,
      --div--;LLL:EXT:frontend/Resources/Private/Language/locallang_ttc.xml:tabs.access,
         --palette--;LLL:EXT:frontend/Resources/Private/Language/locallang_ttc.xml:palette.visibility;visibility,
         --palette--;LLL:EXT:frontend/Resources/Private/Language/locallang_ttc.xml:palette.access;access,
      --div--;LLL:EXT:frontend/Resources/Private/Language/locallang_ttc.xml:tabs.extended
'];";

        plugin.replace("${pluginkey}", &self.element.key.plugin_key())
            .replace("${extension}", &self.element.key.extension_key())
    }

    fn file(&self) -> String {
        format!("{}/Configuration/TCA/Overrides/tt_content_{}.php", self.element.key.extension_key(), self.element.key.plugin_key())
    }
}