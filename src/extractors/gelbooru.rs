use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Gelbooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
    user_id: Option<String>,
}

impl Gelbooru {
    pub fn new() -> Self {
        return Gelbooru {
            base: None,
            api_key: None,
            user_id: None,
        }
    }
}
