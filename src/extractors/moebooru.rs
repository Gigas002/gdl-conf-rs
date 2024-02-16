use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Moebooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<MoebooruPool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
}

impl MoebooruPool {
    pub fn new() -> Self {
        return MoebooruPool {
            metadata: Some(false),
        };
    }
}
