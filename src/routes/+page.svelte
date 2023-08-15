<script lang="ts">
    import Button from "$components/ui/button/Button.svelte";
    import Workspaces from "$lib/custom/Workspaces.svelte";
    import { storeLoaded} from "$lib/stores/app_store";
    import { invoke } from "@tauri-apps/api";
    import { Settings2Icon } from "lucide-svelte";

    let search_input: HTMLInputElement;

    function kPressed(e: KeyboardEvent) {
        if (
            document.activeElement != search_input &&
            search_input != undefined
        ) {
            // search_input.value = e.key;
            search_input.focus();
        }
    }

    function kDown(e: KeyboardEvent) {
        if (e.repeat) {
            return;
        }
        if (selectable_projects.length > 1) {
            switch (e.code) {
                case "ArrowDown":
                    selected_project_index += 1;
                    break;
                case "ArrowUp":
                    selected_project_index -= 1;
                    break;
            }
            selected_project_index =
                selected_project_index < 0
                    ? selectable_projects.length - 1
                    : selected_project_index;
            selected_project_index =
                selected_project_index > selectable_projects.length - 1
                    ? 0
                    : selected_project_index;
        }
        if (search_value.length != 0 && selected_project_id.length != 0) {
            if (e.code == "Enter") {
                console.log(
                    "Open in vscode " +
                        selectable_projects[selected_project_index].name
                );
                invoke("open_project", { id: selected_project_id });
            }
        }
    }

    let selected_project_index = 0;
    $: selected_project_id =
        selectable_projects.at(selected_project_index)?.id ?? "";
    let selectable_projects: App.Project[] = [];

    $: search_value = "";
</script>

<svelte:window on:keydown={kDown} on:keypress={kPressed} />
{#if $storeLoaded}
    <div class="w-full">
        <div class="bg-background sticky top-0 z-10 pb-2 flex gap-4 items-end">
            <div class="grow">
                <p
                    class="scroll-m-20 text-2xl font-semibold tracking-tight pr-2 mb-2"
                >
                    Search Project:
                </p>
                <input
                    bind:this={search_input}
                    on:input={(e) => (search_value = search_input.value)}
                    on:keydown={(e) => {
                        if (!e.repeat) {
                            if (e.code === "ArrowUp" || e.code === "ArrowDown") {
                                e.preventDefault();
                            }
                        }
                    }}
                    type="text"
                    class="w-full pl-1 bg-background border rounded-sm"
                />
            </div>
            <div>
                <Button
                    variant="outline"
                    class="p-2"
                    href="/edit"
                    ><Settings2Icon /></Button
                >
            </div>
        </div>
        <hr class="my-4" />
        {#if search_value.length == 0}
            <Workspaces editable={false} />
        {:else}
            <Workspaces bind:search_str={search_value} bind:selectable_projects={selectable_projects} editable={false} selected_project_id={selected_project_id} />
        {/if}
    </div>
{/if}