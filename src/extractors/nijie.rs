use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Nijie {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<StringOrList>,
}

impl Nijie {
    pub fn new() -> Self {
        return Nijie {
            base: None,
            include: Some(StringOrList::String("illustration,doujin".to_string())),
        }
    }
}
