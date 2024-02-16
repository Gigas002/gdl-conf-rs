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
    
    let cl = Postprocessor {
        name: "classify".to_string(),
        postprocessor: Postprocessors::Classify(Classify::new()),
    };
    let co = Postprocessor {
        name: "compare".to_string(),
        postprocessor: Postprocessors::Compare(Compare::new()),
    };
    let e = Postprocessor {
        name: "exec".to_string(),
        postprocessor: Postprocessors::Exec(Exec::new()),
    };
    let m = Postprocessor {
        name: "metadata".to_string(),
        postprocessor: Postprocessors::Metadata(Metadata::new()),
    };
    let mt = Postprocessor {
        name: "mtime".to_string(),
        postprocessor: Postprocessors::Mtime(Mtime::new()),
    };
    let py = Postprocessor {
        name: "python".to_string(),
        postprocessor: Postprocessors::Python(Python::new()),
    };
    let u = Postprocessor {
        name: "ugoira".to_string(),
        postprocessor: Postprocessors::Ugoira(Ugiora::new()),
    };
    let z = Postprocessor {
        name: "zip".to_string(),
        postprocessor: Postprocessors::Zip(Zip::new()),
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

    let mut extractor: Extractor = Extractor::new();
    let mut e_base = ExtractorBase::default();
    e_base.postprocessors = Some(vec!["huing".to_string()]);
    e_base.postprocessor_options = Some(vec!["pohuing".to_string()]);
    extractor.base = Some(e_base);

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
