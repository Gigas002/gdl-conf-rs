use super::booru::BooruExtractor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Twibooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<BooruExtractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<i64>,
}

impl Twibooru {
    pub fn new() -> Self {
        return Twibooru {
            base: Some(BooruExtractor::new()),
            api_key: None,
            filter: Some(2),
        }
    }
}
