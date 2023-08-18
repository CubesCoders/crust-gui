<script lang="ts">
    import Textarea from "$components/ui/textarea/Textarea.svelte";
    import { config, launch_alert, save_config } from "$lib/stores/app_store";
    import { PlusIcon } from "lucide-svelte";
    import CustomSheet from "./CustomSheet.svelte";

    let r: App.RunConfig = {
        id: $config.run_configs?.length.toString() ?? "0",
        name: "",
        commands: "",
    };
    let error = 0;

    function submit() {
        if (r.name !== "") {
            $config.run_configs = $config.run_configs
                ? [...$config.run_configs, r]
                : [r];
            save_config($config);
            return true;
        } else {
            error = 1;
        }
        return false;
    }
</script>


<CustomSheet onClose={() => {return submit()}}>
    <span slot="button"><PlusIcon /></span>
    <span slot="title">Add run configuration</span>
    <div>
        <p class="text-sm text-muted-foreground mb-4">
            Add a new run configuration that can be used to run projects specifically. <br>Use <span class="bg-accent rounded-sm px-1 text-foreground">$PPATH</span> to be replaced with the project path. 
        </p>
        <p class="underline">
            Name:
            <input
                class="text-muted-foreground w-56 bg-transparent ms-2 border m-1 px-1 rounded"
                class:border-destructive={error === 1}
                bind:value={r.name}
                on:change={() => {
                    if (error === 1 && r.name !== "") {
                        error = 0;
                    } else if (error === 0 && r.name === "") {
                        error = 1;
                    }
                }}
            />
        </p>
        {#if error === 1}
            <p class="text-destructive ms-16 mb-2">You have to set a name!</p>
        {/if}
        <p class="underline">
            Commands:
        </p>
        <Textarea placeholder={"code $PPATH\n..."} bind:value={r.commands} />
    </div>
</CustomSheet>