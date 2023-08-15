<script lang="ts">
    import { readDir, type FileEntry } from "@tauri-apps/api/fs";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { FileIcon, FolderIcon, Loader2Icon } from "lucide-svelte";

    export let path_str_copy: string = "";

    let path = [""];
    let cur_path = () => {
        return path.join("\\") + "\\";
    };
    $: path_str = path.join("\\") + "\\";
    let dirs: Promise<FileEntry[]> = get_dir("");
    let drives: string[] = [];
    let path_input: HTMLInputElement;

    onMount(() => {
        invoke("get_drives").then((e) => {
            drives = e as string[];
            path[0] = drives[0];
            dirs = get_dir(cur_path());
            path_str_copy = cur_path();
        });
    });

    async function get_dir(path: string) {
        return await readDir(path);
    }

    function update_dirs(dir: string | undefined) {
        if (dir == undefined) {
            return;
        }
        path = [...path, dir];
        console.log(path_str);
        dirs = get_dir(cur_path());
        path_str_copy = cur_path();
    }

    function handleDirectoryInput(
        e: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) {
        const inputValue = e.currentTarget.value;
        const lastSlashIndex = inputValue.lastIndexOf("\\");
        if (lastSlashIndex !== -1) {
            path = inputValue.slice(0, lastSlashIndex).split("\\");
        }
        dirs = get_dir(cur_path());
        path_str_copy = cur_path();
    }
</script>

<div class="w-full flex border-2 rounded-sm">
    <select
        class="cursor-pointer border-r-2 px-1 bg-background"
        on:change={(e) => {
            path = [e.currentTarget.value];
            dirs = get_dir(cur_path());
            path_str_copy = cur_path();
            path_input.focus();
        }}
    >
        {#each drives as drive}
            <option>{drive}</option>
        {/each}
    </select>
    <div class="relative flex-grow">
        <input
            type="text"
            bind:this={path_input}
            on:input={(e) => handleDirectoryInput(e)}
            class="peer w-full pl-1 bg-background"
            bind:value={path_str}
        />
        <div
            class="peer-focus:block hover:block hidden absolute top-7 w-full border rounded-sm p-1 bg-background"
        >
            {#await dirs}
                <p><Loader2Icon />Loading...</p>
            {:then entries}
                <div class="overflow-y-auto max-h-64">
                    {#each entries
                        .sort((a, b) => (a.name ?? "").toLowerCase() < (b.name ?? "").toLowerCase() ? 1 : -1)
                        .sort((a, b) => (a.children?.length ?? -1) - (b.children?.length ?? -1))
                        .reverse() as entry}
                        {#if entry.children != null}
                            <button
                                on:click={(e) => {
                                    update_dirs(entry.name);
                                    path_input.focus();
                                }}
                                class="cursor-pointer w-full border-b text-sm last:border-b-0 text-left"
                                ><FolderIcon
                                    class="inline pr-1"
                                />{entry.name}</button
                            >
                        {:else}
                            <p
                                class="w-full border-b text-sm text-muted-foreground"
                            >
                                <FileIcon class="inline pr-1" />{entry.name}
                            </p>
                        {/if}
                    {/each}
                </div>
            {:catch error}
                <p>Error: {error}</p>
            {/await}
        </div>
    </div>
</div>
