use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Nijie {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StringOrList>,
}

impl Nijie {
    pub fn new(username: &str, password: &str) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = Some(username.to_string());
        base.password = Some(password.to_string());

        return Nijie {
            base: Some(base),
            include: Some(StringOrList::String("illustration,doujin".to_string())),
        }
    }
}
