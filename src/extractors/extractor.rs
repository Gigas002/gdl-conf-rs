use std::collections::HashMap;
use crate::{
    enums::*,
    extractors::sankaku::*,
};

/// Base properties for all extractors
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ExtractorBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<StringOrHashMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<Directory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_directory: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_directory: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_parent: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_skip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_restrict: Option<StringOrHashMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_replace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_remove: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_strip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_extended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_map: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_extractor: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_request: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netrc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Cookie>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies_update: Option<BoolOrPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<StringOrHashMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<SourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls12: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<HashMap<String, StringOrInteger>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // String only
    pub keywords_default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_extractor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_http: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_transfer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_pragma: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // only list of strings
    pub postprocessors: Option<Vec<StringOrPostprocessor>>,
    // Needs example
    // only list of strings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postprocessor_options: Option<Vec<StringOrPostprocessor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_codes: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<BoolOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_range: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_filter: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_filter: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_pages: Option<BoolOrString>,
}

impl ExtractorBase {
    pub fn default() -> Self {
        let mut extension_map: HashMap<String, String> = HashMap::new();
        extension_map.insert("jpeg".to_string(), "jpg".to_string());
        extension_map.insert("jpe".to_string(), "jpg".to_string());
        extension_map.insert("jfif".to_string(), "jpg".to_string());
        extension_map.insert("jif".to_string(), "jpg".to_string());
        extension_map.insert("jfi".to_string(), "jpg".to_string());

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("User-Agent".to_string(), "<extractor.*.user-agent>".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.5".to_string());
        headers.insert("Accept-Encoding".to_string(), "gzip, deflate".to_string());
        headers.insert("Referer".to_string(), "<extractor.*.referer>".to_string());

        return ExtractorBase {
            filename: None,
            directory: None,
            base_directory: Some(Path::String("./gallery-dl/".to_string())),
            parent_directory: Some(false),
            metadata_parent: Some(BoolOrString::Bool(false)),
            parent_skip: Some(false),
            path_restrict: Some(StringOrHashMap::String("auto".to_string())),
            path_replace: Some("_".to_string()),
            path_remove: Some("\\u0000-\\u001f\\u007f".to_string()),
            path_strip: Some("auto".to_string()),
            path_extended: Some(true),
            extension_map: Some(extension_map),
            skip: Some(BoolOrString::Bool(true)),
            sleep: Some(Duration::Float(0.0)),
            sleep_extractor: Some(Duration::Float(0.0)),
            sleep_request: Some(Duration::Float(0.0)),
            username: None,
            password: None,
            netrc: Some(false),
            cookies: None,
            cookies_update: Some(BoolOrPath::Bool(true)),
            proxy: None,
            source_address: None,
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/115.0".to_string()),
            browser: Some("firefox".to_string()),
            referer: Some(BoolOrString::Bool(true)),
            headers: Some(headers),
            ciphers: None,
            tls12: Some(true),
            keywords: None,
            keywords_default: Some("None".to_string()),
            metadata_url: None,
            metadata_path: None,
            metadata_extractor: None,
            metadata_http: None,
            metadata_version: None,
            category_transfer: None,
            blacklist: None,
            whitelist: None,
            archive: None,
            archive_format: None,
            archive_prefix: Some("{category}".to_string()),
            archive_pragma: None,
            postprocessors: None,
            postprocessor_options: None,
            retries: Some(4),
            retry_codes: None,
            timeout: Some(30.0),
            verify: Some(BoolOrString::Bool(true)),
            download: Some(true),
            fallback: Some(true),
            image_range: None,
            chapter_range: None,
            image_filter: None,
            chapter_filter: None,
            image_unique: Some(false),
            chapter_unique: Some(false),
            date_format: Some("%Y-%m-%dT%H:%M:%S".to_string()),
            write_pages: Some(BoolOrString::Bool(false)),
        }
    }

    pub fn new(browser: Option<String>, tls12: Option<bool>) -> Self {
        return ExtractorBase {
            filename: None,
            directory: None,
            base_directory: None,
            parent_directory: None,
            metadata_parent: None,
            parent_skip: None,
            path_restrict: None,
            path_replace: None,
            path_remove: None,
            path_strip: None,
            path_extended: None,
            extension_map: None,
            skip: None,
            sleep: None,
            sleep_extractor:None,
            sleep_request: None,
            username: None,
            password: None,
            netrc: None,
            cookies: None,
            cookies_update: None,
            proxy: None,
            source_address: None,
            user_agent: None,
            browser: browser,
            referer: None,
            headers: None,
            ciphers: None,
            tls12: tls12,
            keywords: None,
            keywords_default: None,
            metadata_url: None,
            metadata_path: None,
            metadata_extractor: None,
            metadata_http: None,
            metadata_version: None,
            category_transfer: None,
            blacklist: None,
            whitelist: None,
            archive: None,
            archive_format: None,
            archive_prefix: None,
            archive_pragma: None,
            postprocessors: None,
            postprocessor_options: None,
            retries: None,
            retry_codes: None,
            timeout: None,
            verify: None,
            download: None,
            fallback: None,
            image_range: None,
            chapter_range: None,
            image_filter: None,
            chapter_filter: None,
            image_unique: None,
            chapter_unique: None,
            date_format: None,
            write_pages: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Extractor {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_sources: Option<Vec<Path>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(flatten)]
    // pub categories: Option<HashMap<String, Extractors>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sankaku: Option<Sankaku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sankakucomplex: Option<Sankakucomplex>,
}

impl Extractor {
    pub fn new() -> Self {
        // let mut categories: HashMap<String, Extractors> = HashMap::new();
        // categories.insert("sankaku".to_string(), Extractors::Sankaku(Sankaku::new()));
        // categories.insert("sankakucomplex".to_string(), Extractors::SankakuComplex(SankakuComplex::new()));

        return Extractor {
            base: Some(ExtractorBase::default()),
            modules: None,
            module_sources: None,
            // categories: Some(categories),
            
            sankaku: Some(Sankaku::new()),
            sankakucomplex: Some(Sankakucomplex::new()),
        };
    }
}

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// #[serde(untagged)]
// pub enum Extractors {
//     Sankaku(Sankaku),
//     SankakuComplex(SankakuComplex)
// }

// Children of BaseExtractor

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "2ch")]
pub struct _2ch {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "2chan")]
pub struct _2chan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "2chen")]
pub struct _2chen {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "35photo")]
pub struct _35photo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "4archive")]
pub struct _4archive {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "4chan")]
pub struct _4chan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "4chanarchives")]
pub struct _4chanarchives {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "500px")]
pub struct _500px {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "8chan")]
pub struct _8chan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "8muses")]
pub struct _8muses {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Bluesky {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Chevereto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Desktopography {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Directlink {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Erome {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fanleaks {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fantia {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fapachi {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Fapello {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Foolfuuka {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Foolslide {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Hatenablog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Hotleak {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imagebam {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imagefap {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imagehosts {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imgbb {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Imgbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Itchio {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Jschan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Kabeuchi {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Keenspot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Lexica {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Lightroom {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Livedoor {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Lynxchan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Mangoxo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Myportfolio {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Nozomi {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Photovogue {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Picarto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Piczel {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pixnet {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Poipiku {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Poringa {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Pornhub {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Seiga {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Sexcom {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Shimmie2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Shopify {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Slickpic {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Soundgasm {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Speakerdeck {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Subscribestar {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Tapas {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Toyhouse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Uploadir {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Urlshortener {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vanillarock {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vichan {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vipergifs {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Vk {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Wallpapercave {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Warosu {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Webmshare {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Wikiart {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Wikimedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Xhamster {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub base: Option<ExtractorBase>,
}
