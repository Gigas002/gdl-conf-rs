use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Exhentai {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_retries: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl Exhentai {
    pub fn new() -> Self {
        return Exhentai {
            base: None,
            domain: Some("auto".to_string()),
            fallback_retries: Some(2),
            fav: Some("4".to_string()),
            gp: Some("resized".to_string()),
            limits: None,
            metadata: Some(false),
            original: Some(true),
            source: Some("gallery".to_string()),
        }
    }
}
