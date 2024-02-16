use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Oauth {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser: Option<bool>,
    cache: Option<bool>,
    host: Option<String>,
    port: Option<i64>,
}

impl Oauth {
    pub fn new() -> Self {
        return Oauth {
            base: None,
            browser: Some(true),
            cache: Some(true),
            host: Some("localhost".to_string()),
            port: Some(6414),
        }
    }
}
