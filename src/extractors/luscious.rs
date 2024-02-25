use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Luscious {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif: Option<bool>,
}

impl Luscious {
    pub fn new() -> Self {
        return Luscious {
            base: Some(ExtractorBase::new(None, None)),
            gif: Some(false),
        }
    }
}
