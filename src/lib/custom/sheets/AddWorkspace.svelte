<script lang="ts">
    import DirChooser from "../DirChooser.svelte";
    import { add_workspace, launch_alert } from "$lib/stores/app_store";
    import CustomSheet from "./CustomSheet.svelte";
    import { PlusIcon } from "lucide-svelte";

    export let path = "";
</script>


<CustomSheet onClose={() => {add_workspace(path); return true;}}>
    <span slot="button"><PlusIcon /></span>
    <span slot="title">Add workspace</span>
    <span slot="subtitle">
        Add a new workspace and the projects it contains.
        <br>
        All <span class="underline">underlined</span> fields must be filled out.
    </span>
    <p class="underline">Path:</p>
    <DirChooser bind:path_str_copy={path} />
</CustomSheet>