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

use gdl_conf_rs::{
    cache::*,
    config::*,
    downloaders::*,
    extractors::extractor::*,
    outputs::*,
    postprocessors::*,
};

// TODO: empty classes, nulls, specific extractors, alt values, simplify to use one test for each struct

#[test]
fn ser_extractor_test() {
    let path = "tests/data/ser_extractor.json";

    let extractor = Extractor::default();

    let _ = serde_test_util::ser(&extractor, path).unwrap();
}

#[test]
fn de_extractor_test() {
    let test_cases = [
        ("tests/data/de_extractor.json", "de_extractor"),
        ("tests/data/de_extractor_empty.json", "de_extractor_empty"),
    ];

    for (path, case_name) in &test_cases {
        match serde_test_util::de::<Config>(path) {
            Ok(_) => println!("{} passed successfully!", case_name),
            Err(err) => panic!("{} failed: {:?}", case_name, err),
        }
    }
}

#[test]
fn serde_extractor_test() {
    let path = "tests/data/serde_extractor.json";

    let extractor = Extractor::default();

    let _ = serde_test_util::ser(&extractor, path).unwrap();
    let result = serde_test_util::de::<Extractor>(path).unwrap();

    assert_eq!(extractor, result);
}

#[test]
fn ser_cache_test() {
    let path = "tests/data/ser_cache.json";

    let cache = Cache::default();

    let _ = serde_test_util::ser(&cache, path).unwrap();
}

#[test]
fn de_cache_test() {
    let test_cases = [
        ("tests/data/de_cache.json", "de_cache"),
        ("tests/data/de_cache_empty.json", "de_cache_empty"),
    ];

    for (path, case_name) in &test_cases {
        match serde_test_util::de::<Config>(path) {
            Ok(_) => println!("{case_name} passed successfully!"),
            Err(err) => panic!("{case_name} failed: {:?}", err),
        }
    }
}

#[test]
fn serde_cache_test() {
    let path = "tests/data/serde_cache.json";

    let cache = Cache::default();

    let _ = serde_test_util::ser(&cache, path).unwrap();
    let result = serde_test_util::de::<Cache>(path).unwrap();

    assert_eq!(cache, result);
}

#[test]
fn ser_config_empty_test() {
    let path = "tests/data/ser_config_empty.json";

    let config = Config::new();

    let _ = serde_test_util::ser(&config, path).unwrap();
}

#[test]
fn de_config_empty_test() {
    let path = "tests/data/de_config_empty.json";

    let _ = serde_test_util::de::<Config>(path).unwrap();
}

#[test]
fn serde_config_empty_test() {
    let path = "tests/data/serde_config_empty.json";

    let config = Config::new();

    let _ = serde_test_util::ser(&config, path).unwrap();
    let result = serde_test_util::de::<Config>(path).unwrap();

    assert_eq!(config, result);
}

#[test]
fn ser_config_test() {

}

#[test]
fn de_config_test() {

}

#[test]
fn serde_config_test() {
    let path = "tests/data/serde_config.json";
    
    let config = Config::default();
    
    let _ = serde_test_util::ser(&config, path).unwrap();
    let result = serde_test_util::de::<Config>(path).unwrap();

    assert_eq!(config, result);
}

// TODO: gallery-dl repo's examples

#[test]
fn de_config_gallery_dl_conf_test() {
    let path = "tests/data/de_gallery_dl.json";

    let _ = serde_test_util::de::<Config>(path).unwrap();
}

#[test]
fn de_config_gallery_dl_example_conf_test() {
    let path = "tests/data/de_gallery_dl_example.json";

    let _ = serde_test_util::de::<Config>(path).unwrap();
}

#[test]
fn ser_downloader_test() {
    let path = "tests/data/ser_downloader.json";

    let downloader = Downloader::default();

    let _ = serde_test_util::ser(&downloader, path).unwrap();
}

#[test]
fn de_downloader_test() {
    let test_cases = [
        ("tests/data/de_downloader.json", "de_downloader"),
        ("tests/data/de_downloader_empty.json", "de_downloader_empty"),
    ];

    for (path, case_name) in &test_cases {
        match serde_test_util::de::<Config>(path) {
            Ok(_) => println!("{} passed successfully!", case_name),
            Err(err) => panic!("{} failed: {:?}", case_name, err),
        }
    }
}

#[test]
fn serde_downloader_test() {
    let path = "tests/data/serde_downloader.json";

    let downloader = Downloader::default();

    let _ = serde_test_util::ser(&downloader, path).unwrap();
    let result = serde_test_util::de::<Downloader>(path).unwrap();

    assert_eq!(downloader, result);
}

#[test]
fn ser_output_test() {
    let path = "tests/data/ser_output.json";

    let output = Output::default();

    let _ = serde_test_util::ser(&output, path).unwrap();
}

#[test]
fn de_output_test() {
    let test_cases = [
        ("tests/data/de_output.json", "de_output"),
        ("tests/data/de_output_empty.json", "de_output_empty"),
    ];

    for (path, case_name) in &test_cases {
        match serde_test_util::de::<Config>(path) {
            Ok(_) => println!("{} passed successfully!", case_name),
            Err(err) => panic!("{} failed: {:?}", case_name, err),
        }
    }
}

#[test]
fn serde_output_test() {
    let path = "tests/data/serde_output.json";

    let output = Output::default();

    let _ = serde_test_util::ser(&output, path).unwrap();
    let result = serde_test_util::de::<Output>(path).unwrap();

    assert_eq!(output, result);
}

#[test]
fn ser_postprocessor_test() {
    let path = "tests/data/ser_postprocessor.json";

    let postprocessor = RootPostprocessor::default();

    let _ = serde_test_util::ser(&postprocessor, path).unwrap();
}

#[test]
fn de_postprocessor_test() {
    let test_cases = [
        ("tests/data/de_postprocessor.json", "de_postprocessor"),
        ("tests/data/de_postprocessor_empty.json", "de_postprocessor_empty"),
    ];

    for (path, case_name) in &test_cases {
        match serde_test_util::de::<Config>(path) {
            Ok(_) => println!("{} passed successfully!", case_name),
            Err(err) => panic!("{} failed: {:?}", case_name, err),
        }
    }
}

#[test]
fn serde_postprocessor_test() {
    let path = "tests/data/serde_postprocessor.json";

    let postprocessor = RootPostprocessor::default();

    let _ = serde_test_util::ser(&postprocessor, path).unwrap();
    let result = serde_test_util::de::<RootPostprocessor>(path).unwrap();

    assert_eq!(postprocessor, result);
}
