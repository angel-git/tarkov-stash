<script>
	import { page } from '$app/stores';
	import {onMount} from "svelte";

	$: errorMessage = '';
	onMount(() => {
		errorMessage = $page.url.searchParams.get('message') || 'No error from the backend :(';
	})
</script>

<div class="container container-center">
	<h1>Something went wrong:</h1>
	<p>{errorMessage}</p>
</div>
