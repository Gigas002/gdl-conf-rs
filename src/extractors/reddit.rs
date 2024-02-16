use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Reddit {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morecomments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_min: Option<StringOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_max: Option<StringOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursion: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl Reddit {
    pub fn new() -> Self {
        return Reddit {
            base: None,
            comments: Some(0),
            morecomments: Some(false),
            date_min: Some(StringOrInteger::Integer(0)),
            date_max: Some(StringOrInteger::Integer(253402210800)),
            id_min: None,
            id_max: None,
            previews: Some(true),
            recursion: Some(0),
            refresh_token: None,
            videos: Some(BoolOrString::Bool(true)),
            client_id: None,
            user_agent: None,
        }
    }
}
