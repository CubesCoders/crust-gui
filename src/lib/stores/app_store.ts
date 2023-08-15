import { invoke } from "@tauri-apps/api";
import { derived, writable } from "svelte/store";

export const workspaces = writable<App.Workspace[]>([]);
let workspaces_loaded = writable(false);
export const config = writable<App.Config>();
let config_loaded = writable(false);
export const alert = writable<App.Message | null>(null);

export async function fetch_workspaces() {
    const ret = await invoke("get_workspaces");
    workspaces.set(ret as App.Workspace[]);
    workspaces_loaded.set(true);
}

export async function reindex_workspace(id: string, path: string) {
    await invoke("delete_workspace", { id: id });
    await invoke("add_workspace", { path: path });
    console.log(id + " " + path);
    fetch_workspaces();
}

export async function delete_workspace(id: string) {
    invoke("delete_workspace", { id: id });
    fetch_workspaces();
}


export async function add_workspace(path: string) {
    await invoke("add_workspace", { path: path });
    fetch_workspaces();
}

// TODO: add open function

export async function fetch_config() {
    const ret = await invoke('get_config');
    config.set(ret as App.Config);
    config_loaded.set(true);
}

export async function save_config(config: App.Config) {
    await invoke('save_config', { config: config });
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

fetch_workspaces();
fetch_config();