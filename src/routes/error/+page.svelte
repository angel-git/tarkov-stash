<script>
	import { page } from '$app/stores'
	const errorMessage = $page.url.searchParams.get('message')
</script>

<div class="container container-center">
	<h1>Something went wrong:</h1>
	<p>{errorMessage}</p>
</div>
