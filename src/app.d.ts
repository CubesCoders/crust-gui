// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
declare namespace App {
    // TypeScript type for the Workspace struct
    interface Workspace {
        id: string,
        path: string,
        name: string,
        projects: Project[] | undefined
    }

    // TypeScript type for the Project struct
    interface Project {
        id: string,
        name: string,
        workspace_id: string,
        metadata: string
    }

    interface Config {
        project_types?: ProjectType[],
        run_configs?: RunConfig[],
    }

    interface ProjectType {
        id: string,
        name: string,
        needed_files?: string[],
        color?: string,
        run_config_id?: string,
    }

    interface RunConfig {
        id: string,
        name: string,
        commands: string,
    }

    enum MessageType {
        Default,
        Danger
    }

    interface Message {
        type: "default" | "destructive" | null | undefined,
        title: string,
        content: string,
    }
}