use super::gallery::GalleryExtractor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Hitomi {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<GalleryExtractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl Hitomi {
    pub fn new() -> Self {
        return Hitomi {
            base: Some(GalleryExtractor::new()),
            format: Some("webp".to_string()),
        }
    }
}
