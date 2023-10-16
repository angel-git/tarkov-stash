<script lang="ts">
	import { goto } from '$app/navigation';
	import Modal from './modal.svelte';
	import type { BsgItem, Item, NewItem } from '../../../types';
	import { calculateBackgroundColor } from '../../../helper';
	import { invokeWithLoader } from '../../../helper';

	export let onClose: () => void;
	export let allItems: Record<string, BsgItem>;
	export let grid: Array<Array<Item | undefined>>;

	let showModal = true;
	let parsedItems: Array<BsgItem>;
	let selectedItem: BsgItem | undefined;
	let notEnoughSpaceError = false;

	let filter = '';

	$: if (!showModal) onClose();

	$: {
		if (allItems) {
			parsedItems = Object.keys(allItems)
				.map((i) => allItems[i])
				.filter((i) => i.type === 'Item')
				.filter((i) => !i.unlootable)
				.filter((i) => !i.unbuyable)
				.filter(
					(i) =>
						i.name.toLowerCase().includes(filter.toLowerCase()) ||
						i.shortName.toLowerCase().includes(filter.toLowerCase()),
				);
		}
	}

	function handleConfirm() {
		if (!selectedItem) {
			return;
		}

		const location = findNewItemLocation(selectedItem);
		if (!location) {
			notEnoughSpaceError = true;
			return;
		}

		showModal = false;
		invokeWithLoader<NewItem>('add_item', {
			item: {
				id: selectedItem.id,
				locationX: location?.x,
				locationY: location?.y,
			},
		}).catch((e) => goto(`/error?message=${e}`));
	}

	function selectItem(item: BsgItem) {
		console.log('selectItem', item);
		selectedItem = item;
	}

	function findNewItemLocation(item: BsgItem) {
		const { width, height } = item;

		const sizeY = grid.length;
		const sizeX = grid[0].length;

		for (let row = 0; row <= sizeY - height; row++) {
			for (let col = 0; col <= sizeX - width; col++) {
				let hasSpace = true;
				for (let i = 0; i < height && hasSpace; i++) {
					for (let j = 0; j < width; j++) {
						if (grid[row + i][col + j]) {
							hasSpace = false;
							break;
						}
					}
				}

				if (hasSpace) {
					return { x: col, y: row };
				}
			}
		}
		return null;
	}
</script>

<Modal bind:showModal onConfirm={handleConfirm}>
	{#if notEnoughSpaceError}
		<h3>You don't have enough space for this item</h3>
	{/if}
	<h2 slot="header">Add item into stash <strong>(BETA!)</strong></h2>

	<div>
		<div>
			<input placeholder="Filter..." bind:value={filter} />
			<div>
				<ul>
					{#each parsedItems as item}
						<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
						<li
							class={item.id === selectedItem?.id ? 'selected' : ''}
							on:click={() => selectItem(item)}
						>
							{item.name}
						</li>
					{/each}
				</ul>
			</div>
		</div>
		{#if selectedItem}
			<div
				class="selected-item"
				style={`background-color: ${calculateBackgroundColor(selectedItem.backgroundColor)}`}
			>
				<div>{selectedItem.shortName}</div>
				<div>{selectedItem.width}X{selectedItem.height}</div>
				<img alt="item" src={`https://assets.tarkov.dev/${selectedItem.id}-base-image.png`} />
			</div>
		{/if}
	</div>
</Modal>

<style>
	h3 {
		color: orangered;
	}
	input {
		padding: 8px;
		background-color: var(--color-background);
		color: var(--color-text);
	}

	input:focus {
		outline: none;
		border: 2px solid var(--color-highlight);
	}
	ul {
		max-height: 300px;
		overflow-y: auto;
	}

	li {
		list-style-type: none;
		cursor: pointer;
		margin: 8px 0;
	}

	li.selected {
		color: var(--color-highlight);
	}

	li:hover {
		color: var(--color-highlight);
	}

	.selected-item {
		margin: 16px;
		padding: 8px;
	}
</style>
