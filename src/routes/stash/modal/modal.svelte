<script lang="ts">
	export let showModal: boolean;
	export let onConfirm: () => void;

	let dialog: HTMLDialogElement;

	$: if (dialog && showModal) dialog.showModal();
	$: if (dialog && !showModal) dialog.close();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
	bind:this={dialog}
	on:close={() => (showModal = false)}
	on:click|self={() => (showModal = false)}
>
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div on:click|stopPropagation>
		<slot name="header" />
		<slot />
		<div class="container container-flex">
			<button class="secondary" on:click={() => (showModal = false)}>Cancel</button>
			<button class="primary" on:click={onConfirm}>Update</button>
		</div>
	</div>
</dialog>

<style>
	dialog {
		color: var(--color-text);
		background-color: var(--color-background);
		max-width: 32em;
		border-radius: 0.2em;
		border: none;
		padding: 0;
	}

	dialog::backdrop {
		background: rgba(0, 0, 0, 0.3);
	}

	dialog > div {
		padding: 1em;
	}

	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	@keyframes zoom {
		from {
			transform: scale(0.95);
		}
		to {
			transform: scale(1);
		}
	}

	dialog[open]::backdrop {
		animation: fade 0.2s ease-out;
	}

	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	button {
		display: block;
		background-color: var(--color-background);
		color: var(--color-text);
		border: 1px solid;
		border-radius: 2px;
		cursor: pointer;
		transition-duration: 0.2s;
	}

	button.primary {
		color: var(--color-highlight);
		border-color: var(--color-highlight);
	}

	button.primary:hover {
		background-color: var(--color-highlight);
		color: var(--color-background);
	}

	button.secondary {
		color: var(--color-text);
		border-color: var(--color-text);
	}

	button.secondary:hover {
		background-color: var(--color-text);
		color: var(--color-background);
	}
</style>
