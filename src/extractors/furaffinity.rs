use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Furaffinity {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
}

impl Furaffinity {
    pub fn new() -> Self {
        return Furaffinity {
            base: Some(ExtractorBase::new(None, None)),
            descriptions: Some("text".to_string()),
            external: Some(false),
            include: Some(StringOrList::String("gallery".to_string())),
            layout: Some("auto".to_string()),
        }
    }
}
