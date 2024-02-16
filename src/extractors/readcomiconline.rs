use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Readcomiconline {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
}

impl Readcomiconline {
    pub fn new() -> Self {
        return Readcomiconline {
            base: None,
            captcha: Some("stop".to_string()),
            quality: Some("auto".to_string()),
        }
    }
}