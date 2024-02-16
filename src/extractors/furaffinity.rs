use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Furaffinity {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    descriptions: Option<String>,
    external: Option<bool>,
    include: Option<StringOrList>,
    layout: Option<String>,
}

impl Furaffinity {
    pub fn new() -> Self {
        return Furaffinity {
            base: None,
            descriptions: Some("text".to_string()),
            external: Some(false),
            include: Some(StringOrList::String("gallery".to_string())),
            layout: Some("auto".to_string()),
        }
    }
}
