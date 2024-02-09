use std::collections::HashMap;

use super::{
    extractors::extractor::Extractor,
    outputs::Output,
    downloaders::Downloader,
    postprocessors::*,
    cache::Cache,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub extractor: Extractor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloader: Option<Downloader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Output>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: comments cause panic
    pub postprocessor: Option<HashMap<String, Postprocessor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Cache>,
    pub format_separator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals_ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subconfigs: Option<Vec<Path>>,
    pub warnings: String,
}

impl Config {
    pub fn new() -> Self {
        return Config {
            extractor: Extractor::new(),
            downloader: None,
            output: None,
            postprocessor: None,
            globals: None,
            cache: None,
            format_separator: "/".to_string(),
            signals_ignore: None,
            subconfigs: None,
            warnings: "default".to_string(),
        }
    }
}
