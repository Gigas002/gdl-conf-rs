use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Deviantart {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_watch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_unwatch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journals: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mature: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<DeviantartAvatar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

impl Deviantart {
    pub fn new() -> Self {
        return Deviantart {
            base: Some(ExtractorBase::new(None, None)),
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
