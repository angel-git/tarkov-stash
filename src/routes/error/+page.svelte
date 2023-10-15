<script>
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	$: errorMessage = '';
	onMount(() => {
		errorMessage = $page.url.searchParams.get('message') || 'No error from the backend :(';
	});

	function goBack() {
		goto('/');
	}
</script>

<div class="container container-center">
	<h1>Something went wrong:</h1>
	<p>{errorMessage}</p>
	<button class="primary" on:click={goBack}>Go back</button>
</div>

<style>
	button {
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
</style>
