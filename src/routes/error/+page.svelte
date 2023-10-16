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
</style>
