use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Reddit {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<i64>,
    morecomments: Option<bool>,
    // date_min: Option<Date>,
    // date_max: Option<Date>,
    id_min: Option<String>,
    id_max: Option<String>,
    previews: Option<bool>,
    recursion: Option<i64>,
    refresh_token: Option<String>,
    videos: Option<BoolOrString>,
    client_id: Option<String>,
    user_agent: Option<String>,
}

impl Reddit {
    pub fn new() -> Self {
        return Reddit {
            base: None,
            comments: Some(0),
            morecomments: Some(false),
            // date_min: Some(0),
            // date_max: Some(253402210800),
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
