use serde::{Deserialize, Serialize};
use serde_yaml::{from_str, Value};
use std::fs;
use yaml_rust::{yaml, YamlEmitter, YamlLoader};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config_TiKV {
    pdaddrs: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config_Http {
    port: u16,
    addr: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    tikv: Config_TiKV,
    http: Config_Http,
}

impl Config_TiKV {
    pub fn default() -> Self {
        Self {
            pdaddrs: vec!["127.0.0.1:2379".to_string()],
        }
    }
}

impl Config_Http {
    pub fn default() -> Self {
        Self {
            port: 3000,
            addr: "0.0.0.0".to_string(),
        }
    }
}

impl Config {
    pub fn default() -> Self {
        Self {
            tikv: Config_TiKV::default(),
            http: Config_Http::default(),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("Config.yml").expect("Something went wrong reading the file");

    let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    println!("{:?}", docs);
    println!("{:?}", doc);
    println!("{:?}", doc["tikv"]["pdaddrs"]);

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&doc["tikv"]["pdaddrs"]).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
    let s = r#"{ A: 65, B: 66, C: 67 }"#;
    let object = from_str::<Value>(s).unwrap();
    let x = object.get("A").unwrap();
    println!("{:?}", x);

    let yml = from_str::<Config>(contents.as_str()).unwrap();
    println!("yml is: {:?}", yml);

    let config = Config::default();
    let ymlstr = serde_yaml::to_string(&config).unwrap();
    fs::write("config_default.yml", ymlstr);
}
