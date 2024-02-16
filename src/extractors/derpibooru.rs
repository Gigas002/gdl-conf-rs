use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Derpibooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<i64>,
}

impl Derpibooru {
    pub fn new() -> Self {
        return Derpibooru {
            base: None,
            filter: Some(56027),
        }
    }
}
