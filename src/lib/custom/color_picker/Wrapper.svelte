<script lang="ts">
    export let wrapper;
    export let isOpen: boolean;
    export let isPopup: boolean;
    export let toRight;
</script>

<div class="bg-background wrapper w-max hidden absolute top-14 p-2 border rounded z-10" bind:this={wrapper} class:isOpen class:isPopup role={isPopup ? 'dialog' : undefined} aria-label="color picker" >
    <slot />
</div>

<style>
    .isOpen {
        display: block;
    }

    
</style>