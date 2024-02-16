use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Booru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Booru {
    pub fn new() -> Self {
        return Booru {
            base: None,
            tags: Some(false),
            notes: Some(false),
            url: Some("file_url".to_string()),
        }
    }
}
