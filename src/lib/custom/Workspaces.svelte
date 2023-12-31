<script lang="ts">
    import Button from "$components/ui/button/Button.svelte";
    import {
        config,
        delete_workspace,
        get_project_from_id,
        open_project,
        reindex_workspace,
        storeLoaded,
        workspaces,
    } from "../stores/app_store";
    import { onDestroy } from "svelte";

    export let editable = false;
    export let selected_project_id = "-1";
    export let search_str: string = "";
    export let selectable_projects: App.Project[] = [];

    let last_opened = $config.last_opened
        ? get_project_from_id($config.last_opened, $workspaces)
        : undefined;

    let unsubscirbe = config.subscribe((value) => {
        last_opened = value.last_opened ? get_project_from_id(value.last_opened, $workspaces) : undefined;
    });

    onDestroy(unsubscirbe);

    // console.log($config)

    function filter_search(workspaces: App.Workspace[], search_value: string) {
        let workspaces_copy: App.Workspace[] = [];
        workspaces.forEach((workspace) => {
            if (
                workspace.projects?.some((value, index, array) => {
                    return value.name
                        .toLowerCase()
                        .includes(search_value.toLowerCase());
                })
            ) {
                workspaces_copy.push(structuredClone(workspace));
            }
        });
        selectable_projects = [];
        workspaces_copy.forEach((workspace) => {
            workspace.projects = workspace.projects?.filter(
                (value, index, array) => {
                    return value.name
                        .toLowerCase()
                        .includes(search_value.toLowerCase());
                }
            );
            selectable_projects = [
                ...selectable_projects,
                ...(workspace.projects ?? []),
            ];
        });
        return workspaces_copy;
    }
</script>

{#if storeLoaded}
    {#if $config.last_opened && !editable}
        {#if last_opened !== undefined}
            <p class="scroll-m-20 text-xl font-semibold tracking-tight pr-2">
                Last Opened Project
            </p>
            <div class="ps-6 pb-6">
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div
                    class="flex text-muted-foreground cursor-pointer hover:text-foreground hover:scale-[1.01] select-none"
                    class:text-muted-foreground={selected_project_id !==
                        last_opened.id}
                    class:scale-[1.01]={selected_project_id === last_opened.id}
                    on:click={() => open_project(last_opened.id)}
                >
                    <p class="w-44">{last_opened.name}</p>
                    {#if $config.project_types}
                        <p
                            style="color: {$config.project_types.find(
                                (v, i, a) => v.id === last_opened.metadata
                            )?.color};"
                        >
                            {$config.project_types.find(
                                (v, i, a) => v.id === last_opened.metadata
                            )?.name}
                        </p>
                    {/if}
                </div>
            </div>
        {/if}
    {/if}
    {#each filter_search($workspaces, search_str) as workspace}
        <div class="mb-2">
            <!-- <p>{workspace.id}; {workspace.path}</p> -->
            <div class="flex">
                <div class="grow flex">
                    <p
                        class="scroll-m-20 text-xl font-semibold tracking-tight pr-2"
                    >
                        {workspace.name}
                    </p>
                    <p class="scroll-m-20 text-xl text-muted-foreground">
                        ({workspace.path})
                    </p>
                </div>
                {#if editable}
                    <Button
                        variant="destructive"
                        size="sm"
                        class="mr-2"
                        on:click={(e) => delete_workspace(workspace.id)}
                        >Delete Workspace</Button
                    >
                    <Button
                        variant="outline"
                        size="sm"
                        on:click={(e) =>
                            reindex_workspace(workspace.id, workspace.path)}
                        >Reindex Workspace</Button
                    >
                {/if}
            </div>
            <div class="ps-6">
                {#if workspace.projects != undefined}
                    {#if workspace.projects.length != 0}
                        {#each workspace.projects as project}
                            <!-- svelte-ignore a11y-click-events-have-key-events -->
                            <div
                                class="flex text-muted-foreground cursor-pointer hover:text-foreground hover:scale-[1.01] select-none"
                                class:text-muted-foreground={selected_project_id !==
                                    project.id}
                                class:scale-[1.01]={selected_project_id ===
                                    project.id}
                                on:click={() => open_project(project.id)}
                            >
                                <p class="w-44">{project.name}</p>
                                {#if $config.project_types}
                                    <p
                                        style="color: {$config.project_types.find(
                                            (v, i, a) =>
                                                v.id === project.metadata
                                        )?.color};"
                                    >
                                        {$config.project_types.find(
                                            (v, i, a) =>
                                                v.id === project.metadata
                                        )?.name}
                                    </p>
                                {/if}
                            </div>
                        {/each}
                    {:else}
                        <p>No projects in workspace!</p>
                    {/if}
                {:else}
                    <p>No projects in workspace!</p>
                {/if}
            </div>
        </div>
    {/each}
{:else}
    <p>Fetching workspaces...</p>
{/if}
