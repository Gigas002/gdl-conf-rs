use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Lolisafe {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl Lolisafe {
    pub fn new() -> Self {
        return Lolisafe {
            base: Some(ExtractorBase::new(None, None)),
            domain: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Bunkr {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<Lolisafe>,
}

impl Bunkr {
    pub fn new() -> Self {
        return Bunkr {
            base: Some(Lolisafe::new()),
        }
    }
}
