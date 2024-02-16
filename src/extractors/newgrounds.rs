use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Newgrounds {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flash: Option<bool>,
    format: Option<String>,
    include: Option<StringOrList>
}

impl Newgrounds {
    pub fn new() -> Self {
        return Newgrounds {
            base: None,
            flash: Some(true),
            format: Some("original".to_string()),
            include: Some(StringOrList::String("art".to_string())),
        }
    }
}
