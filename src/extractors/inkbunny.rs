use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Inkbunny {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderby: Option<String>,
}

impl Inkbunny {
    pub fn new() -> Self {
        return Inkbunny {
            base: None,
            orderby: Some("create_datetime".to_string()),
        }
    }
}
