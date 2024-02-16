use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Misskey {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<String>,
    renotes: Option<bool>,
    replies: Option<bool>,
}

impl Misskey {
    pub fn new() -> Self {
        return Misskey {
            base: None,
            access_token: None,
            renotes: Some(false),
            replies: Some(true),
        }
    }
}
