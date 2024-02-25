use super::{
    booru::BooruExtractor,
    extractor::ExtractorBase
};

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
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;
        let mut booru = BooruExtractor::new();
        booru.base = Some(base);

        return Zerochan {
            base: Some(booru),
            metadata: Some(false),
        }
    }
}
