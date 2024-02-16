use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Aryion {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

impl Aryion {
    pub fn new() -> Self {
        return Aryion {
            base: None,
            recursive: Some(true),
        }
    }
}
