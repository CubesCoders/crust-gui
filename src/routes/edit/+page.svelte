<script lang="ts">
    import Button from "$components/ui/button/Button.svelte";
    import { Tabs } from "$components/ui/tabs";
    import TabsContent from "$components/ui/tabs/TabsContent.svelte";
    import TabsList from "$components/ui/tabs/TabsList.svelte";
    import TabsTrigger from "$components/ui/tabs/TabsTrigger.svelte";
    import Workspaces from "$lib/custom/Workspaces.svelte";
    import AddWorkspace from "$lib/custom/sheets/AddWorkspace.svelte";
    import ColorPicker from 'svelte-awesome-color-picker';
    import {
        config,
        storeLoaded,
        save_config,
    } from "$lib/stores/app_store";
    import { ChevronLeftIcon, TrashIcon } from "lucide-svelte";
    import Wrapper from "$lib/custom/color_picker/Wrapper.svelte";
    import Input from "$lib/custom/color_picker/Input.svelte";
    import TextInput from "$lib/custom/color_picker/TextInput.svelte";
    import AddProjectType from "$lib/custom/sheets/AddProjectType.svelte";

    function onChange(p: App.ProjectType) {
        config.update((n) => {
            if (p.needed_files) p.needed_files = p.needed_files.filter((v, i, a) => v !== "");
            if (n.project_types)
                n.project_types[
                    n.project_types.findIndex((v, i, a) => v.id === p.id) ?? -1
                ] = p;
            return n;
        });
        save_config($config);
    }

    function deleteProjectType(p: App.ProjectType) {
        config.update((n) => {
            if (n.project_types) {
                let index = n.project_types.findIndex((v, i, a) => v.id === p.id) ?? -1;
                n.project_types.splice(index, 1);
            }
            return n;
        });
        save_config($config);
    }

    function focusNewInput(el: HTMLInputElement) {
        if (el.value === "") el.focus();
    }
</script>

<Button variant="outline" class="p-2 mb-2" href="/"><ChevronLeftIcon /></Button>
{#if $storeLoaded}
    <Tabs value="workspaces" class="w-full">
        <TabsList class="grid w-full grid-cols-2 sticky top-2 z-40">
            <TabsTrigger value="workspaces">Workspaces</TabsTrigger>
            <TabsTrigger value="project-types">Project Types</TabsTrigger>
        </TabsList>
        <TabsContent value="workspaces" class="h-full overflow-hidden">
            <AddWorkspace />
            <hr class="my-4" />
            <Workspaces editable={true} />
        </TabsContent>
        <TabsContent value="project-types">
            <AddProjectType />
            {#if $config.project_types}
                {#each $config.project_types as p, i}
                    <div class="relative">
                        <div class="absolute top-5 right-0">
                            <Button variant="destructive" class="text-xs px-2" on:click={() => deleteProjectType(p)}><TrashIcon /></Button>
                        </div>
                        <hr class="my-4" />
                        <p>
                            Project type name:<input
                                class="text-muted-foreground w-56 bg-transparent ms-2 border m-1 px-1 rounded"
                                bind:value={p.name}
                                on:change={(e) => onChange(p)}
                            />
                        </p>
                        <div>
                            <ColorPicker bind:hex={p.color} on:input={(e) => onChange(p)} label="Color:" components={{wrapper: Wrapper, textInput: TextInput, input: Input}} />
                        </div>
                        <p class="text-muted-foreground" />
                        <p>Needs files:</p>
                        {#if p.needed_files}
                            {#each p.needed_files as file}
                                <p class="ps-4 text-muted-foreground">
                                    - <input class="text-muted-foreground w-56 bg-transparent ms-1 mt-1 border px-1 rounded" bind:value={file} use:focusNewInput on:change={(e) => onChange(p)} >
                                </p>
                            {/each}
                        {/if}
                        <Button variant="outline" size="sm" class="ms-4 mt-1" on:click={() => {p.needed_files = p.needed_files ? [...p.needed_files, ""] : [""]}}
                            >Add needed file</Button
                        >
                    </div>
                {/each}
            {/if}
        </TabsContent>
    </Tabs>
{/if}
