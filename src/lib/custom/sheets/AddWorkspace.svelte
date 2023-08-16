<script lang="ts">
    import Button from "$components/ui/button/Button.svelte";
    import { XIcon } from "lucide-svelte";
    import DirChooser from "../DirChooser.svelte";
    import { add_workspace, launch_alert } from "$lib/stores/app_store";

    export let path = "";

    let modal: HTMLDivElement;
    let modal_background: HTMLDivElement;

    function open() {
        document.documentElement.classList.add('overflow-hidden');
        modal_background.classList.replace('hidden', 'flex');
        setTimeout(() => {
            modal.classList.remove('-translate-x-full');
        });
    }

    function close() {
        document.documentElement.classList.remove('overflow-hidden');
        modal.classList.add('-translate-x-full');
        modal_background.classList.replace('flex', 'hidden');
    }
</script>


<Button variant="outline" on:click={open}>Add Workspace</Button>


<div class="absolute top-0 left-0 w-full h-full backdrop-filter backdrop-blur-sm bg-background/80 hidden z-50" bind:this={modal_background}>
    <div class="w-3/4 max-w-sm h-screen bg-background border-e p-5 transition-transform duration-300 -translate-x-full" bind:this={modal}>
        <button class="float-right" on:click={close}><XIcon /></button>
        <p class="text-lg font-semibold">Add Workspace</p>
        <span class="clear-both"></span>
        <p class="text-sm text-muted-foreground mb-4">Add a new workspace and the projects it contains</p>
        <DirChooser bind:path_str_copy={path} />
        <Button type="submit" size="sm" class="mt-4" on:click={(e) => {add_workspace(path); close(); launch_alert("default", "Success", `Successfully added '${path}' to workspaces!`)}}>Add</Button>
    </div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="grow" on:click={close}></div>
</div>