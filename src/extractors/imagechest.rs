use super::gallery::GalleryExtractor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imagechest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<GalleryExtractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

impl Imagechest {
    pub fn new() -> Self {
        return Imagechest {
            base: Some(GalleryExtractor::new()),
            access_token: None,
        }
    }
}
