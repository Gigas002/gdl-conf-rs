use super::{
    extractors::extractor::Extractor,
    outputs::Output,
    downloaders::Downloader,
    cache::Cache,
    postprocessors::RootPostprocessor,
    enums::Path,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extractor: Option<Extractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloader: Option<Downloader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Output>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postprocessor: Option<RootPostprocessor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Cache>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_separator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals_ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subconfigs: Option<Vec<Path>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
}

impl Config {
    pub fn default() -> Self {
        return Config {
            extractor: Some(Extractor::default()),
            downloader: Some(Downloader::default()),
            output: Some(Output::default()),
            postprocessor: Some(RootPostprocessor::default()),
            globals: None,
            cache: Some(Cache::default()),
            format_separator: Some("/".to_string()),
            signals_ignore: None,
            subconfigs: None,
            warnings: Some("default".to_string()),
        }
    }

    pub fn new() -> Self {
        return Config {
            extractor: None,
            downloader: None,
            output: None,
            postprocessor: None,
            globals: None,
            cache: None,
            format_separator: None,
            signals_ignore: None,
            subconfigs: None,
            warnings: None,
        }
    }
}
