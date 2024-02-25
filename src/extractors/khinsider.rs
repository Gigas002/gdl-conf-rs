use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Khinsider {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl Khinsider {
    pub fn new() -> Self {
        return Khinsider {
            base: Some(ExtractorBase::new(None, None)),
            format: Some("mp3".to_string()),
        }
    }
}
