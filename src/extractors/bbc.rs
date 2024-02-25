use super::gallery::GalleryExtractor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Bbc {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<GalleryExtractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl Bbc {
    pub fn new() -> Self {
        return Bbc {
            base: None,
            width: Some(1920),
        }
    }
}
