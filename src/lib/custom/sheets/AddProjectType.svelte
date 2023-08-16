<script lang="ts">
    import Button from "$components/ui/button/Button.svelte";
    import { XIcon } from "lucide-svelte";
    import { config, launch_alert, save_config } from "$lib/stores/app_store";
    import ColorPicker from "svelte-awesome-color-picker";
    import Wrapper from "../color_picker/Wrapper.svelte";
    import TextInput from "../color_picker/TextInput.svelte";
    import Input from "../color_picker/Input.svelte";

    let modal: HTMLDivElement;
    let modal_background: HTMLDivElement;
    let p: App.ProjectType = { id: $config.project_types?.length.toString() ?? "0", name: "", needed_files: [""], color: "#fff" };
    let error = 0;

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

    function submit() {
        if (p.name !== "") {
            p.needed_files = p.needed_files?.filter((v, i, a) => v !== "");
            $config.project_types = $config.project_types ? [...$config.project_types, p] : [p];
            save_config($config);
            close();
            launch_alert("default", "Success", `Successfully added '${p.name}' as project type!`);
        } else {
            error = 1;
        }
    }

    function focusNewInput(el: HTMLInputElement) {
        if (el.value === "") el.focus();
    }
</script>


<Button variant="outline" on:click={open}>Add project type</Button>


<div class="absolute top-0 left-0 w-full h-full backdrop-filter backdrop-blur-sm bg-background/80 hidden z-40" bind:this={modal_background}>
    <div class="w-3/4 max-w-sm h-screen bg-background border-e p-5 transition-transform duration-300 -translate-x-full" bind:this={modal}>
        <button class="float-right" on:click={close}><XIcon /></button>
        <p class="text-lg font-semibold">Add Project Type</p>
        <span class="clear-both"></span>
        <p class="text-sm text-muted-foreground mb-4">Add a new project type that can be used to auto-detect special projects in workspaces.</p>
        <p>Name*:
            <input class="text-muted-foreground w-56 bg-transparent ms-2 border m-1 px-1 rounded" class:border-destructive={error === 1} bind:value={p.name} on:change={() => {if (error === 1 && p.name !== "") {error = 0} else if (error === 0 && p.name === "") {error = 1}}}/>
        </p>
        {#if error === 1}
            <p class="text-destructive ms-16 mb-2">You have to set a name!</p>
        {/if}
        <ColorPicker bind:hex={p.color} label="Color:" components={{wrapper: Wrapper, textInput: TextInput, input: Input}} />
        <p>Needs files:</p>
        {#if p.needed_files}
            {#each p.needed_files as file}
                <p class="ps-4 text-muted-foreground">
                    - <input class="text-muted-foreground w-56 bg-transparent ms-1 mt-1 border px-1 rounded" bind:value={file} use:focusNewInput >
                </p>
            {/each}
        {/if}
        <Button variant="outline" size="sm" class="ms-4 mt-1" on:click={() => {p.needed_files = p.needed_files ? [...p.needed_files, ""] : [""]}}
            >Add needed file</Button
        >
        <br>
        <Button type="submit" size="sm" class="mt-4" on:click={(e) => submit()}>Add</Button>
    </div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="grow" on:click={close}></div>
</div>