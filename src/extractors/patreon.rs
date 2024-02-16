use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Patreon {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

impl Patreon {
    pub fn new() -> Self {
        return Patreon {
            base: None,
            files: Some(vec!["images".to_string(), "image_large".to_string(), "attachments".to_string(), "postfile".to_string(), "content".to_string()])
        }
    }
}
