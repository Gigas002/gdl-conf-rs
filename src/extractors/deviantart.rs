use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Deviantart {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_watch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_unwatch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folders: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intermediary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    journals: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mature: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<DeviantartAvatar>,
    client_id: Option<String>,
    client_secret: Option<String>,
}

impl Deviantart {
    pub fn new() -> Self {
        return Deviantart {
            base: None,
            auto_watch: Some(false),
            auto_unwatch: Some(false),
            comments: Some(false),
            extra: Some(false),
            flat: Some(true),
            folders: Some(false),
            group: Some(BoolOrString::Bool(true)),
            include: Some(StringOrList::String("gallery".to_string())),
            intermediary: Some(true),
            journals: Some("html".to_string()),
            jwt: Some(false),
            mature: Some(true),
            metadata: Some(false),
            original: Some(BoolOrString::Bool(true)),
            pagination: Some("api".to_string()),
            public: Some(true),
            quality: Some(100),
            refresh_token: None,
            wait_min: Some(0),
            avatar: None,
            client_id: None,
            client_secret: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DeviantartAvatar {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<String>>,
}

impl DeviantartAvatar {
    pub fn new() -> Self {
        return DeviantartAvatar {
            formats: None,
        };
    }
}
