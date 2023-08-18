import { invoke } from "@tauri-apps/api";
import { derived, writable } from "svelte/store";

export const workspaces = writable<App.Workspace[]>([]);
let workspaces_loaded = writable(false);
export const config = writable<App.Config>();
let config_loaded = writable(false);
export const alert = writable<App.Message | null>(null);

export async function fetch_workspaces() {
    const ret = unwrap_result<App.Workspace[]>(await invoke("get_workspaces"), "loading workspaces");
    if (typeof(ret) == "boolean") return;
    workspaces.set(ret);
    workspaces_loaded.set(true);
}

export async function reindex_workspace(id: string, path: string) {
    let del = unwrap_result(await invoke("delete_workspace", { id: id }), "deleting workspace");
    if (!del) return;
    launch_alert("default", "Successfully deleted Workspace!", `${path}`);
    add_workspace(path);
}

export async function delete_workspace(id: string) {
    let del = unwrap_result(await invoke("delete_workspace", { id: id }), "deleting workspace");
    if (!del) return;
    launch_alert("default", "Successfully deleted Workspace!", "");
    fetch_workspaces();
}


export async function add_workspace(path: string) {
    let add = unwrap_result(await invoke("add_workspace", { path: path }), "adding workspace");
    if (!add) return;
    launch_alert("default", "Successfully added Workspace!", `${path}`);
    fetch_workspaces();
}

export async function open_project(id: String) {
    const ret = unwrap_result(await invoke("open_project", { id: id }), "opening project");
    if (!ret) return;
    launch_alert("default", "Successfully opened project", "");
}

export async function fetch_config() {
    const ret = unwrap_result<App.Config>(await invoke('get_config'), "loading config");
    if (typeof(ret) == "boolean") return;
    config.set(ret);
    config_loaded.set(true);
}

export async function save_config(config: App.Config) {
    const ret = unwrap_result(await invoke('save_config', { config: config }), "saving config");
    if (!ret) return;
    launch_alert("default", "Successfully saved configuartion", "", 2000);
}

export async function get_project_type_id() {
    return await invoke("get_uuid");
}

export function is_default_pt(id: string) {
    return id.length < 2;
}

export function launch_alert(type: "default" | "destructive" | null | undefined, title: string, message: string, duration?: number) {
    alert.set({type, title, content: message});
    setTimeout(() => {
        alert.set(null);
    }, duration ?? 4000)
}

export const hasWorkspaces = derived(workspaces, ($workspaces) => $workspaces !== null);
export const hasConfig = derived(config, ($config) => $config !== null);

export const storeLoaded = workspaces_loaded && config_loaded;

function unwrap_result<Type>(call: App.FunctionResult, errorPlace: String): Type | boolean {
    if (call.Error) {
        console.log(call);
        launch_alert("destructive", `Error on ${errorPlace}`, call.Error!);
        return false;
    }

    return call.Success as Type ?? true;
}

fetch_workspaces();
fetch_config();