<script lang="ts">
    import Textarea from "$components/ui/textarea/Textarea.svelte";
    import { config, launch_alert, save_config } from "$lib/stores/app_store";
    import { PlusIcon } from "lucide-svelte";
    import CustomSheet from "./CustomSheet.svelte";
    import Helper from "../utils/Helper.svelte";

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
    <span slot="subtitle">
        Add a new run configuration that can be used to run projects specifically.
        <br>
        All <span class="underline">underlined</span> fields must be filled out.
    </span>
    <div class="grid gap-2">
        <div>
            <p class="underline">
                Name:
            </p>
            <input
                class="text-muted-foreground w-56 bg-transparent border px-1 rounded"
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
            {#if error === 1}
                <p class="text-destructive mb-2">You have to set a name!</p>
            {/if}
        </div>
        <div>
            <p class="underline">
                Commands: <Helper><p><span class="font-bold text-foreground">Commands</span> will be executed when <span class="font-bold text-foreground">running</span> a selected <span class="font-bold text-foreground">project</span>. Each new <span class="font-bold text-foreground">line</span> will be executed <span class="font-bold text-foreground">seperatly</span> as a new <span class="font-bold text-foreground">command</span>. <span class="bg-accent rounded-sm px-1 font-bold text-foreground">$PPATH</span> will be replaced with the <span class="font-bold text-foreground">project path</span>.</p></Helper>
            </p>
            <Textarea placeholder={"code $PPATH\n..."} bind:value={r.commands} />
        </div>
    </div>
</CustomSheet>