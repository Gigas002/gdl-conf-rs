use super::booru::BooruExtractor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Moebooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<BooruExtractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<MoebooruPool>,
}

impl Moebooru {
    pub fn new() -> Self {
        return Moebooru {
            base: Some(BooruExtractor::new()),
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "3dbooru")]
pub struct _3dbooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<Moebooru>,
}

impl _3dbooru {
    pub fn new() -> Self {
        return _3dbooru {
            base: Some(Moebooru::new()),
        }
    }
}
