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
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_parameters: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Vec<String>>,
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
