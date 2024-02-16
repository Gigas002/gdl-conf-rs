use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct MangaExtractor {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_reverse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_reverse: Option<bool>,
}

impl MangaExtractor {
    pub fn new() -> Self {
        return MangaExtractor {
            base: None,
            chapter_reverse: Some(false),
            page_reverse: Some(false),
        }
    }
}