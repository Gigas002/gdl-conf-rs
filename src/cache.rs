use super::enums::Path;
use std::env::consts;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Cache {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Path>,
}

impl Cache {
    pub fn default() -> Self {
        let path = match consts::OS {
            "windows" => "~",
            _ => "~/.cache",
        };

        return Cache {
            file: Some(Path::String(format!("{path}/gallery-dl/cache.sqlite3"))),
        };
    }

    pub fn new() -> Self {
        return Cache {
            file: None,
        }
    }
}
