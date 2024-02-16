use std::collections::HashMap;
use super::enums::*;
use serde::{
    Deserialize,
    Deserializer,
};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Classify {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<HashMap<String, Vec<String>>>,
}

impl Classify {
    pub fn new() -> Self {
        return Classify {
            mapping: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Compare {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shallow: Option<bool>,
}

impl Compare {
    pub fn new() -> Self {
        return Compare {
            action: Some("replace".to_string()),
            equal: Some("null".to_string()),
            shallow: Some(false),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Exec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "async")]
    pub asynchro: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

impl Exec {
    pub fn new() -> Self {
        return Exec {
            archive: None,
            asynchro: Some(false),
            command: None,
            event: Some("after".to_string()),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<ListOrHashMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_format: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascii: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<StringOrInteger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separators: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<bool>,
}

impl Metadata {
    pub fn new() -> Self {
        let separators = vec![", ".to_string(), ": ".to_string()];

        return Metadata {
            mode: Some("json".to_string()),
            filename: None,
            directory: Some(".".to_string()),
            extension: Some("json".to_string()),
            extension_format: None,
            event: Some("file".to_string()),
            fields: None,
            content_format: None,
            ascii: Some(false),
            indent: Some(StringOrInteger::Integer(4)),
            separators: Some(separators),
            sort: Some(false),
            open: Some("w".to_string()),
            encoding: Some("utf-8".to_string()),
            private: Some(false),
            skip: Some(false),
            archive: None,
            mtime: Some(false),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mtime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Mtime {
    pub fn new() -> Self {
        return Mtime {
            event: Some("file".to_string()),
            key: Some("date".to_string()),
            value: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Python {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
}

impl Python {
    pub fn new() -> Self {
        return Python {
            archive: None,
            event: Some("file".to_string()),
            function: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Ugiora {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ffmpeg_args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ffmpeg_demuxer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ffmpeg_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mkvmerge_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ffmpeg_output: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ffmpeg_twopass: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libx264_prevent_odd: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_last_frame: Option<bool>,
}

impl Ugiora {
    pub fn new() -> Self {
        return Ugiora {
            extension: Some("webm".to_string()),
            ffmpeg_args: None,
            ffmpeg_demuxer: Some("auto".to_string()),
            ffmpeg_location: Some("ffmpeg".to_string()),
            mkvmerge_location: Some("mkvmerge".to_string()),
            ffmpeg_output: Some(BoolOrString::String("error".to_string())),
            ffmpeg_twopass: Some(false),
            framerate: Some("auto".to_string()),
            keep_files: Some(false),
            libx264_prevent_odd: Some(true),
            mtime: Some(true),
            repeat_last_frame: Some(true),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Zip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Path>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_files: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl Zip {
    pub fn new() -> Self {
        return Zip {
            compression: Some("store".to_string()),
            extension: Some("zip".to_string()),
            files: None,
            keep_files: Some(false),
            mode: Some("default".to_string()),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Postprocessor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub postprocessor: Option<Postprocessors>,
}

impl Postprocessor {
    pub fn new(name: String, postprocessor: Postprocessors) -> Self {
        return Postprocessor {
            name: Some(name),
            postprocessor: Some(postprocessor),
        }
    }
}

impl <'de> Deserialize<'de> for Postprocessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>,
    {
        let postprocessor_value = Value::deserialize(deserializer)?;
        let postprocessor_map = postprocessor_value.as_object().unwrap();

        let mut name: Option<String> = None;
        let mut postprocessor: Option<Postprocessors> = None;
            
        for postprocessor_map_pair in postprocessor_map {
            let key = postprocessor_map_pair.0;
            let value = postprocessor_map_pair.1;

            if key.to_string().eq("name") {
                let postprocessor_type = value.as_str().unwrap();

                postprocessor = match postprocessor_type {
                    "classify" => Some(Postprocessors::Classify(serde_json::from_value::<Classify>(postprocessor_value.clone()).unwrap())),
                    "compare" => Some(Postprocessors::Compare(serde_json::from_value::<Compare>(postprocessor_value.clone()).unwrap())),
                    "exec" => Some(Postprocessors::Exec(serde_json::from_value::<Exec>(postprocessor_value.clone()).unwrap())),
                    "metadata" => Some(Postprocessors::Metadata(serde_json::from_value::<Metadata>(postprocessor_value.clone()).unwrap())),
                    "mtime" => Some(Postprocessors::Mtime(serde_json::from_value::<Mtime>(postprocessor_value.clone()).unwrap())),
                    "python" => Some(Postprocessors::Python(serde_json::from_value::<Python>(postprocessor_value.clone()).unwrap())),
                    "ugoira" => Some(Postprocessors::Ugoira(serde_json::from_value::<Ugiora>(postprocessor_value.clone()).unwrap())),
                    "zip" => Some(Postprocessors::Zip(serde_json::from_value::<Zip>(postprocessor_value.clone()).unwrap())),
                    _ => None
                };

                name = Some(postprocessor_type.to_string());
                break;
            }
        }

        return Ok(Postprocessor::new(name.unwrap(), postprocessor.unwrap()));
    }
}
