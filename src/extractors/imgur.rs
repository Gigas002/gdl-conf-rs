use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imgur {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp4: Option<BoolOrString>,
}

impl Imgur {
    pub fn new() -> Self {
        return Imgur {
            base: Some(ExtractorBase::new(None, None)),
            client_id: None,
            mp4: Some(BoolOrString::Bool(true)),
        }
    }
}
