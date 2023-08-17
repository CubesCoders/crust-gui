<script lang="ts">
    import { Button } from "$components/ui/button";
    import { config, save_config } from "$lib/stores/app_store";
    import { TrashIcon } from "lucide-svelte";
    import AddRunConfig from "../sheets/AddRunConfig.svelte";
    import { Textarea } from "$components/ui/textarea";

    function onChange(r: App.RunConfig) {
        config.update((n) => {
            if (n.run_configs)
                n.run_configs[
                    n.run_configs.findIndex((v, i, a) => v.id === r.id) ?? -1
                ] = r;
            return n;
        });
        save_config($config);
    }

    function deleteRunConfig(r: App.RunConfig) {
        config.update((n) => {
            if (n.run_configs) {
                let index = n.run_configs.findIndex((v, i, a) => v.id === r.id) ?? -1;
                n.run_configs.splice(index, 1);
            }
            return n;
        });
        save_config($config);
    }
</script>

<AddRunConfig />
{#if $config.run_configs}
    {#each $config.run_configs as r}
        <div class="relative">
            <div class="absolute top-5 right-0">
                <Button
                    variant="destructive"
                    class="text-xs px-2"
                    on:click={() => deleteRunConfig(r)}><TrashIcon /></Button
                >
            </div>
            <hr class="my-4" />
            <p>
                Run config name:<input
                    class="text-muted-foreground w-56 bg-transparent ms-2 border m-1 px-1 rounded"
                    bind:value={r.name}
                    on:change={(e) => onChange(r)}
                />
            </p>
            <p>
                Commands:
            </p>
            <Textarea placeholder={"code $PPATH\n..."} bind:value={r.commands} class="text-muted-foreground" on:change={() => onChange(r)} />
        </div>
    {/each}
{/if}
