use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Hentaifoundry {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StringOrList>,
}

impl Hentaifoundry {
    pub fn new() -> Self {
        return Hentaifoundry {
            base: Some(ExtractorBase::new(None, None)),
            include: Some(StringOrList::String("pictures".to_string())),
        }
    }
}
