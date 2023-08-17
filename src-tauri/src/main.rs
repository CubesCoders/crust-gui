#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env, sync::{Arc, Mutex}, path::PathBuf, process::Command};

use config::Config;
use db::{DB, Workspace, Project, hash};
use sysinfo::{System, SystemExt, DiskExt};
use tauri::State;

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
fn get_workspaces(app_state: State<AppState>) -> Vec<Workspace> {
    let mut app_state_guard = app_state.inner().0.lock().unwrap();
    let mut workspaces = app_state_guard.db.select_workspaces();
    for workspace in &mut workspaces {
        workspace.projects = Some(app_state_guard.db.select_projects(workspace.id.clone()));
    }
    app_state_guard.workspaces_cache = Some(workspaces.clone());
    workspaces
}

#[tauri::command]
fn add_workspace(path: String, app_state: State<AppState>) {
    let workspace: Workspace = if path.ends_with("/") { 
        path.split("/").enumerate().filter(|(i, _)| *i != path.split("/").count() -1).map(|(_, g)| g).collect::<Vec<&str>>().join("/").into()
    } else {
        path.clone().into()
    };

    let mut app_state_guard = app_state.inner().0.lock().unwrap();
    
    let mut projects: Vec<Project> = vec![];

    for entry in jwalk::WalkDir::new(&path).min_depth(1).max_depth(1).into_iter().flatten() {
        if entry.file_type.is_dir() {
            let project_name = entry.file_name.to_string_lossy().to_string();
            let project_path = entry.path();
            let mut project_type = "";

            if let Some(project_types) = &app_state_guard.config.project_types {
                for pt in project_types {
                    if let Some(files) = &pt.needed_files {
                        if files.iter().all(|p| project_path.join(p).exists()) {
                            project_type = &pt.id; // TODO: deny undefined
                            break;  // Exit the loop if a matching project type is found
                        }
                    }
                }
            }

            if !project_type.is_empty() {
                projects.push((
                    hash(project_path.to_string_lossy().to_string()).to_string(),
                    workspace.id.clone(),
                    project_name,
                    project_type.to_owned()
                ).into());
            }
        }
    }

    projects.iter().for_each(|p| {
        app_state_guard.db.insert_project(p.clone());
    });
    app_state_guard.db.insert_workspace(workspace);
}

#[tauri::command]
fn delete_workspace(id: String, app_state: State<AppState>) -> bool {
    let mut app_state_guard = app_state.inner().0.lock().unwrap();
    app_state_guard.db.delete_workspace(&id)
}

// TODO: add reindex_workspace (NOT delete_workspace and add_workspace!) function

#[tauri::command]
fn delete_project(id: String, app_state: State<AppState>) -> bool {
    let mut app_state_guard = app_state.inner().0.lock().unwrap();
    app_state_guard.db.delete_project(&id)
}

#[tauri::command]
fn open_project(id: String, app_state: State<AppState>) {
    // retrieve project
    let mut root_path: PathBuf;

    let app_state_guard = app_state.inner().0.lock().unwrap();
    if let Some(wksps_cache) = &app_state_guard.workspaces_cache {
        let wksps: Vec<Workspace> = wksps_cache.iter().filter(|w| w.projects.clone().is_some_and(|ps| ps.iter().any(|p| p.id == id))).cloned().collect();
        if let Some(workspace) = wksps.first() {
            root_path = PathBuf::from(workspace.path.as_str());
            let mut cmds = "";

            if let Some(projects) = &workspace.projects {
                let ps = projects.iter().filter(|p| p.id == id).cloned().collect::<Vec<Project>>();
                let project = ps.first().unwrap();
                root_path.push(project.name.as_str());

                if let Some(project_types) = &app_state_guard.config.project_types {
                    if let Some(project_type) = project_types.iter().find(|pt| pt.id == project.metadata) {
                        if let Some(run_configs) = &app_state_guard.config.run_configs {
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
                    Command::new("cmd").arg("/c").arg("explorer").arg(&root_path).spawn().expect("Couldn't run explorer command!");
                } else {
                    Command::new("sh").arg("-c").arg("open").arg(&root_path).spawn().expect("Couldn't run explorer command!"); // TODO: testing this (especially on mac)
                }
            } else {
                for cmd_line in cmds.split("\n") {
                    if cfg!(target_os = "windows") { 
                        Command::new("cmd").arg("/c").arg(cmd_line.replace("$PPATH", &root_path.to_string_lossy().to_string())).spawn().expect("Couldn't run run_config command!");
                    } else {
                        Command::new("sh").arg("-c").arg(cmd_line.replace("$PPATH", &root_path.to_string_lossy().to_string())).spawn().expect("Couldn't run run_config command!");
                    }
                }
            }
        }
    }
}

#[tauri::command]
fn get_config(app_state: State<AppState>) -> Config {
    let app_state_guard = app_state.inner().0.lock().unwrap();
    app_state_guard.config.clone()
}

#[tauri::command]
fn save_config(config: Config, app_state: State<AppState>) {
    let mut app_state_guard = app_state.inner().0.lock().unwrap();
    app_state_guard.config = config;
}

#[tauri::command]
fn exit(app_state: State<AppState>, window: tauri::Window) {
    println!("Closing....");
    let app_state_guard = app_state.inner().0.lock().unwrap();
    app_state_guard.config.save();

    let _ = window.close();
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .manage(AppState(Arc::new(Mutex::new(MyAppState::new()))))
        .invoke_handler(tauri::generate_handler![get_drives, get_workspaces, add_workspace, delete_project, delete_workspace, open_project, get_config, save_config, exit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
