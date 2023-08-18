<script lang="ts">
    import DirChooser from "../DirChooser.svelte";
    import { add_workspace, launch_alert } from "$lib/stores/app_store";
    import CustomSheet from "./CustomSheet.svelte";

    export let path = "";
</script>


<CustomSheet onClose={() => {add_workspace(path); return true;}}>
    <span slot="button">Add workspace</span>
    <span slot="title">Add workspace</span>
    <p class="text-sm text-muted-foreground mb-4">Add a new workspace and the projects it contains</p>
    <DirChooser bind:path_str_copy={path} />
</CustomSheet>