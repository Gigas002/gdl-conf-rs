use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fanbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<BoolOrPath>,
}

impl Fanbox {
    pub fn new() -> Self {
        return Fanbox {
            base: None,
            embeds: Some(BoolOrString::Bool(true)),
            metadata: Some(BoolOrPath::Bool(false)),
        }
    }
}
