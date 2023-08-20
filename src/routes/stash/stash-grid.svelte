<script lang="ts">
	import type { Item, Profile, Option, BsgItem } from '../../types';
	import StashItem from './item/stash-item.svelte';

	export let profile: Profile;
	export let onOptionClicked: (option: Option, item: Item) => void;
	let orderedItems: Array<Item | undefined>;
	let bsgItems: Record<string, BsgItem>;
	$: orderedItems = [];
	$: bsgItems = {};

	let itemOpenId = '-1';
	let searchTerm = '';

	$: {
		if (profile) {
			const addedItems = new Set();
			const tempItems: Array<Item | undefined> = [];

			const grid = Array.from({ length: profile.sizeY }, () => Array(profile.sizeX).fill(null));

			profile.items
				.filter((item) => {
					return (
						profile.bsgItems[item.tpl].name.toLowerCase().includes(searchTerm.toLowerCase()) ||
						profile.bsgItems[item.tpl].shortName.toLowerCase().includes(searchTerm.toLowerCase())
					);
				})
				.forEach((item) => {
					const rotatedItem =
						item.rotation === 'Vertical'
							? {
									...item,
									sizeX: item.sizeY,
									sizeY: item.sizeX,
							  }
							: item;
					for (let col = rotatedItem.y; col < rotatedItem.y + rotatedItem.sizeY; col++) {
						for (let row = rotatedItem.x; row < rotatedItem.x + rotatedItem.sizeX; row++) {
							grid[col][row] = rotatedItem;
						}
					}
				});

			for (let col = 0; col < profile.sizeY; col++) {
				for (let row = 0; row < profile.sizeX; row++) {
					const item = grid[col][row];
					if (item) {
						if (!addedItems.has(item.id)) {
							tempItems.push(item);
							addedItems.add(item.id);
						} else {
							tempItems.push(undefined);
						}
					} else {
						tempItems.push(undefined);
					}
				}
			}

			orderedItems = [...tempItems];
			bsgItems = profile.bsgItems;
		}
	}

	function handleOpenClick(item: Item | undefined) {
		if (!item) return;
		if (itemOpenId === item.id) {
			itemOpenId = '-1';
		} else {
			itemOpenId = item.id;
		}
	}

	function handleOptionClicked(option: Option, item: Item | undefined) {
		if (!item) return;
		itemOpenId = '-1';
		onOptionClicked(option, item);
	}
</script>

<div class="filter">
	<input placeholder="Type to filter" bind:value={searchTerm} />
</div>

<div class="grid">
	{#each orderedItems as item}
		<div class="grid-item">
			{#if item}
				<StashItem {bsgItems} {handleOpenClick} {item} />
			{:else}
				<div class="empty" />
			{/if}
			{#if item?.id === itemOpenId}
				<div class="options">
					<div class="title">{bsgItems[item.tpl].name}</div>
					{#if item.isStockable}
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<div
							class="option"
							tabindex="-1"
							role="button"
							on:click={() => handleOptionClicked('amount', item)}
						>
							Change amount
						</div>
					{/if}
					{#if !item.isFir}
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<div
							class="option"
							tabindex="-1"
							role="button"
							on:click={() => handleOptionClicked('fir', item)}
						>
							Set fir
						</div>
					{/if}
					{#if item.maxResource && item.maxResource !== 1 && item.resource !== item.maxResource}
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<div
							class="option"
							tabindex="-1"
							role="button"
							on:click={() => handleOptionClicked('resource', item)}
						>
							Restore durability
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{/each}
</div>

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

	.grid {
		display: grid;
		grid-template-columns: repeat(10, 64px);
		width: 640px;
		margin: 0 auto;
	}

	.grid-item {
		height: 64px;
		width: 64px;
		position: relative;
		background-image: url($lib/images/empty.png);
	}

	.empty {
		height: 64px;
		width: 64px;
	}

	.options {
		position: absolute;
		top: 0;
		left: 0;
		background-color: var(--color-background);
		border: 1px solid var(--color-text);
		font-size: 12px;
		z-index: 5;
		min-width: 120px;
	}

	.options .option {
		padding: 8px 4px;
	}

	.options .title {
		font-weight: bold;
		border-bottom: 1px solid var(--color-text);
	}

	.options .option:hover {
		cursor: pointer;
		color: var(--color-highlight);
	}
</style>
