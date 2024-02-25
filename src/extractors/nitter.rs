use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Nitter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quoted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retweets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<BoolOrString>,
}

impl Nitter {
    pub fn new() -> Self {
        return Nitter {
            base: Some(ExtractorBase::new(None, None)),
            quoted: Some(false),
            retweets: Some(false),
            videos: Some(BoolOrString::Bool(true)),
        }
    }
}
