use crate::extractors::lolisafe::Lolisafe;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Cyberdrop {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<Lolisafe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl Cyberdrop {
    pub fn new() -> Self {
        return Cyberdrop {
            base: None,
            domain: None,
        }
    }
}
