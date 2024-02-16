use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Photobucket {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subalbums: Option<bool>,
}

impl Photobucket {
    pub fn new() -> Self {
        return Photobucket {
            base: None,
            subalbums: Some(true),
        }
    }
}
