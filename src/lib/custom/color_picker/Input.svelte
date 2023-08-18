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
    class:nothing={isOpen}
>
    Color:
	<br>
	<div class="container inline relative cursor-pointer">
		<input
            class="m-0 p-0 w-0 h-0 cursor-pointer rounded absolute left-2 top-2"
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
		<div class="alpha rounded w-5 h-5 absolute bg-background top-0 left-0" />
		<div class="color rounded w-5 h-5 absolute border border-muted-foreground top-0" style="background: {hex}" />
		<p class="absolute bottom-0 left-7 w-max" style="color: {hex}">{label}</p>
	</div>
</label>

<style>
</style>