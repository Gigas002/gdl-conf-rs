use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Steamgriddb {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animated: Option<bool>,
    epilepsy: Option<bool>,
    dimensions: Option<StringOrList>,
    file_types: Option<StringOrList>,
    download_fake_png: Option<bool>,
    humor: Option<bool>,
    languages: Option<StringOrList>,
    nsfw: Option<bool>,
    sort: Option<String>,
    #[serde(rename = "static")]
    static_asstes: Option<bool>,
    styles: Option<StringOrList>,
    untagged: Option<bool>,
}

impl Steamgriddb {
    pub fn new() -> Self {
        return Steamgriddb {
            base: None,
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
