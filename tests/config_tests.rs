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
    outputs::*,
    postprocessors::*,
    extractors::extractor::*,
};

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
