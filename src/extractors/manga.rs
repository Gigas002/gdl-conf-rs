use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct MangaExtractor {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_reverse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_reverse: Option<bool>,
}

impl MangaExtractor {
    pub fn new() -> Self {
        return MangaExtractor {
            base: Some(ExtractorBase::new(None, None)),
            chapter_reverse: Some(false),
            page_reverse: Some(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Batoto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Batoto {
    pub fn new() -> Self {
        return Batoto {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Dynastyscans {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Dynastyscans {
    pub fn new() -> Self {
        return Dynastyscans {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fallenangels {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Fallenangels {
    pub fn new() -> Self {
        return Fallenangels {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "hentai2read")]
pub struct Hentai2read {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Hentai2read {
    pub fn new() -> Self {
        return Hentai2read {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Hentaihere {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Hentaihere {
    pub fn new() -> Self {
        return Hentaihere {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Hiperdex {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Hiperdex {
    pub fn new() -> Self {
        return Hiperdex {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Komikcast {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Komikcast {
    pub fn new() -> Self {
        return Komikcast {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Lensdump {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Lensdump {
    pub fn new() -> Self {
        return Lensdump {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mangafox {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Mangafox {
    pub fn new() -> Self {
        return Mangafox {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mangahere {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Mangahere {
    pub fn new() -> Self {
        return Mangahere {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mangakakalot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Mangakakalot {
    pub fn new() -> Self {
        return Mangakakalot {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Manganelo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Manganelo {
    pub fn new() -> Self {
        return Manganelo {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mangaread {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Mangaread {
    pub fn new() -> Self {
        return Mangaread {
            base: Some(MangaExtractor::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mangasee {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Mangasee {
    pub fn new() -> Self {
        let mut base = MangaExtractor::new();
        base.base = Some(ExtractorBase::new(Some("firefox".to_string()), None));
        
        return Mangasee {
            base: Some(base),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Tcbscans {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<MangaExtractor>,
}

impl Tcbscans {
    pub fn new() -> Self {
        return Tcbscans {
            base: Some(MangaExtractor::new()),
        }
    }
}
