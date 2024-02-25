use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Steamgriddb {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epilepsy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_types: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_fake_png: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "static")]
    pub static_asstes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged: Option<bool>,
}

impl Steamgriddb {
    pub fn new() -> Self {
        return Steamgriddb {
            base: Some(ExtractorBase::new(None, None)),
            animated: Some(true),
            epilepsy: Some(true),
            dimensions: Some(StringOrList::String("all".to_string())),
            file_types: Some(StringOrList::String("all".to_string())),
            download_fake_png: Some(true),
            humor: Some(true),
            languages: Some(StringOrList::String("all".to_string())),
            nsfw: Some(true),
            sort: Some("score_desc".to_string()),
            static_asstes: Some(true),
            styles: Some(StringOrList::String("all".to_string())),
            untagged: Some(true),
        }
    }
}
