use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mastodon {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reblogs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_posts: Option<bool>,
}

impl Mastodon {
    pub fn new() -> Self {
        return Mastodon {
            base: None,
            access_token: None,
            reblogs: Some(false),
            replies: Some(true),
            text_posts: Some(false),
        }
    }
}
