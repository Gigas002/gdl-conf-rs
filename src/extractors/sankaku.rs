use super::extractor::ExtractorBase;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Sankaku {
    #[serde(flatten)]
    pub base: ExtractorBase,
    // TODO: enum
    pub id_format: String,
    pub refresh: bool,
}

impl Sankaku {
    pub fn new() -> Self {
        return Sankaku {
            base: ExtractorBase::new(None, None),
            id_format: "numeric".to_string(),
            refresh: false
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SankakuComplex {
    #[serde(flatten)]
    pub base: ExtractorBase,
    pub embeds: bool,
    pub videos: bool,
}

impl SankakuComplex {
    pub fn new() -> Self {
        return SankakuComplex {
            base: ExtractorBase::new(None, None),
            embeds: false,
            videos: true,
        }
    }
}
