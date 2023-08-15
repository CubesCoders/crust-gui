<script lang="ts">
    export let wrapper;
    export let isOpen;
    export let isPopup;
    export let toRight;
</script>

<div class="bg-background wrapper w-max hidden absolute top-8 z-30 p-2 border rounded" bind:this={wrapper} class:isOpen class:isPopup role={isPopup ? 'dialog' : undefined} aria-label="color picker" >
    <slot />
</div>

<style>
    .isOpen {
        display: block;
    }

    
</style>