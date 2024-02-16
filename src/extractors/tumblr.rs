use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Tumblr {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<bool>,
    date_min: Option<i64>,
    date_max: Option<i64>,
    external: Option<bool>,
    inline: Option<bool>,
    offset: Option<i64>,
    original: Option<bool>,
    ratelimit: Option<String>,
    reblogs: Option<BoolOrString>,
    posts: Option<StringOrList>,
    fallback_delay: Option<f64>,
    fallback_retries: Option<i64>,
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl Tumblr {
    pub fn new() -> Self {
        return Tumblr {
            base: None,
            avatar: Some(false),
            date_min: Some(0),
            date_max: Some(0),
            external: Some(false),
            inline: Some(true),
            offset: Some(0),
            original: Some(true),
            ratelimit: Some("abort".to_string()),
            reblogs: Some(BoolOrString::Bool(true)),
            posts: Some(StringOrList::String("all".to_string())),
            fallback_delay: Some(120.0),
            fallback_retries: Some(2),
            api_key: None,
            api_secret: None,
        }
    }
}
