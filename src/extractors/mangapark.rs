use crate::{
    extractors::chapter::ChapterExtractor,
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
        return Mangapark {
            base: None,
            source: None,
        }
    }
}
