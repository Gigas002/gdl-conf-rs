use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Inkbunny {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderby: Option<String>,
}

impl Inkbunny {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;

        return Inkbunny {
            base: Some(base),
            orderby: Some("create_datetime".to_string()),
        }
    }
}
