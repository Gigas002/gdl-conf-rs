use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Cyberdrop {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>,
}

impl Cyberdrop {
    pub fn new() -> Self {
        return Cyberdrop {
            base: None,
            domain: None,
        }
    }
}
