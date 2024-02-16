use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Redgifs {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<StringOrList>,
}

impl Redgifs {
    pub fn new() -> Self {
        return Redgifs {
            base: None,
            format: Some(StringOrList::List(vec!["hd".to_string(), "sd".to_string(), "gif".to_string()])),
        }
    }
}
