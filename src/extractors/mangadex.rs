use std::collections::HashMap;
use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mangadex {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_server: Option<String>,
    api_parameters: Option<HashMap<String, String>>,
    lang: Option<StringOrList>,
    ratings: Option<Vec<String>>,
}

impl Mangadex {
    pub fn new() -> Self {
        return Mangadex {
            base: None,
            api_server: Some("https://api.mangadex.org".to_string()),
            api_parameters:None,
            lang: None,
            ratings: Some(vec!["safe".to_string(), "suggestive".to_string(), "erotica".to_string(), "pornographic".to_string()]),
        }
    }
}
