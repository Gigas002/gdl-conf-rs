use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Flickr {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exif: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BoolOrPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_max: Option<StringOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_secret: Option<String>,
}

impl Flickr {
    pub fn new() -> Self {
        return Flickr {
            base: Some(ExtractorBase::new(None, None)),
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
