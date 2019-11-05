use std::fs::File;
use std::io::prelude::*;

use serde_yaml;

pub fn load_yaml(file_path: &str) -> serde_yaml::Value {
    let file = File::open(file_path).expect(&format!("Unable to open file {:?}", file_path));
    let v: serde_yaml::Value = serde_yaml::from_reader(file).expect("Could not parse yaml");
    v
}

pub fn file_to_str(file_path: &std::path::Path) -> String {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Could not read file!");
    contents
}
