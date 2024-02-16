use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Ytdl {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    format: Option<String>,
    generic: Option<bool>,
    logging: Option<bool>,
    module: Option<String>,
    // raw-options: ,
    cmdline_args: Option<StringOrList>,
    config_file: Option<Path>,
}

impl Ytdl {
    pub fn new() -> Self {
        return Ytdl {
            base: None,
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
