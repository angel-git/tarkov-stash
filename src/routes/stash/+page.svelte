<script lang="ts">
	import type { Item, Option, Profile } from '../../types';
	import { profile } from '../../store';
	import StashGrid from './stash-grid.svelte';
	import AmountModal from './modal/modal-amount.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import { afterNavigate, goto } from '$app/navigation';

	let selectedOption: Option | undefined;
	let selectedItem: Item | undefined;

	afterNavigate(() => {
		invoke<Profile>('load_profile_file', {})
			.then((p) => {
				console.log('got new profile from backend', p);
				profile.set(p);
			})
			.catch((error) => {
				goto(`/error?message=${error}`);
			});
	});

	// let profile = $profileStore;

	function handleOptionClicked(option: Option, item: Item) {
		// document.body.style.overflow = "hidden";
		// document.body.style.height = "100%";
		selectedOption = option;
		selectedItem = item;
	}

	// function handleCloseDialog() {
	// document.body.style.overflow = "auto";
	// document.body.style.height = "auto";
	// }

	function handleCloseModal() {
		selectedOption = undefined;
		selectedItem = undefined;
	}
</script>

<div class="container container-center">
	{#if $profile}
		<h3>
			Editing <span class="highlight">{$profile.name}</span>'s stash
		</h3>
		<h4>
			Your current stash size is {$profile.sizeX}x{$profile.sizeY}
		</h4>
		{#if selectedOption && selectedOption === 'amount'}
			<AmountModal item={selectedItem} onClose={handleCloseModal} />
		{/if}
		<StashGrid profile={$profile} onOptionClicked={handleOptionClicked} />
	{:else}
		<p>loading profile</p>
	{/if}
</div>
