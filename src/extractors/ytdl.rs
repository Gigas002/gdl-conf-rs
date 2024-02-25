use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Ytdl {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    // raw-options: ,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmdline_args: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file: Option<Path>,
}

impl Ytdl {
    pub fn new() -> Self {
        return Ytdl {
            base: Some(ExtractorBase::new(None, None)),
            enabled: Some(false),
            format: Some("bestvideo+bestaudio/best".to_string()),
            generic: Some(true),
            logging: Some(true),
            module: None,
            cmdline_args: None,
            config_file: None,
        }
    }
}
