use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Nitter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quoted: Option<bool>,
    retweets: Option<bool>,
    videos: Option<BoolOrString>,
}

impl Nitter {
    pub fn new() -> Self {
        return Nitter {
            base: None,
            quoted: Some(false),
            retweets: Some(false),
            videos: Some(BoolOrString::Bool(true)),
        }
    }
}
