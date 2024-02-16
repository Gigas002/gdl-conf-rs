use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Twibooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<i64>,
}

impl Twibooru {
    pub fn new() -> Self {
        return Twibooru {
            base: None,
            api_key: None,
            filter: Some(2),
        }
    }
}
