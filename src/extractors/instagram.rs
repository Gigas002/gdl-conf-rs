use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Instagram {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_files: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_posts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<bool>,
}

impl Instagram {
    pub fn new() -> Self {
        return Instagram {
            base: None,
            api: Some("rest".to_string()),
            include: Some(StringOrList::String("posts".to_string())),
            metadata: Some(false),
            order_files: Some("asc".to_string()),
            order_posts: Some("asc".to_string()),
            previews: Some(true),
            videos: Some(true),
        }
    }
}
