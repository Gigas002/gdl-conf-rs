use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Artstation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_posts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<ArtstationSearch>,
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
