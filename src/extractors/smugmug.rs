use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Smugmug {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    videos: Option<bool>,
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl Smugmug {
    pub fn new() -> Self {
        return Smugmug {
            base: None,
            videos: Some(true),
            api_key: None,
            api_secret: None,
        }
    }
}
