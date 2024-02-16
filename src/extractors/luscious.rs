use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Luscious {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif: Option<bool>,
}

impl Luscious {
    pub fn new() -> Self {
        return Luscious {
            base: None,
            gif: Some(false),
        }
    }
}
