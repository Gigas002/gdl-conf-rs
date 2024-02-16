use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Behance {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<String>>,
}

impl Behance {
    pub fn new() -> Self {
        return Behance {
            base: None,
            modules: Some(vec!["image".to_string(), "video".to_string(), "mediacollection".to_string(), "embed".to_string()]),
        }
    }
}
