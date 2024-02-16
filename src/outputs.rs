use std::collections::HashMap;
use super::enums::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<StringOrHashMap>,
    // str only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    // str only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<String>,
    // str only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shorten: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ansi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<PathOrLogConf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logfile: Option<PathOrLogConf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsupportedfile: Option<PathOrLogConf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorfile: Option<PathOrLogConf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_to_str: Option<bool>,
}

impl Output {
    pub fn new() -> Self {
        let mut colors: HashMap<String, String> = HashMap::new();
        colors.insert("success".to_string(), "1;32".to_string());
        colors.insert("skip".to_string(), "2".to_string());

        return Output {
            mode: Some(StringOrHashMap::String("auto".to_string())),
            stdout: None,
            stdin: None,
            stderr: None,
            shorten: Some(BoolOrString::Bool(true)),
            colors: Some(colors),
            ansi: Some(false),
            skip: Some(true),
            fallback: Some(true),
            private: Some(false),
            progress: Some(BoolOrString::Bool(true)),
            log: Some(PathOrLogConf::Path(Path::String("[{name}][{levelname}] {message}".to_string()))),
            logfile: None,
            unsupportedfile: None,
            errorfile: None,
            num_to_str: Some(false),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct LoggingConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StringOrHashMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}
