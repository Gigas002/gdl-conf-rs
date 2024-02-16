use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Unsplash {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
}

impl Unsplash {
    pub fn new() -> Self {
        return Unsplash {
            base: None,
            format: Some("raw".to_string()),
        }
    }
}
