<script lang="ts">
    import { Tooltip, TooltipProvider, TooltipTrigger } from "$components/ui/tooltip";
    import TooltipContent from "$components/ui/tooltip/TooltipContent.svelte";
    import { HelpCircleIcon } from "lucide-svelte";

</script>

<TooltipProvider>
    <Tooltip>
        <TooltipTrigger><HelpCircleIcon size=10 /></TooltipTrigger>
        <TooltipContent>
            <slot />
        </TooltipContent>
    </Tooltip>
</TooltipProvider>