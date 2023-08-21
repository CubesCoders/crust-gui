use std::{fs::{File, OpenOptions}, io::{Read, Write}, env};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

pub struct Config {
    pub file: ConfigFile,
    pub excluded_dirs: Vec<String>,
    pub extensions: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let excluded_dirs: Vec<String> = vec![
            ".github".to_owned(),
            ".svelte-kit".to_owned(),
            ".idea".to_owned(),
            ".vscode".to_owned(),
            "node_modules".to_owned(),
        ];
        let file = ConfigFile::new();
        let mut extensions: Vec<String> = Vec::new();

        for pt in file.project_types.clone().unwrap() { // Can't fail because of the default project types
            if let Some(needed_files) = pt.needed_files {
                for needed_file in needed_files {
                    if needed_file.starts_with("!ext:") {
                        let extension = needed_file.split("!ext:").enumerate().find(|(i, _p)| i.eq(&1)).unwrap().1;
                        if !extensions.contains(&extension.to_string()) {
                            extensions.push(extension.to_string());
                        }
                    }
                }
            }
        }


        Config {file, excluded_dirs, extensions}
    }

    pub fn update(&mut self, new_file: ConfigFile) {
        let mut extensions: Vec<String> = Vec::new();

        for pt in new_file.project_types.clone().unwrap() { // Can't fail because of the default project types
            if let Some(needed_files) = pt.needed_files {
                for needed_file in needed_files {
                    if needed_file.starts_with("!ext:") {
                        let extension = needed_file.split("!ext:").enumerate().find(|(i, _p)| i.eq(&1)).unwrap().1;
                        if !extensions.contains(&extension.to_string()) {
                            extensions.push(extension.to_string());
                        }
                    }
                }
            }
        }

        self.extensions = extensions;
        self.file = new_file;
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ConfigFile {
    pub project_types: Option<Vec<ProjectType>>,
    pub run_configs: Option<Vec<RunConfig>>,
}

impl ConfigFile {
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

        let mut cfg: ConfigFile = from_str(&contents).expect("Couldn't deserialize config file!");
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
    array.push(default_project_type("0", "undef.", vec![""], ""));
    array.push(default_project_type("1", "html", vec!["!ext:html"], "#fa6b61"));
    array.push(default_project_type("2", "javascript", vec!["!ext:js"], "#fad73c"));
    array.push(default_project_type("3", "typescript", vec!["!ext:ts"], "#0074cd"));
    array.push(default_project_type("4", "react", vec!["!ext:tsx"], "#5ae4ff"));
    array.push(default_project_type("5", "svelte", vec!["!ext:svelte", "svelte.config.js"], "#fb3d00"));
    array.push(default_project_type("6", "c++", vec!["!ext:cpp"], "#6396cd"));
    array.push(default_project_type("7", "c", vec!["!ext:c"], "#283590"));
    array.push(default_project_type("8", "java", vec!["!ext:java"], "#ed4034"));
    array.push(default_project_type("9", "python", vec!["!ext:py"], "#3670a0"));
    array
}

fn default_project_type(id: &str, name: &str, needed_files: Vec<&str>, color: &str) -> ProjectType {
    let c = if color.is_empty() {
        None
    } else {
        Some(color.to_string())
    };
    ProjectType { id: id.to_string(), name: name.to_string(), needed_files: Some(needed_files.iter().map(|f| f.to_string()).collect::<Vec<String>>()), color: c, run_config_id: None }
}