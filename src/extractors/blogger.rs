use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Blogger {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    videos: Option<bool>,
}

impl Blogger {
    pub fn new() -> Self {
        return Blogger {
            base: None,
            videos: Some(true),
        }
    }
}
