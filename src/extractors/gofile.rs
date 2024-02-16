use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Gofile {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

impl Gofile {
    pub fn new() -> Self {
        return Gofile {
            base: None,
            api_token: None,
            website_token: None,
            recursive: Some(false),
        }
    }
}
