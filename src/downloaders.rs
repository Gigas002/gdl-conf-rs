use std::collections::HashMap;
use super::enums::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DownloaderBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_directory: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<StringOrHashMap>,
}

impl DownloaderBase {
    pub fn new() -> Self {
        return DownloaderBase {
            enabled: Some(true),
            filesize_min: None,
            filesize_max: None,
            mtime: Some(true),
            part: Some(true),
            part_directory: None,
            progress: Some(3.0),
            rate: None,
            retries: None,
            timeout: None,
            verify: None,
            proxy: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Http {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<DownloaderBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_extensions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<StringOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_codes: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

impl Http {
    pub fn new() -> Self {
        return Http {
            base: None,
            adjust_extensions: Some(true),
            consume_content: Some(false),
            chunk_size: Some(StringOrInteger::Integer(32768)),
            headers: None,
            retry_codes: None,
            validate: Some(true),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Ytdl {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<DownloaderBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_cookies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outtmpl: Option<String>,
    // hashmap only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmdline_args: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file: Option<String>,
}

impl Ytdl {
    pub fn new() -> Self {
        return Ytdl {
            base: None,
            format: Some("bestvideo+bestaudio/best".to_string()),
            forward_cookies: Some(false),
            logging: Some(true),
            module: None,
            outtmpl: None,
            raw_options: None,
            cmdline_args: None,
            config_file: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Downloader {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<DownloaderBase>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(flatten)]
    // pub downloaders: Option<HashMap<String, Downloaders>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<Http>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytdl: Option<Ytdl>,
}

impl Downloader {
    pub fn new() -> Self {
        return Downloader {
            base: Some(DownloaderBase::new()),
            http: None,
            ytdl: None,
            // downloaders: None,
        };
    }
}

// This method doesn't work if we want to use as much optional fields as possible
// because it doesn't know how to match them to structs
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Downloaders {
    Http(Http),
    Ytdl(Ytdl),
}
