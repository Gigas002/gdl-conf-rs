use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Photobucket {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subalbums: Option<bool>,
}

impl Photobucket {
    pub fn new() -> Self {
        return Photobucket {
            base: Some(ExtractorBase::new(None, None)),
            subalbums: Some(true),
        }
    }
}
