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

    for entry in jwalk::WalkDir::new(&path).max_depth(1).into_iter().flatten() {
        if entry.file_type.is_dir() {
            let project_name = entry.file_name.to_string_lossy().to_string();
            let project_path = entry.path();
            let mut project_type = "";

            /* if app_state_guard.config.project_types.is_some() {
                let pts = app_state_guard.config.project_types.clone().unwrap();
                pts.iter().for_each(|pt| {
                    pt.to_owned().needed_files.and_then(|files| {
                        if files.iter().all(|p| project_path.join(p).exists()) {
                            project_type = &pt.id;
                        }
                        Some(())
                    });
                });
            } */

            if let Some(project_types) = &app_state_guard.config.project_types {
                for pt in project_types {
                    if let Some(files) = &pt.needed_files {
                        if files.iter().all(|p| project_path.join(p).exists()) {
                            project_type = &pt.id;
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
    // TODO: open projects in different ways e.g. tauri: root, src-tauri; rust: root; (stored in config)
    // retrieve project
    let mut root_path: PathBuf;

    let app_state_guard = app_state.inner().0.lock().unwrap();
    if let Some(wksps_cache) = &app_state_guard.workspaces_cache {
        let wksps: Vec<Workspace> = wksps_cache.iter().filter(|w| w.projects.clone().is_some_and(|ps| ps.iter().any(|p| p.id == id))).cloned().collect();
        if let Some(workspace) = wksps.first() {
            root_path = PathBuf::from(workspace.path.as_str());
            if let Some(projects) = &workspace.projects {
                let ps = projects.iter().filter(|p| p.id == id).cloned().collect::<Vec<Project>>();
                let project = ps.first().unwrap();
                root_path.push(project.name.as_str());
            }

            println!("{:?}", root_path);
            Command::new("cmd").arg("/c").arg("code").arg(&root_path).spawn().expect("Couldn't run vscode command!");
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
