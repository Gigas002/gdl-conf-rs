use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Skeb {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_requests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnails: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<SkebSearch>,
}

impl Skeb {
    pub fn new() -> Self {
        return Skeb {
            base: Some(ExtractorBase::new(None, None)),
            article: Some(false),
            sent_requests: Some(false),
            thumbnails: Some(false),
            search: Some(SkebSearch::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct SkebSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<StringOrList>,
}

impl SkebSearch {
    pub fn new() -> Self {
        return SkebSearch {
            filters: Some(StringOrList::List(vec!["genre:art".to_string(), "genre:voice".to_string(), "genre:novel".to_string(),
                                                  "genre:video".to_string(), "genre:music".to_string(), "genre:correction".to_string()])),
        };
    }
}
