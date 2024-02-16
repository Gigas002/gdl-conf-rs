use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Booru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<bool>,
    notes: Option<bool>,
    url: Option<String>,
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
