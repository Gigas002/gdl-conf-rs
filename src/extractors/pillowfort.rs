use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pillowfort {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reblogs: Option<bool>,
}

impl Pillowfort {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;

        return Pillowfort {
            base: Some(base),
            external: Some(false),
            inline: Some(false),
            reblogs: Some(false),
        }
    }
}
