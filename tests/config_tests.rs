// TODO: why the fuck I can't reference this from serde_test_util.rs file nearby...
#[cfg(test)]
pub mod serde_test_util {
    use serde::{
        Serialize,
        de::DeserializeOwned,
    };
    use std::{
        fs::File,
        io::Write,
        error::Error,
    };

    pub fn ser<T: Serialize>(to_ser: &T, path: &str) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&to_ser)?;
        let mut file = File::create(path)?;
        let _ = file.write_all(serialized.as_bytes());

        return Ok(());
    }

    pub fn de<T: DeserializeOwned>(path: & str) -> Result<T, Box<dyn Error>> {
        let file = File::open(path)?;
        let deserialized = serde_json::from_reader::<_, T>(file)?;

        return Ok(deserialized)
    }
}

use std::collections::HashMap;
use gdl_conf_rs::{
    cache::*,
    config::*,
    downloaders::*,
    extractors::extractor::*,
    outputs::*,
    postprocessors::*,
};

#[test]
fn ser_empty_test() {
    let path = "tests/data/ser_empty.json";

    let mut config = Config::new();
    config.format_separator = None;
    config.warnings = None;

    let _ = serde_test_util::ser(&config, path).unwrap();
}

#[test]
fn de_empty_test() {
    let path = "tests/data/de_empty.json";

    let _ = serde_test_util::de::<Config>(path).unwrap();
}

#[test]
fn serde_empty_test() {
    let path = "tests/data/serde_empty.json";

    let mut config = Config::new();
    config.format_separator = None;
    config.warnings = None;

    let _ = serde_test_util::ser(&config, path).unwrap();
    let result = serde_test_util::de::<Config>(path).unwrap();

    assert_eq!(config, result);
}

#[test]
fn ser_extractor_test() {
    let path = "tests/data/ser_extractor.json";

    let extractor = Extractor::new();

    let _ = serde_test_util::ser(&extractor, path).unwrap();
}

#[test]
fn de_extractor_test() {
    let path = "tests/data/de_extractor.json";

    let _ = serde_test_util::de::<Extractor>(path).unwrap();
}

#[test]
fn serde_extractor_test() {
    let path = "tests/data/serde_extractor.json";

    let extractor = Extractor::new();

    let _ = serde_test_util::ser(&extractor, path).unwrap();
    let result = serde_test_util::de::<Extractor>(path).unwrap();

    assert_eq!(extractor, result);
}

#[test]
fn ser_cache_test() {
    let path = "tests/data/ser_cache.json";

    let cache = Cache::new();

    let _ = serde_test_util::ser(&cache, path).unwrap();
}

#[test]
fn de_cache_test() {
    let path = "tests/data/de_cache.json";

    let _ = serde_test_util::de::<Cache>(path).unwrap();
}

#[test]
fn serde_cache_test() {
    let path = "tests/data/serde_cache.json";

    let cache = Cache::new();

    let _ = serde_test_util::ser(&cache, path).unwrap();
    let result = serde_test_util::de::<Cache>(path).unwrap();

    assert_eq!(cache, result);
}

#[test]
fn ser_config_test() {

}

#[test]
fn de_config_test() {

}

// TODO: simplify test variants to use one test

#[test]
fn de_gallery_dl_conf_test() {
    let path = "tests/data/de_gallery_dl.json";

    let _ = serde_test_util::de::<Config>(path).unwrap();
}

#[test]
fn de_gallery_dl_example_conf_test() {
    let path = "tests/data/de_gallery_dl_example.json";

    let _ = serde_test_util::de::<Config>(path).unwrap();
}

// #[test]
// fn serde_config_test() {

// }

// TODO: gallery-dl repo's examples

#[test]
fn ser_downloader_test() {
    let path = "tests/data/ser_downloader.json";

    let downloader = Downloader::new();

    let _ = serde_test_util::ser(&downloader, path).unwrap();
}

#[test]
fn de_downloader_test() {
    let path = "tests/data/de_downloader.json";

    let _ = serde_test_util::de::<Downloader>(path).unwrap();
}

