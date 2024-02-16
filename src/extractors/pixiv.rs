use crate::{
    extractors::extractor::ExtractorBase,
    enums::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pixiv {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<StringOrList>,
    refresh_token: Option<String>,
    embeds: Option<bool>,
    novel: Option<PixivNovel>,
    metadata: Option<bool>,
    metadata_bookmark: Option<bool>,
    work: Option<PixivWork>,
    tags: Option<String>,
    ugoira: Option<bool>,
    max_posts: Option<i64>,
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
    pub related: Option<bool>,
}

impl PixivWork {
    pub fn new() -> Self {
        return PixivWork {
            related: Some(false),
        };
    }
}
