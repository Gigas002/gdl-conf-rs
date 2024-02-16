use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Danbooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ugoira: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // this is the same as bool or str or vec<str>
    metadata: Option<BoolOrPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<StringOrInteger>,
}

impl Danbooru {
    pub fn new() -> Self {
        return Danbooru {
            base: None,
            external: Some(false),
            ugoira: Some(false),
            metadata: Some(BoolOrPath::Bool(false)),
            threshold: Some(StringOrInteger::String("auto".to_string())),
        }
    }
}
