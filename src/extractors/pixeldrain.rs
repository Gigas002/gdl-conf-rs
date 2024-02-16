use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pixeldrain {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
}

impl Pixeldrain {
    pub fn new() -> Self {
        return Pixeldrain {
            base: None,
            api_key: None,
        }
    }
}
