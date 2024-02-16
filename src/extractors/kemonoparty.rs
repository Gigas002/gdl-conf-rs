use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Kemonoparty {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<bool>,
    duplicates: Option<bool>,
    dms: Option<bool>,
    favorites: Option<String>,
    files: Option<Vec<String>>,
    max_posts: Option<i64>,
    metadata: Option<bool>,
    revisions: Option<BoolOrString>,
}

impl Kemonoparty {
    pub fn new() -> Self {
        return Kemonoparty {
            base: None,
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
