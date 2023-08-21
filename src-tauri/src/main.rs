#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env, sync::{Arc, Mutex}, path::PathBuf, process::Command, collections::HashMap};

use config::{ConfigFile, Config};
use db::{DB, Workspace, Project, hash};
use sysinfo::{System, SystemExt, DiskExt};
use tauri::State;
use uuid::Uuid;
use serde::Serialize;

mod db;
mod config;

struct AppState(Arc<Mutex<MyAppState>>);

struct MyAppState {
    db: DB,
    workspaces_cache: Option<Vec<Workspace>>,
    config: Config,
}

impl MyAppState {
    fn new() -> Self {
        MyAppState { db: DB::new(), workspaces_cache: None, config: Config::new() }
    }
}

#[derive(Debug, Serialize)]
enum FunctionResult<T> {
    Success(T),
    Error(String),
}

#[tauri::command]
fn get_drives() -> Vec<String> {
    let sys = System::new_all();

    sys.disks().iter().map(|d| if sys.name().unwrap().as_str() != "Windows" {
        d.mount_point().to_string_lossy().to_string()
    } else {
        d.mount_point().to_string_lossy().to_string().split_at(2).0.to_string() // Remove double slash from mount point
    }).collect()
}

#[tauri::command]
fn get_workspaces(app_state: State<AppState>) -> FunctionResult<Vec<Workspace>> {
    let mut app_state_guard = match app_state.inner().0.lock() {
        Ok(lock) => lock,
        Err(_) => return FunctionResult::Error("Failed to acquire lock on app_state".to_owned()),
    };

    let mut workspaces = app_state_guard.db.select_workspaces();
    for workspace in &mut workspaces {
        workspace.projects = Some(app_state_guard.db.select_projects(workspace.id.clone()));
    }
    app_state_guard.workspaces_cache = Some(workspaces.clone());
    FunctionResult::Success(workspaces)
}

#[tauri::command]
fn add_workspace(path: String, app_state: State<AppState>) -> FunctionResult<usize> {
    let workspace: Workspace = if path.ends_with("/") {
        let path_segments: Vec<&str> = path.split("/").collect();
        path_segments[..path_segments.len() - 1].join("/").into()
    } else {
        path.clone().into()
    };

    let mut app_state_guard = match app_state.inner().0.lock() {
        Ok(lock) => lock,
        Err(_) => return FunctionResult::Error("Failed to acquire lock on app_state".to_owned()),
    };
    
    let mut projects: Vec<Project> = vec![];

    for entry in jwalk::WalkDir::new(&path).min_depth(1).max_depth(1).into_iter().flatten() {
        if entry.file_type.is_dir() { // Projects
            let project_name = entry.file_name.to_string_lossy().to_string();
            let project_path = entry.path();
            let mut project_type_options: Vec<&str> = vec![];

            let mut paths: Vec<String> = Vec::new(); // Path needed
            let mut extensions: HashMap<String, usize> = HashMap::new(); // get count of available extensions

            let excluded_dirs = app_state_guard.config.excluded_dirs.clone();

            for entry in jwalk::WalkDir::new(&project_path).min_depth(1).process_read_dir(
                move |_depth, _path, _read_dir_state, children| {
                    children.iter_mut().for_each(
                        |dir_entry_result| {
                            if let Ok(dir_entry) = dir_entry_result {
                                if excluded_dirs.iter().any(|excluded_dir| dir_entry.path().to_string_lossy().to_string().contains(excluded_dir)) {
                                    dir_entry.read_children_path = None;
                                }
                            }
                        }
                    );
            }).into_iter().flatten() {
                paths.push(entry.path().to_string_lossy().to_string());
                if entry.file_type.is_file() {
                    if let Some(ext_wrapped) = entry.path().extension() {
                        let ext = ext_wrapped.to_string_lossy().to_string();
                        if app_state_guard.config.extensions.contains(&ext) {
                            if extensions.contains_key(&ext) {
                                extensions.insert(ext.clone(), extensions.get(&ext).unwrap() + 1);
                            } else {
                                extensions.insert(ext, 1);
                            }
                        }
                    }
                }
            }

            println!("{:#?}", extensions);

            if let Some(project_types) = &app_state_guard.config.file.project_types {
                for pt in project_types {
                    if let Some(files) = &pt.needed_files {
                        if files.iter().all(|p| {
                            let first = project_path.join(p).exists();
                            let mut second = false;
                            if p.contains("!ext:") {
                                let extension = p.splitn(2, "!ext:").last().unwrap();
                                second = extensions.contains_key(&extension.to_string());
                            }
                            first || second
                        }) {
                            project_type_options.push(&pt.id);
                        }
                    }
                }
            }

            project_type_options.reverse();

            if !project_type_options.is_empty() {
                projects.push((
                    hash(project_path.to_string_lossy().to_string()).to_string(),
                    workspace.id.clone(),
                    project_name,
                    project_type_options.first().unwrap().to_string()
                ).into());
            }
        }
    }

    for project in &projects {
        app_state_guard.db.insert_project(project.clone());
    }
    
    if !app_state_guard.db.insert_workspace(workspace) {
        return FunctionResult::Error("Failed to insert workspace into the database".to_owned());
    }

    FunctionResult::Success(projects.len())
}

