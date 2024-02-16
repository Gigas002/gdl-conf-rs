use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Lolisafe {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>,
}

impl Lolisafe {
    pub fn new() -> Self {
        return Lolisafe {
            base: None,
            domain: None,
        }
    }
}
