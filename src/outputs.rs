use std::collections::HashMap;
use super::enums::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Output {
    pub mode: StringOrHashMap,
    // str only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    // str only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<String>,
    // str only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    pub shorten: bool,
    pub colors: HashMap<String, String>,
    pub ansi: bool,
    pub skip: bool,
    pub fallback: bool,
    pub private: bool,
    pub progress: BoolOrString,
    pub log: PathOrLogConf,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub logfile: Option<PathOrLogConf>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsupportedfile: Option<PathOrLogConf>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorfile: Option<PathOrLogConf>,
    pub num_to_str: bool,
}

impl Output {
    pub fn new() -> Self {
        let mut colors: HashMap<String, String> = HashMap::new();
        colors.insert("success".to_string(), "1;32".to_string());
        colors.insert("skip".to_string(), "2".to_string());

        return Output {
            mode: StringOrHashMap::String("auto".to_string()),
            stdout: None,
            stdin: None,
            stderr: None,
            shorten: true,
            colors,
            ansi: false,
            skip: true,
            fallback: true,
            private: false,
            progress: BoolOrString::Bool(true),
            log: PathOrLogConf::Path(Path::String("[{name}][{levelname}] {message}".to_string())),
            logfile: None,
            unsupportedfile: None,
            errorfile: None,
            num_to_str: false,
        };
    }
}