#[tauri::command]
fn delete_workspace(id: String, app_state: State<AppState>) -> FunctionResult<()> {
    let mut app_state_guard = match app_state.inner().0.lock() {
        Ok(lock) => lock,
        Err(_) => return FunctionResult::Error("Failed to acquire lock on app_state".to_owned()),
    };
    if false == app_state_guard.db.delete_workspace(&id) {
        return FunctionResult::Error(format!("Failed to delete workspace (id:{})", id));
    }
    FunctionResult::Success(())
}

// TODO: add reindex_workspace (NOT delete_workspace and add_workspace!) function

#[tauri::command]
fn delete_project(id: String, app_state: State<AppState>) -> FunctionResult<()> {
    let mut app_state_guard = match app_state.inner().0.lock() {
        Ok(lock) => lock,
        Err(_) => return FunctionResult::Error("Failed to acquire lock on app_state".to_owned()),
    };
    if false == app_state_guard.db.delete_project(&id) {
        return FunctionResult::Error(format!("Failed to delete project (id:{})", id));
    }
    FunctionResult::Success(())
}

#[tauri::command]
fn open_project(id: String, app_state: State<AppState>) -> FunctionResult<()> {
    // retrieve project
    let mut root_path: PathBuf;

    let app_state_guard = match app_state.inner().0.lock() {
        Ok(lock) => lock,
        Err(_) => return FunctionResult::Error("Failed to acquire lock on app_state".to_owned()),
    };

    if let Some(wksps_cache) = &app_state_guard.workspaces_cache {
        let wksps: Vec<Workspace> = wksps_cache.iter().filter(|w| w.projects.clone().is_some_and(|ps| ps.iter().any(|p| p.id == id))).cloned().collect();
        if let Some(workspace) = wksps.first() {
            root_path = PathBuf::from(workspace.path.as_str());
            let mut cmds = "";

            if let Some(projects) = &workspace.projects {
                let ps = projects.iter().filter(|p| p.id == id).cloned().collect::<Vec<Project>>();
                let project = match ps.first() {
                    Some(project) => project,
                    None => return FunctionResult::Error(format!("No project found for with id = {}", id)),
                };
                root_path.push(project.name.as_str());

                if let Some(project_types) = &app_state_guard.config.file.project_types {
                    if let Some(project_type) = project_types.iter().find(|pt| pt.id == project.metadata) {
                        if let Some(run_configs) = &app_state_guard.config.file.run_configs {
                            if let Some(run_config_id) = &project_type.run_config_id {
                                if let Some(run_cfg) = run_configs.iter().find(|cfg| cfg.id.as_str() == run_config_id.as_str()) {
                                    cmds = run_cfg.commands.as_str();
                                }
                            }
                        }
                    }
                }
            }

            if cmds == "" {
                if cfg!(target_os = "windows") { 
                    if let Err(e) = Command::new("cmd").arg("/c").arg("explorer").arg(&root_path).spawn() {
                        return FunctionResult::Error(format!("Couldn't run project on default config: {}", e.to_string()))
                    }
                } else {
                    if let Err(e) = Command::new("sh").arg("-c").arg("open").arg(&root_path).spawn() {
                        return FunctionResult::Error(format!("Couldn't run project on default config: {}", e.to_string()))
                    } // TODO: testing this (especially on mac)
                }
            } else {
                for cmd_line in cmds.split("\n") {
                    if cfg!(target_os = "windows") { 
                        if let Err(e) = Command::new("cmd").arg("/c").args(cmd_line.replace("$PPATH", &root_path.to_string_lossy().to_string()).split(" ").collect::<Vec<&str>>()).spawn() {
                            return FunctionResult::Error(format!("Couldn't run project on custom config (Command: {}): {}", cmd_line, e.to_string()))
                        }
                    } else {
                        if let Err(e) = Command::new("sh").arg("-c").arg(cmd_line.replace("$PPATH", &root_path.to_string_lossy().to_string())).spawn() {
                            return FunctionResult::Error(format!("Couldn't run project on custom config (Command: {}): {}", cmd_line, e.to_string()))
                        }
                    }
                }
            }
        }
    }

    FunctionResult::Success(())
}

#[tauri::command]
fn get_config(app_state: State<AppState>) -> FunctionResult<ConfigFile> {
    let app_state_guard = match app_state.inner().0.lock() {
        Ok(lock) => lock,
        Err(_) => return FunctionResult::Error("Failed to acquire lock on app_state".to_owned()),
    };
    FunctionResult::Success(app_state_guard.config.file.clone())
}

#[tauri::command]
fn update_config(config: ConfigFile, app_state: State<AppState>) -> FunctionResult<()> {
    let mut app_state_guard = match app_state.inner().0.lock() {
        Ok(lock) => lock,
        Err(_) => return FunctionResult::Error("Failed to acquire lock on app_state".to_owned()),
    };
    app_state_guard.config.update(config);
    FunctionResult::Success(())
}

#[tauri::command]
fn exit(app_state: State<AppState>, window: tauri::Window) {
    println!("Closing....");
    let app_state_guard = app_state.inner().0.lock().unwrap();
    app_state_guard.config.file.save();

    let _ = window.close();
}

#[tauri::command]
fn get_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .manage(AppState(Arc::new(Mutex::new(MyAppState::new()))))
        .invoke_handler(tauri::generate_handler![get_drives, get_workspaces, add_workspace, delete_project, delete_workspace, open_project, get_config, update_config, exit, get_uuid])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
