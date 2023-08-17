<script lang="ts">
    import { Button } from "$components/ui/button";
    import { config, save_config } from "$lib/stores/app_store";
    import { TrashIcon } from "lucide-svelte";
    import ColorPicker from "svelte-awesome-color-picker";
    import AddProjectType from "../sheets/AddProjectType.svelte";
    import Wrapper from "../color_picker/Wrapper.svelte";
    import Input from "../color_picker/Input.svelte";
    import TextInput from "../color_picker/TextInput.svelte";



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
                        
                    {#if $config.run_configs}
                    <p class="mt-1">
                        Run Config:
                        <select
                            class="cursor-pointer border bg-background rounded"
                            on:change={(e) => {
                                p.run_config_id = e.currentTarget.value;
                                onChange(p)
                            }}
                            value={p.run_config_id ?? ""}
                        >
                            <option value="">None</option>
                            {#each $config.run_configs as run_config}
                                <option value="{run_config.id}">{run_config.name}</option>
                            {/each}
                        </select>
                    </p>
                {/if}
                    </div>
                {/each}
            {/if}