use crate::extractors::gallery::GalleryExtractor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ChapterExtractor {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<GalleryExtractor>,
}

impl ChapterExtractor {
    pub fn new() -> Self {
        return ChapterExtractor {
            base: Some(GalleryExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Senmanga {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ChapterExtractor>,
}

impl Senmanga {
    pub fn default() -> Self {
        return Senmanga {
            base: Some(ChapterExtractor::new()),
        }
    }
}