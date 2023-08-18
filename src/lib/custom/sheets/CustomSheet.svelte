<script lang="ts">
    import Button from "$components/ui/button/Button.svelte";
    import { XIcon } from "lucide-svelte";
    import { portal } from "svelte-portal";

    export let onClose: () => boolean;

    let modal: HTMLDivElement;
    let modal_background: HTMLDivElement;

    function open() {
        modal_background.style.transform = `translateY(${document.documentElement.scrollTop}px)`
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


<Button variant="outline" on:click={open}><slot name="button" /></Button>


<div class="absolute top-0 left-0 w-full h-full backdrop-blur-sm bg-background/80 hidden" bind:this={modal_background} use:portal={"body"}>
    <div class="w-3/4 max-w-2xl h-screen bg-background border-e p-5 transition-transform duration-300 -translate-x-full" bind:this={modal}>
        <button class="float-right" on:click={close}><XIcon /></button>
        <p class="text-lg font-semibold">
            <slot name="title" />
        </p>
        <span class="clear-both"></span>
        <p class="text-sm text-muted-foreground mb-6">
            <slot name="subtitle" />
        </p>
        <slot />
        <hr class="mb-4 mt-6" />
        <Button type="submit" size="sm" on:click={(e) => {
            if (onClose()) close();
        }}>Add</Button>
    </div>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="grow" on:click={close}></div>
</div>