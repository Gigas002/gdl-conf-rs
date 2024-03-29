use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Oauth {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

impl Oauth {
    pub fn new() -> Self {
        return Oauth {
            base: Some(ExtractorBase::new(None, None)),
            browser: Some(true),
            cache: Some(true),
            host: Some("localhost".to_string()),
            port: Some(6414),
        }
    }
}
