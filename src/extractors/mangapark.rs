use crate::{
    extractors::{
        extractor::ExtractorBase,
        gallery::GalleryExtractor,
        chapter::ChapterExtractor,
    },
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mangapark {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ChapterExtractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<StringOrInteger>,
}

impl Mangapark {
    pub fn new() -> Self {
        let mut gallery_extractor = GalleryExtractor::new();
        gallery_extractor.base = Some(ExtractorBase::new(Some("firefox".to_string()), None));
        let mut base = ChapterExtractor::new();
        base.base = Some(gallery_extractor);
        
        return Mangapark {
            base: Some(ChapterExtractor::new()),
            source: None,
        }
    }
}
