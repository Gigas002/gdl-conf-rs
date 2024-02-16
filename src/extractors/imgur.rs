use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imgur {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    mp4: Option<BoolOrString>,
}

impl Imgur {
    pub fn new() -> Self {
        return Imgur {
            base: None,
            client_id: None,
            mp4: Some(BoolOrString::Bool(true)),
        }
    }
}
