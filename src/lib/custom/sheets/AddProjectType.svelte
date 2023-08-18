<script lang="ts">
    import Button from "$components/ui/button/Button.svelte";
    import {
        config,
        get_project_type_id,
        launch_alert,
        save_config,
    } from "$lib/stores/app_store";
    import ColorPicker from "svelte-awesome-color-picker";
    import Wrapper from "../color_picker/Wrapper.svelte";
    import TextInput from "../color_picker/TextInput.svelte";
    import Input from "../color_picker/Input.svelte";
    import CustomSheet from "./CustomSheet.svelte";
    import { PlusIcon } from "lucide-svelte";
    import Helper from "../utils/Helper.svelte";

    let p: App.ProjectType = {
        id: "-1",
        name: "",
        needed_files: [""],
        color: "#ffffff",
        run_config_id: "",
    };
    let error = 0;
    set_id();

    async function set_id() {
        p.id = String(await get_project_type_id());
        console.log(p.id);
    }

    function submit() {
        if (p.id != "-1") {
            if (p.name !== "") {
                p.needed_files = p.needed_files?.filter((v, i, a) => v !== "");
                $config.project_types = $config.project_types
                    ? [...$config.project_types, p]
                    : [p];
                save_config($config);
                return true;
            } else {
                error = 1;
            }
        } else {
            error = 2;
        }
        return false;
    }

    function clear() {
        // cleanup
        p = {
            id: "-1",
            name: "",
            needed_files: [""],
            color: "#ffffff",
            run_config_id: "",
        };
        error = 0;
        set_id();
    }

    function focusNewInput(el: HTMLInputElement) {
        if (el.value === "") el.focus();
    }
</script>

<CustomSheet
    onClose={() => {
        let ok = submit();
        if (!ok) return false;
        clear();
        return true;
    }}
>
    <span slot="button"><PlusIcon /></span>
    <span slot="title">Add project type</span>
    <span slot="subtitle">
        Add a new project type that can be used to auto-detect special projects in workspaces.
        <br>
        All <span class="underline">underlined</span> fields must be filled out.
    </span>
    <div>
        <div class="grid gap-2">
            <div>
                <p class="underline">
                    Name:
                </p>
                <input
                    class="text-muted-foreground w-56 bg-transparent border px-1 rounded"
                    class:border-destructive={error === 1}
                    bind:value={p.name}
                    on:change={() => {
                        if (error === 1 && p.name !== "") {
                            error = 0;
                        } else if (error === 0 && p.name === "") {
                            error = 1;
                        }
                    }}
                />
                {#if error === 1}
                    <p class="text-destructive">You have to set a name!</p>
                {/if}
            </div>
            <ColorPicker
                bind:hex={p.color}
                label={p.name}
                components={{
                    wrapper: Wrapper,
                    textInput: TextInput,
                    input: Input,
                }}
            />
            <div>
                <p>Criteria Files: <Helper><p><span class="font-bold text-foreground">Criteria files</span> are needed for the indexer to <span class="font-bold text-foreground">automatically assign</span> this project type! The indexer looks if these <span class="font-bold text-foreground">files (or directorys)</span> are <span class="font-bold text-foreground">matching</span>.</p></Helper></p>
                {#if p.needed_files}
                    {#each p.needed_files as file}
                        <p class="ps-4 text-muted-foreground">
                            - <input
                                class="text-muted-foreground w-56 bg-transparent ms-1 mt-1 border px-1 rounded"
                                bind:value={file}
                                use:focusNewInput
                            />
                        </p>
                    {/each}
                {/if}
                <Button
                    variant="outline"
                    size="sm"
                    class="mt-1 w-max"
                    on:click={() => {
                        p.needed_files = p.needed_files
                            ? [...p.needed_files, ""]
                            : [""];
                    }}>Add criteria file</Button
                >
            </div>
            {#if $config.run_configs}
                <p class="mt-1">
                    Run Config: <Helper><p>Here you can define a <span class="font-bold text-foreground">run configuration</span>. This configuration will be <span class="font-bold text-foreground">executed</span> when you select a <span class="font-bold text-foreground">project with this project type</span>.</p></Helper>
                </p>
                <select
                    class="cursor-pointer border bg-background rounded w-2/4"
                    on:change={(e) => {
                        p.run_config_id = e.currentTarget.value;
                    }}
                >
                    <option value="">None</option>
                    {#each $config.run_configs as run_config}
                        <option value={run_config.id}
                            >{run_config.name}</option
                        >
                    {/each}
                </select>
            {/if}
        </div>
    </div>
</CustomSheet>
