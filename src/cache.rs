use super::enums::Path;
use std::env::consts;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Cache {
    pub file: Path,
}

impl Cache {
    pub fn new() -> Self {
        let path = match consts::OS {
            "windows" => "~",
            _ => "~/.cache",
        };

        return Cache {
            file: Path::String(format!("{path}/gallery-dl/cache.sqlite3")),
        };
    }
}
