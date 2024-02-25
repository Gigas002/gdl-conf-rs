use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Wallhaven {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
}

impl Wallhaven {
    pub fn new() -> Self {
        return Wallhaven {
            base: Some(ExtractorBase::new(None, None)),
            api_key: None,
            include: Some(StringOrList::String("uploads".to_string())),
            metadata: Some(false),
        }
    }
}
