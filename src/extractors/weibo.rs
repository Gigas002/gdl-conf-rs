use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Weibo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<StringOrList>,
    livephoto: Option<bool>,
    retweets: Option<bool>,
    videos: Option<bool>,
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
