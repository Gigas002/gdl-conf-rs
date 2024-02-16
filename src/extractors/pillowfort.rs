use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pillowfort {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reblogs: Option<bool>,
}

impl Pillowfort {
    pub fn new() -> Self {
        return Pillowfort {
            base: None,
            external: Some(false),
            inline: Some(false),
            reblogs: Some(false),
        }
    }
}
