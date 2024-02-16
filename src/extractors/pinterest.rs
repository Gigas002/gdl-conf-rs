use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pinterest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>,
    sections: Option<bool>,
    videos: Option<bool>,
}

impl Pinterest {
    pub fn new() -> Self {
        return Pinterest {
            base: None,
            domain: Some("auto".to_string()),
            sections: Some(true),
            videos: Some(true),
        }
    }
}
