use super::{booru::BooruExtractor, extractor::ExtractorBase};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Sankaku {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<BooruExtractor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
}

impl Sankaku {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        let mut base = ExtractorBase::new(None, None);
        base.username = username;
        base.password = password;
        let mut booru = BooruExtractor::new();
        booru.base = Some(base);

        return Sankaku {
            base: Some(booru),
            id_format: Some("numeric".to_string()),
            refresh: Some(false),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Sankakucomplex {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<bool>,
}

impl Sankakucomplex {
    pub fn new() -> Self {
        return Sankakucomplex {
            base: Some(ExtractorBase::new(None, None)),
            embeds: Some(false),
            videos: Some(true),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Idolcomplex {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<Sankaku>,
}

impl Idolcomplex {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        return Idolcomplex {
            base: Some(Sankaku::new(username, password)),
        };
    }
}
