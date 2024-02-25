use super::manga::MangaExtractor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Readcomiconline {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
}

impl Readcomiconline {
    pub fn new() -> Self {
        return Readcomiconline {
            base: Some(MangaExtractor::new()),
            captcha: Some("stop".to_string()),
            quality: Some("auto".to_string()),
        }
    }
}
