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
        <div>
            <hr class="my-4" />
            <div class="grid gap-2">
                <div>
                    <p>
                        Run config name:
                    </p>
                    <input
                        class="text-muted-foreground w-56 bg-transparent border px-1 rounded"
                        bind:value={r.name}
                        on:change={(e) => onChange(r)}
                    />
                </div>
                <div>
                    <p>
                        Commands:
                    </p>
                    <Textarea placeholder={"code $PPATH\n..."} bind:value={r.commands} class="text-muted-foreground resize-none" on:change={() => onChange(r)} />
                </div>
                    <Button
                        variant="destructive"
                        size="sm"
                        class="w-max"
                        on:click={() => deleteRunConfig(r)}>Delete</Button
                    >
            </div>
        </div>
    {/each}
{/if}
