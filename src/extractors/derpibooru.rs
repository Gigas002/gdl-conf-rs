use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Derpibooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<i64>,
}

impl Derpibooru {
    pub fn new() -> Self {
        return Derpibooru {
            base: Some(ExtractorBase::new(None, None)),
            filter: Some(56027),
        }
    }
}
