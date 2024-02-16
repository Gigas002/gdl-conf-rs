use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Weibo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livephoto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retweets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<bool>,
}

impl Weibo {
    pub fn new() -> Self {
        return Weibo {
            base: None,
            include: Some(StringOrList::String("feed".to_string())),
            livephoto: Some(true),
            retweets: Some(true),
            videos: Some(true),
        }
    }
}
