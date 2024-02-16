use std::collections::HashMap;
use super::{
    outputs::*,
    postprocessors::*,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    String(String)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum BoolOrPath {
    Bool(bool),
    Path(Path),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrHashMap {
    String(String),
    Object(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Directory {
    String(String),
    Object(HashMap<String, Vec<String>>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Cookie {
    Path(Path),
    Object(HashMap<String, String>),
    List(Vec<Option<String>>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum SourceAddress {
    String(String),
    List(Vec<StringOrInteger>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrInteger {
    String(String),
    Integer(i64),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Path {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrList {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum ListOrHashMap {
    List(Vec<String>),
    Object(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum PathOrLogConf {
    Path(Path),
    LogConf(LoggingConfiguration)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrPostprocessor {
    String(String),
    Postprocessor(Postprocessor),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Duration {
    Float(f64),
    FloatList(Vec<f64>),
    String(String),
}
