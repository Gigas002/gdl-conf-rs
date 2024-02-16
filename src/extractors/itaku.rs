use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Itaku {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<bool>,
}

impl Itaku {
    pub fn new() -> Self {
        return Itaku {
            base: None,
            videos: Some(true),
        }
    }
}
