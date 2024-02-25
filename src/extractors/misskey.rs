use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Misskey {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renotes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<bool>,
}

impl Misskey {
    pub fn new() -> Self {
        return Misskey {
            base: Some(ExtractorBase::new(None, None)),
            access_token: None,
            renotes: Some(false),
            replies: Some(true),
        }
    }
}
