use crate::{
    extractors::danbooru::Danbooru,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct E621 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<Danbooru>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // this is the same as bool or str or vec<str>
    pub metadata: Option<BoolOrPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<StringOrInteger>,
}

impl E621 {
    pub fn new() -> Self {
        return E621 {
            base: None,
            metadata: Some(BoolOrPath::Bool(false)),
            threshold: Some(StringOrInteger::String("auto".to_string())),
        }
    }
}
