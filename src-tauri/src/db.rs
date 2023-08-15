use rusqlite::Connection;
use std::{fs::create_dir, env, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};
use serde::Serialize;

pub struct DB {
    sql: Connection
}

impl DB {
    pub fn new() -> Self {
        let mut exe_dir = env::current_exe().expect("Failed to get exe directory");
        exe_dir.pop();
        exe_dir.push("data");

        if !exe_dir.exists() {
            create_dir(&exe_dir).expect("Failed to create data directory");
        }
        exe_dir.push("data.sqlite3");

        let conn = Connection::open(exe_dir).expect("Failed to open SQLite database");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
                id      TEXT NOT NULL PRIMARY KEY,
                path    TEXT NOT NULL,
                name    TEXT NOT NULL
            )",
            ()
        ).expect("Couldn't create workspace table in sqlite");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id              TEXT NOT NULL PRIMARY KEY,
                workspace_id    TEXT NOT NULL,
                name            TEXT NOT NULL,
                metadata        TEXT NOT NULL
            )",
            ()
        ).unwrap();

        DB { sql: conn }
    }

    pub fn insert_workspace(&mut self, workspace: Workspace) -> bool {
        self.sql.execute(
            "INSERT OR IGNORE INTO workspaces (id, path, name) VALUES (?1, ?2, ?3)",
            &[&workspace.id, &workspace.path, &workspace.name]
        ).is_ok()
    }

    pub fn insert_project(&mut self, project: Project) -> bool {
        self.sql.execute(
            "INSERT OR IGNORE INTO projects (id, workspace_id, name, metadata) VALUES (?1, ?2, ?3, ?4)",
            &[&project.id, &project.workspace_id, &project.name, &project.metadata]
        ).is_ok()
    }

    pub fn delete_workspace(&mut self, id: &str) -> bool {
        self.sql.execute(
            "DELETE FROM workspaces WHERE id = ?1",
            &[id]
        ).is_ok() && self.sql.execute(
            "DELETE FROM projects WHERE workspace_id = ?1",
            &[id]
        ).is_ok()
    }

    pub fn delete_project(&mut self, id: &str) -> bool {
        self.sql.execute(
            "DELETE FROM projects WHERE id = ?1",
            &[id]
        ).is_ok()
    }

    pub fn select_workspaces(&mut self) -> Vec<Workspace> {
        let mut stmt = self.sql.prepare("SELECT id, path, name FROM workspaces").unwrap();
        let workspace_iter = stmt.query_map((), |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        }).unwrap();

        let mut workspaces: Vec<Workspace> = Vec::new();
        for workspace in workspace_iter {
            workspaces.push(workspace.unwrap().into());
        }
        workspaces
    }

    pub fn select_projects(&mut self, workspace_id: String) -> Vec<Project> {
        let mut stmt = self.sql.prepare("SELECT id, workspace_id, name, metadata FROM projects WHERE workspace_id = ?1").unwrap();
        let project_iter = stmt.query_map([&workspace_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        }).unwrap();

        let mut projects: Vec<Project> = Vec::new();
        for project in project_iter {
            projects.push(project.unwrap().into());
        }
        projects
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Workspace {
    pub id: String,
    pub path: String,
    name: String,
    pub projects: Option<Vec<Project>>,
}

impl From<String> for Workspace {
    fn from(path: String) -> Self {
        let id = hash(&path);
        let path_binding = path.clone();
        let name = path_binding.split("\\").last().unwrap_or_default();
        Workspace { id: id.to_string(), path: path, name: name.to_string(), projects: None }
    }
}

impl From<(String, String, String)> for Workspace {
    fn from(array: (String, String, String)) -> Self {
        Workspace { id: array.0, path: array.1, name: array.2, projects: None }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    workspace_id: String,
    metadata: String,
}

impl From<String> for Project {
    fn from(path: String) -> Self {
        let id = hash(&path);
        let path_binding = path.clone();
        let name = path_binding.split("\\").last().unwrap_or_default();
        let workspace = path_binding.split("\\").enumerate().filter(|(i, _)| *i != path_binding.split("\\").count() - 1).map(|(_, g)| g).collect::<Vec<&str>>().join("\\");
        Project { id: id.to_string(), name: name.to_string(), workspace_id: hash(workspace).to_string(), metadata: "".to_owned() }
    }
}

impl From<(String, String, String, String)> for Project {
    fn from(array: (String, String, String, String)) -> Self {
        Project { id: array.0, name: array.2, workspace_id: array.1, metadata: array.3 }
    }
}

pub fn hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}