use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Hitomi {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl Hitomi {
    pub fn new() -> Self {
        return Hitomi {
            base: None,
            format: Some("webp".to_string()),
        }
    }
}
