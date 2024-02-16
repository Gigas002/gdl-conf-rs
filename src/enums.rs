use std::collections::HashMap;
use super::{
    outputs::*,
    postprocessors::*,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    String(String)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum BoolOrPath {
    Bool(bool),
    Path(Path),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrHashMap {
    String(String),
    Object(HashMap<String, String>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Directory {
    String(String),
    Object(HashMap<String, Vec<String>>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Cookie {
    Path(Path),
    Object(HashMap<String, String>),
    List(Vec<Option<String>>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum SourceAddress {
    String(String),
    List(Vec<StringOrInteger>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrInteger {
    String(String),
    Integer(i64),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Path {
    String(String),
    List(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringOrList {
    String(String),
    List(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ListOrHashMap {
    List(Vec<String>),
    Object(HashMap<String, String>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PathOrLogConf {
    Path(Path),
    LogConf(LoggingConfiguration)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ListStringOrListPostprocessor {
    String(Vec<String>),
    Postprocessor(Vec<Postprocessor>),
}
