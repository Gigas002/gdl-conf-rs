use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Plurk {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<bool>,
}

impl Plurk {
    pub fn new() -> Self {
        return Plurk {
            base: None,
            comments: Some(false),
        }
    }
}
