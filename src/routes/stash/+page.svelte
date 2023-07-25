<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import StashGrid from './stash-grid.svelte';
	import type {Profile} from "../../types";

	let profile = undefined;
	onMount(async () => {
		try {
			profile = await invoke<Profile>('load_profile_file', {});
		} catch (e) {
			goto(`/error?message=${e}`);
		}
	});
</script>

<div class="container container-center">
	{#if profile}
		<h3>
			Editing <span class="highlight">{profile.name}</span>'s stash
		</h3>
		<h4>
			Your current stash size is {profile.sizeX}x{profile.sizeY}
		</h4>
		<StashGrid profile={profile} />
	{:else}
		<p>loading profile</p>
	{/if}
</div>
