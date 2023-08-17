<script lang="ts">
    import Button from "$components/ui/button/Button.svelte";
    import { Tabs } from "$components/ui/tabs";
    import TabsContent from "$components/ui/tabs/TabsContent.svelte";
    import TabsList from "$components/ui/tabs/TabsList.svelte";
    import TabsTrigger from "$components/ui/tabs/TabsTrigger.svelte";
    import Workspaces from "$lib/custom/Workspaces.svelte";
    import AddWorkspace from "$lib/custom/sheets/AddWorkspace.svelte";
    import {
        storeLoaded,
    } from "$lib/stores/app_store";
    import { ChevronLeftIcon, TrashIcon } from "lucide-svelte";
    import ProjectTypeSettings from "$lib/custom/pages/ProjectTypeSettings.svelte";
    import RunConfigSettings from "$lib/custom/pages/RunConfigSettings.svelte";
</script>

<Button variant="outline" class="p-2 mb-2" href="/"><ChevronLeftIcon /></Button>
{#if $storeLoaded}
    <Tabs value="workspaces" class="w-full">
        <TabsList class="grid w-full grid-cols-3 sticky top-2 z-40">
            <TabsTrigger value="workspaces">Workspaces</TabsTrigger>
            <TabsTrigger value="project-types">Project Types</TabsTrigger>
            <!-- <TabsTrigger value="project-languages">Project Languages</TabsTrigger> -->
            <TabsTrigger value="run-configs">Run Configs</TabsTrigger>
        </TabsList>
        <TabsContent value="workspaces" class="h-full overflow-hidden">
            <AddWorkspace />
            <hr class="my-4" />
            <Workspaces editable={true} />
        </TabsContent>
        <TabsContent value="project-types">
            <ProjectTypeSettings />
        </TabsContent>
        <!-- <TabsContent value="project-languages">
            <ProjectTypeSettings />
        </TabsContent> -->
        <TabsContent value="run-configs">
            <RunConfigSettings />
        </TabsContent>
    </Tabs>
{/if}
