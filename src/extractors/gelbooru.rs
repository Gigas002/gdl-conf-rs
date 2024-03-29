use super::booru::BooruExtractor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "gelbooru_v01")]
pub struct GelbooruV01 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<BooruExtractor>,
}

impl GelbooruV01 {
    pub fn new() -> Self {
        return GelbooruV01 {
            base: Some(BooruExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "gelbooru_v02")]
pub struct GelbooruV02 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<BooruExtractor>,
}

impl GelbooruV02 {
    pub fn new() -> Self {
        return GelbooruV02 {
            base: Some(BooruExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Gelbooru {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<GelbooruV02>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl Gelbooru {
    pub fn new() -> Self {
        return Gelbooru {
            base: Some(GelbooruV02::new()),
            api_key: None,
            user_id: None,
        }
    }
}
