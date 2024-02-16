use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Postmill {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    save_link_post_body: Option<bool>,
}

impl Postmill {
    pub fn new() -> Self {
        return Postmill {
            base: None,
            save_link_post_body: Some(false),
        }
    }
}
