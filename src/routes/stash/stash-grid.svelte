<script lang="ts">
	import type { Item, Profile, Option } from '../../types';
	import Grid from './grid/grid.svelte';
	import NewItemModal from './modal/modal-item.svelte';

	export let profile: Profile;
	export let onOptionClicked: (option: Option, item: Item) => void;
	let filteredItems: Array<Item | undefined>;
	$: filteredItems = [];
	let searchTerm = '';
	let isNewItemModalOpen = false;

	$: {
		if (profile) {
			filteredItems = profile.items.filter((item) => {
				return (
					profile.bsgItems[item.tpl].name.toLowerCase().includes(searchTerm.toLowerCase()) ||
					profile.bsgItems[item.tpl].shortName.toLowerCase().includes(searchTerm.toLowerCase())
				);
			});
		}
	}

	function openNewItemModal() {
		isNewItemModalOpen = true;
	}
</script>

<div class="filter">
	<input placeholder="Type to filter" bind:value={searchTerm} />
	<button on:click={openNewItemModal}>Add item</button>
</div>

{#if isNewItemModalOpen}
	<NewItemModal allItems={profile.bsgItems} onClose={() => (isNewItemModalOpen = false)} />
{/if}
<Grid
	nestedLevel={1}
	bsgItems={profile.bsgItems}
	items={filteredItems}
	sizeX={profile.sizeX}
	sizeY={profile.sizeY}
	{onOptionClicked}
/>

<style>
	.filter {
		margin: 12px 0;
	}

	.filter input {
		padding: 8px;
		background-color: var(--color-background);
		color: var(--color-text);
	}

	.filter input:focus {
		outline: none;
		border: 2px solid var(--color-highlight);
	}
</style>
