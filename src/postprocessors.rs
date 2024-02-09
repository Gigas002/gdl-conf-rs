use std::collections::HashMap;
use super::enums::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Classify {
    pub mapping: HashMap<String, Vec<String>>,
}

impl Classify {
    pub fn new() -> Self {
        return Classify {
            mapping: HashMap::new(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Compare {
    pub action: String,
    pub equal: String,
    pub shallow: bool,
}

impl Compare {
    pub fn new() -> Self {
        return Compare {
            action: "replace".to_string(),
            equal: "null".to_string(),
            shallow: false,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Exec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<Path>,
    #[serde(rename = "async")]
    pub asynchro: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<StringOrList>,
    pub event: String,
}

impl Exec {
    pub fn new() -> Self {
        return Exec {
            archive: None,
            asynchro: false,
            command: None,
            event: "after".to_string(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub directory: String,
    pub extension: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_format: Option<String>,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<ListOrHashMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_format: Option<StringOrList>,
    pub ascii: bool,
    pub indent: StringOrInteger,
    pub separators: Vec<String>,
    pub sort: bool,
    pub open: String,
    pub encoding: String,
    pub private: bool,
    pub skip: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<Path>,
    pub mtime: bool,
}

impl Metadata {
    pub fn new() -> Self {
        let separators = vec![", ".to_string(), ": ".to_string()];

        return Metadata {
            mode: "json".to_string(),
            filename: None,
            directory: ".".to_string(),
            extension: "json".to_string(),
            extension_format: None,
            event: "file".to_string(),
            fields: None,
            content_format: None,
            ascii: false,
            indent: StringOrInteger::Integer(4),
            separators,
            sort: false,
            open: "w".to_string(),
            encoding: "utf-8".to_string(),
            private: false,
            skip: false,
            archive: None,
            mtime: false,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Mtime {
    pub event: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Mtime {
    pub fn new() -> Self {
        return Mtime {
            event: "file".to_string(),
            key: "date".to_string(),
            value: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Python {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<Path>,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
}

impl Python {
    pub fn new() -> Self {
        return Python {
            archive: None,
            event: "file".to_string(),
            function: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Ugiora {
    pub extension: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ffmpeg_args: Option<Vec<String>>,
    pub ffmpeg_demuxer: String,
    pub ffmpeg_location: String,
    pub mkvmerge_location: String,
    pub ffmpeg_output: BoolOrString,
    pub ffmpeg_twopass: bool,
    pub framerate: String,
    pub keep_files: bool,
    pub libx264_prevent_odd: bool,
    pub mtime: bool,
    pub repeat_last_frame: bool,
}

impl Ugiora {
    pub fn new() -> Self {
        return Ugiora {
            extension: "webm".to_string(),
            ffmpeg_args: None,
            ffmpeg_demuxer: "auto".to_string(),
            ffmpeg_location: "ffmpeg".to_string(),
            mkvmerge_location: "mkvmerge".to_string(),
            ffmpeg_output: BoolOrString::String("error".to_string()),
            ffmpeg_twopass: false,
            framerate: "auto".to_string(),
            keep_files: false,
            libx264_prevent_odd: true,
            mtime: true,
            repeat_last_frame: true,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Zip {
    pub compression: String,
    pub extension: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Path>>,
    pub keep_files: bool,
    pub mode: String,
}

impl Zip {
    pub fn new() -> Self {
        return Zip {
            compression: "store".to_string(),
            extension: "zip".to_string(),
            files: None,
            keep_files: false,
            mode: "default".to_string(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Postprocessors {
    Classify(Classify),
    Compare(Compare),
    Exec(Exec),
    Metadata(Metadata),
    Mtime(Mtime),
    Python(Python),
    Ugoira(Ugiora),
    Zip(Zip),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Postprocessor {
    pub name: String,
    #[serde(flatten)]
    pub postprocessor: Postprocessors,
}
