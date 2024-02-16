use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pixiv {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub novel: Option<PixivNovel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_bookmark: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work: Option<PixivWork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ugoira: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_posts: Option<i64>,
}

impl Pixiv {
    pub fn new() -> Self {
        return Pixiv {
            base: None,
            include: Some(StringOrList::String("artworks".to_string())),
            refresh_token: None,
            embeds: Some(false),
            novel: Some(PixivNovel::new()),
            metadata: Some(false),
            metadata_bookmark: Some(false),
            work: Some(PixivWork::new()),
            tags: Some("japanese".to_string()),
            ugoira: Some(true),
            max_posts: Some(0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PixivNovel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_series: Option<bool>,
}

impl PixivNovel {
    pub fn new() -> Self {
        return PixivNovel {
            full_series: Some(false),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PixivWork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<bool>,
}

impl PixivWork {
    pub fn new() -> Self {
        return PixivWork {
            related: Some(false),
        };
    }
}
