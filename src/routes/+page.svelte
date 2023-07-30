<script lang="ts">
	// import { profile } from '../store';
	import { listen } from '@tauri-apps/api/event';
	import { goto } from '$app/navigation';
	// import { invoke } from '@tauri-apps/api';
	// import type { Profile } from '../types';

	listen('profile_loaded', () => {
		console.log('profile_loaded event received');

		goto('/stash', { invalidateAll: true });

		// invoke<Profile>('load_profile_file', {})
		// 	.then((p) => {
		// 		console.log('got new profile from backend', p);
		// 		profile.set(p);
		// 		goto('/stash');
		// 	})
		// 	.catch((error) => {
		// 		goto(`/error?message=${error}`);
		// 	});
	});
	listen('error', (event) => {
		goto(`/error?message=${event.payload}`);
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Tarkov Stash" />
</svelte:head>

<div class="container container-center">
	<h1>
		Welcome to <span class="highlight">tarkov-stash</span> mod
	</h1>
	<h4>Select your profile from the menu to start editing it</h4>
</div>
