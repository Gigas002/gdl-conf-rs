use super::booru::BooruExtractor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Zerochan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<BooruExtractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
}

impl Zerochan {
    pub fn new() -> Self {
        return Zerochan {
            base: None,
            metadata: Some(false),
        }
    }
}
