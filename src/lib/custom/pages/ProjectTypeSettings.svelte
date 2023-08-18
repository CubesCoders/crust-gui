<script lang="ts">
    import { Button } from "$components/ui/button";
    import { config, is_default_pt, save_config } from "$lib/stores/app_store";
    import { TrashIcon } from "lucide-svelte";
    import ColorPicker from "svelte-awesome-color-picker";
    import AddProjectType from "../sheets/AddProjectType.svelte";
    import Wrapper from "../color_picker/Wrapper.svelte";
    import Input from "../color_picker/Input.svelte";
    import TextInput from "../color_picker/TextInput.svelte";
    import Helper from "../utils/Helper.svelte";

    function onChange(p: App.ProjectType) {
        config.update((n) => {
            if (p.needed_files)
                p.needed_files = p.needed_files.filter((v, i, a) => v !== "");
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
                let index =
                    n.project_types.findIndex((v, i, a) => v.id === p.id) ?? -1;
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
            <hr class="my-4" />
            <div class="flex gap-4">
                <div class="grow grid gap-2">
                    <div>
                        <p>
                            Project type name:
                        </p>
                        <input
                            class="text-muted-foreground w-11/12 bg-transparent border px-1 rounded"
                            bind:value={p.name}
                            on:change={(e) => onChange(p)}
                        />
                    </div>

                    <div>
                        <ColorPicker
                            bind:hex={p.color}
                            on:input={(e) => onChange(p)}
                            label={p.name}
                            components={{
                                wrapper: Wrapper,
                                textInput: TextInput,
                                input: Input,
                            }}
                        />
                    </div>

                    {#if $config.run_configs}
                        <p class="mt-1">
                            Run Config:
                            <select
                                class="cursor-pointer border bg-background rounded"
                                on:change={(e) => {
                                    p.run_config_id = e.currentTarget.value;
                                    onChange(p);
                                }}
                                value={p.run_config_id ?? ""}
                            >
                                <option value="">None</option>
                                {#each $config.run_configs as run_config}
                                    <option value={run_config.id}
                                        >{run_config.name}</option
                                    >
                                {/each}
                            </select>
                        </p>
                    {/if}

                    {#if !is_default_pt(p.id)}
                        <Button
                                variant="destructive"
                                size="sm"
                                class="w-max"
                                on:click={() => deleteProjectType(p)}
                                >Delete</Button
                            >
                    {/if}
                </div>
                <div class="grow grid gap-2">
                    <div>
                        <p>Criteria Files:</p>
                        <div class="grid gap-1">
                            {#if p.needed_files}
                                {#each p.needed_files as file}
                                    <p class="ps-4 text-muted-foreground">
                                        - <input
                                            class="text-muted-foreground w-11/12 bg-transparent ms-1 mt-1 border px-1 rounded"
                                            bind:value={file}
                                            use:focusNewInput
                                            on:change={(e) => onChange(p)}
                                            disabled={is_default_pt(p.id)}
                                        />
                                    </p>
                                {/each}
                            {/if}
                        </div>
                    </div>
                    {#if !is_default_pt(p.id)}
                        <Button
                            variant="outline"
                            size="sm"
                            class="w-max"
                            on:click={() => {
                                p.needed_files = p.needed_files
                                    ? [...p.needed_files, ""]
                                    : [""];
                            }}>Add criteria file</Button
                        >
                    {/if}
                </div>
            </div>
        </div>
    {/each}
{/if}