#[test]
fn serde_downloader_test() {
    let path = "tests/data/serde_downloader.json";

    let downloader = Downloader::new();

    let _ = serde_test_util::ser(&downloader, path).unwrap();
    let result = serde_test_util::de::<Downloader>(path).unwrap();

    assert_eq!(downloader, result);
}

#[test]
fn ser_output_test() {
    let path = "tests/data/ser_output.json";

    let output = Output::new();

    let _ = serde_test_util::ser(&output, path).unwrap();
}

#[test]
fn de_output_test() {
    let path = "tests/data/de_output.json";

    let _ = serde_test_util::de::<Output>(path).unwrap();
}

#[test]
fn serde_output_test() {
    let path = "tests/data/serde_output.json";

    let output = Output::new();

    let _ = serde_test_util::ser(&output, path).unwrap();
    let result = serde_test_util::de::<Output>(path).unwrap();

    assert_eq!(output, result);
}

#[test]
fn ser_postprocessor_test() {
    let path = "tests/data/ser_postprocessor.json";

    let zip = Postprocessors::Zip(Zip::new());
    let postprocessor = Postprocessor::new("zip".to_string(), zip);

    let _ = serde_test_util::ser(&postprocessor, path).unwrap();
}

#[test]
fn de_postprocessor_test() {
    let path = "tests/data/de_postprocessor.json";

    let _ = serde_test_util::de::<Postprocessor>(path).unwrap();
}

#[test]
fn serde_postprocessor_test() {
    let path = "tests/data/serde_postprocessor.json";

    let zip = Postprocessors::Zip(Zip::new());
    let postprocessor = Postprocessor::new("zip".to_string(), zip);

    let _ = serde_test_util::ser(&postprocessor, path).unwrap();
    let result = serde_test_util::de::<Postprocessor>(path).unwrap();

    assert_eq!(postprocessor, result);
}

#[test]
fn serde_config_test() {
    // let path = "tests/data/gdl_example.json";
    // let path = "tests/data/gdl.json";
    let path = "tests/data/config.json";
    // let path = "tests/data/gdl_ex.json";
    
    let cl = Postprocessor {
        name: Some("classify".to_string()),
        postprocessor: Some(Postprocessors::Classify(Classify::new())),
    };
    let co = Postprocessor {
        name: Some("compare".to_string()),
        postprocessor: Some(Postprocessors::Compare(Compare::new())),
    };
    let e = Postprocessor {
        name: Some("exec".to_string()),
        postprocessor: Some(Postprocessors::Exec(Exec::new())),
    };
    let m = Postprocessor {
        name: Some("metadata".to_string()),
        postprocessor: Some(Postprocessors::Metadata(Metadata::new())),
    };
    let mt = Postprocessor {
        name: Some("mtime".to_string()),
        postprocessor: Some(Postprocessors::Mtime(Mtime::new())),
    };
    let py = Postprocessor {
        name: Some("python".to_string()),
        postprocessor: Some(Postprocessors::Python(Python::new())),
    };
    let u = Postprocessor {
        name: Some("ugoira".to_string()),
        postprocessor: Some(Postprocessors::Ugoira(Ugiora::new())),
    };
    let z = Postprocessor {
        name: Some("zip".to_string()),
        postprocessor: Some(Postprocessors::Zip(Zip::new())),
    };

    let mut pps: HashMap<String, Postprocessor> = HashMap::new();
    pps.insert("cla".to_string(), cl);
    pps.insert("co".to_string(), co);
    pps.insert("exec".to_string(), e);
    pps.insert("meta".to_string(), m);
    pps.insert("mt".to_string(), mt);
    pps.insert("py".to_string(), py);
    pps.insert("ugo".to_string(), u);
    pps.insert("zi".to_string(), z);

    let extractor: Extractor = Extractor::new();

    let mut config = Config::new();
    config.postprocessor = Some(pps);
    config.cache = Some(Cache::new());
    config.downloader = Some(Downloader::new());
    config.output = Some(Output::new());
    config.extractor = Some(extractor);
    
    let _ = serde_test_util::ser(&config, path).unwrap();
    let result = serde_test_util::de::<Config>(path).unwrap();

    let path_serde = "tests/data/serde_config.json";
    let _ = serde_test_util::ser(&result, path_serde).unwrap();
    
    assert_eq!(config, result);
}
