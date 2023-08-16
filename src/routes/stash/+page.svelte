<script lang="ts">
	import type { Item, Option, Profile } from '../../types';
	import { profile } from '../../store';
	import StashGrid from './stash-grid.svelte';
	import AmountModal from './modal/modal-amount.svelte';
	import { invoke } from '@tauri-apps/api';
	import { afterNavigate, goto } from '$app/navigation';

	let selectedOption: Option | undefined;
	let selectedItem: Item | undefined;

	afterNavigate(() => {
		invoke<Profile>('load_profile_file', {})
			.then((p) => {
				profile.set(p);
			})
			.catch((error) => {
				goto(`/error?message=${error}`);
			});
	});

	function handleOptionClicked(option: Option, item: Item) {
		if (option === 'fir') {
			invoke('change_fir', { item }).catch((e) => goto(`/error?message=${e}`));
			handleCloseModal();
		} else {
			selectedOption = option;
			selectedItem = item;
		}
	}

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
		{#if selectedItem && selectedOption && selectedOption === 'amount'}
			<AmountModal item={selectedItem} onClose={handleCloseModal} />
		{/if}
		<StashGrid profile={$profile} onOptionClicked={handleOptionClicked} />
	{:else}
		<p>loading profile</p>
	{/if}
</div>
