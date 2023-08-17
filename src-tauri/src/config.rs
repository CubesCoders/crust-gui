use std::{fs::{File, OpenOptions}, io::{Read, Write}, env};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    pub project_types: Option<Vec<ProjectType>>,
    pub run_configs: Option<Vec<RunConfig>>,
}

impl Config {
    pub fn new() -> Self {
        let mut exe_dir = env::current_exe().expect("Failed to get exe directory!");
        exe_dir.pop();
        exe_dir.push("config.json");
        let mut file = OpenOptions::new().read(true).write(true).create_new(true).open(exe_dir.clone()).unwrap_or_else(|_| File::open(exe_dir.clone()).unwrap());

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Couldn't read config file!");

        if contents.is_empty() {
            contents = "{}".to_string();
            file.write_all(contents.as_bytes()).expect("Couldn't write empty config!");
        }

        let mut cfg: Config = from_str(&contents).expect("Couldn't deserialize config file!");
        let mut array = get_default_project_types();
        array.append(&mut cfg.project_types.unwrap_or_default());
        cfg.project_types = Some(array);

        cfg
    }

    pub fn save(&self) {
        let mut exe_dir = env::current_exe().expect("Failed to get exe directory!");
        exe_dir.pop();
        exe_dir.push("config.json");
        let mut file = File::create(exe_dir).unwrap();

        let mut cfg = self.clone();
        cfg.project_types = Some(cfg.project_types.unwrap().iter().filter(|pt| pt.id.len() > 2).cloned().collect::<Vec<ProjectType>>());

        file.write_all(to_string(&cfg).unwrap().as_bytes()).expect("Couldn't write to file!");
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ProjectType {
    pub id: String,
    pub name: String,
    pub needed_files: Option<Vec<String>>,
    pub color: Option<String>,
    pub run_config_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RunConfig {
    pub id: String,
    pub name: String,
    pub commands: String,
}

fn get_default_project_types() -> Vec<ProjectType> {
    let mut array: Vec<ProjectType> = vec![];
    array.push(ProjectType { id: "0".to_owned(), name: "undef.".to_owned(), needed_files: Some(vec!["".to_owned()]), color: None, run_config_id: None });
    array
}