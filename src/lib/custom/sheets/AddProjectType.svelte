<script lang="ts">
    import Button from "$components/ui/button/Button.svelte";
    import { XIcon } from "lucide-svelte";
    import { config, launch_alert, save_config } from "$lib/stores/app_store";
    import ColorPicker from "svelte-awesome-color-picker";
    import Wrapper from "../color_picker/Wrapper.svelte";
    import TextInput from "../color_picker/TextInput.svelte";
    import Input from "../color_picker/Input.svelte";
    import CustomSheet from "./CustomSheet.svelte";

    let p: App.ProjectType = {
        id: $config.project_types?.length.toString() ?? "0",
        name: "",
        needed_files: [""],
        color: "#fff",
        run_config_id: "",
    };
    let error = 0;

    function submit() {
        if (p.name !== "") {
            p.needed_files = p.needed_files?.filter((v, i, a) => v !== "");
            $config.project_types = $config.project_types
                ? [...$config.project_types, p]
                : [p];
            save_config($config);
            launch_alert(
                "default",
                "Success",
                `Successfully added '${p.name}' as project type!`
            );
        } else {
            error = 1;
        }
    }

    function focusNewInput(el: HTMLInputElement) {
        if (el.value === "") el.focus();
    }
</script>

<CustomSheet
    onClose={() => {
        submit();
    }}
>
    <span slot="button">Add project type</span>
    <span slot="title">Add project type</span>
    <div>
        <p class="text-sm text-muted-foreground mb-4">
            Add a new project type that can be used to auto-detect special
            projects in workspaces.
        </p>
        <div class="flex gap-2 flex-col">
        <p>
            Name*:
            <input
                class="text-muted-foreground w-56 bg-transparent ms-2 border m-1 px-1 rounded"
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
        </p>
        {#if error === 1}
            <p class="text-destructive ms-16 mb-2">You have to set a name!</p>
        {/if}
        <ColorPicker
            bind:hex={p.color}
            label="Color:"
            components={{
                wrapper: Wrapper,
                textInput: TextInput,
                input: Input,
            }}
        />
        <p>Needs files:</p>
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
            class="ms-4 mt-1 w-2/6"
            on:click={() => {
                p.needed_files = p.needed_files
                    ? [...p.needed_files, ""]
                    : [""];
            }}>Add needed file</Button
        >
        {#if $config.run_configs}
            <p class="mt-1">
                Run Config:
                <select
                    class="cursor-pointer border bg-background rounded"
                    on:change={(e) => {
                        p.run_config_id = e.currentTarget.value;
                    }}
                >
                    <option value="">None</option>
                    {#each $config.run_configs as run_config}
                        <option value="{run_config.id}">{run_config.name}</option>
                    {/each}
                </select>
            </p>
        {/if}
        </div>
    </div>
</CustomSheet>
