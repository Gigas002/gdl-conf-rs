use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vsco {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    videos: Option<bool>,
}

impl Vsco {
    pub fn new() -> Self {
        return Vsco {
            base: None,
            videos: Some(true),
        }
    }
}
