<script lang="ts">
	export let labelWrapper: HTMLLabelElement;
	export let hex: string;
	export let label: string;
	/* svelte-ignore unused-export-let */
	export let isOpen: boolean;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<label
	bind:this={labelWrapper}
	on:click|preventDefault={() => {
		/* prevent default behavior on safari */
	}}
	on:mousedown|preventDefault={() => {
		/* prevent default behavior on safari */
	}}
    class="cursor-pointer"
    class:nothing={isOpen}
>
    {label}
	<div class="container inline relative">
		<input
            class="m-0 p-0 w-1 h-1 cursor-pointer rounded"
			type="color"
			value={hex}
			on:click|preventDefault={() => {
				/* prevent default behavior on most browsers */
			}}
			on:mousedown|preventDefault={() => {
				/* prevent default behavior on safari */
			}}
			aria-haspopup="dialog"
		/>
		<div class="alpha rounded w-5 h-5 absolute bg-background top-0 left-4" />
		<div class="color rounded w-5 h-5 absolute border border-muted-foreground top-0 left-4" style="background: {hex}" />
	</div>
</label>

<style>
</style>