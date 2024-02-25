use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Kemonoparty {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorites: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_posts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<BoolOrString>,
}

impl Kemonoparty {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;

        return Kemonoparty {
            base: Some(base),
            comments: Some(false),
            duplicates: Some(false),
            dms: Some(false),
            favorites: Some("artist".to_string()),
            files: Some(vec!["attachments".to_string(), "file".to_string(), "inline".to_string()]),
            max_posts: None,
            metadata: Some(false),
            revisions: Some(BoolOrString::Bool(false)),
        }
    }
}
