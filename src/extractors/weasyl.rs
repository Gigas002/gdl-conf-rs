use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Weasyl {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
    metadata: Option<bool>,
}

impl Weasyl {
    pub fn new() -> Self {
        return Weasyl {
            base: None,
            api_key: None,
            metadata: Some(false),
        }
    }
}
