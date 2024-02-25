use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Reactor {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif: Option<bool>,
}

impl Reactor {
    pub fn new() -> Self {
        return Reactor {
            base: Some(ExtractorBase::new(None, None)),
            gif: Some(false),
        }
    }
}
