use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Instagram {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api: Option<String>,
    include: Option<StringOrList>,
    metadata: Option<bool>,
    order_files: Option<String>,
    order_posts: Option<String>,
    previews: Option<bool>,
    videos: Option<bool>,
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
