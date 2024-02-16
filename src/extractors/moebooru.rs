use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Moebooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool: Option<MoebooruPool>,
}

impl Moebooru {
    pub fn new() -> Self {
        return Moebooru {
            base: None,
            pool: Some(MoebooruPool::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct MoebooruPool {
    pub metadata: Option<bool>,
}

impl MoebooruPool {
    pub fn new() -> Self {
        return MoebooruPool {
            metadata: Some(false),
        };
    }
}
