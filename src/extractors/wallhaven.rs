use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Wallhaven {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
    include: Option<StringOrList>,
    metadata: Option<bool>,
}

impl Wallhaven {
    pub fn new() -> Self {
        return Wallhaven {
            base: None,
            api_key: None,
            include: Some(StringOrList::String("uploads".to_string())),
            metadata: Some(false),
        }
    }
}
