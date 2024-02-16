use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Artstation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_posts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<ArtstationSearch>,
}

impl Artstation {
    pub fn new() -> Self {
        return Artstation {
            base: None,
            external: Some(false),
            max_posts: None,
            search: Some(ArtstationSearch::new())
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ArtstationSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro_first: Option<bool>,
}

impl ArtstationSearch {
    pub fn new() -> Self {
        return ArtstationSearch {
            pro_first: Some(true),
        }
    }
}
