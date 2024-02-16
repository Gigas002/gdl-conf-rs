use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Flickr {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exif: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<BoolOrPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    videos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_max: Option<StringOrInteger>,
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl Flickr {
    pub fn new() -> Self {
        return Flickr {
            base: None,
            access_token: None,
            access_token_secret: None,
            exif: Some(false),
            metadata: Some(BoolOrPath::Bool(false)),
            videos: Some(true),
            size_max: None,
            api_key: None,
            api_secret: None,
        }
    }
}
