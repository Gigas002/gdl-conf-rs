use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Tumblr {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_min: Option<StringOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_max: Option<StringOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reblogs: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_delay: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_retries: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_secret: Option<String>,
}

impl Tumblr {
    pub fn new() -> Self {
        return Tumblr {
            base: None,
            avatar: Some(false),
            date_min: Some(StringOrInteger::Integer(0)),
            date_max: None,
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
