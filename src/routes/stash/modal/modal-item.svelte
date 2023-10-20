<script lang="ts">
	import { goto } from '$app/navigation';
	import Modal from './modal.svelte';
	import type { BsgItem, Item, NewItem } from '../../../types';
	import { calculateBackgroundColor } from '../../../helper';
	import { invokeWithLoader } from '../../../helper';
	import { addNewItem } from '../../../store';

	export let onClose: () => void;
	export let allItems: Record<string, BsgItem>;
	export let grid: Array<Array<Item | undefined>>;

	let showModal = true;
	let parsedItems: Array<BsgItem>;
	let notEnoughSpaceError = false;

	$: if (!showModal) onClose();

	$: {
		if (allItems) {
			parsedItems = Object.keys(allItems)
				.map((i) => allItems[i])
				.filter((i) => i.type === 'Item')
				.filter((i) => !i.unbuyable)
				.filter(
					(i) =>
						i.name.toLowerCase().includes($addNewItem.input.toLowerCase()) ||
						i.shortName.toLowerCase().includes($addNewItem.input.toLowerCase()) ||
						$addNewItem.item?.id === i.id,
				);
		}
	}

	function handleConfirm() {
		if (!$addNewItem.item) {
			return;
		}

		const location = findNewItemLocation($addNewItem.item);
		if (!location) {
			notEnoughSpaceError = true;
			return;
		}

		showModal = false;
		invokeWithLoader<NewItem>('add_item', {
			item: {
				id: $addNewItem.item.id,
				locationX: location?.x,
				locationY: location?.y,
			},
		}).catch((e) => goto(`/error?message=${e}`));
	}

	function selectItem(item: BsgItem) {
		if (item.id === $addNewItem.item?.id) {
			addNewItem.set({ item: undefined, input: $addNewItem.input });
		} else {
			addNewItem.set({ item, input: $addNewItem.input });
		}
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
			<!-- svelte-ignore a11y-autofocus -->
			<input autofocus placeholder="Filter..." bind:value={$addNewItem.input} />
			<div>
				<ul>
					{#each parsedItems as item}
						<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
						<li
							class={item.id === $addNewItem.item?.id ? 'selected' : ''}
							on:click={() => selectItem(item)}
						>
							{item.name}
						</li>
					{/each}
				</ul>
			</div>
		</div>
		{#if $addNewItem.item}
			<div
				class="selected-item"
				style={`background-color: ${calculateBackgroundColor($addNewItem.item.backgroundColor)}`}
			>
				<div>{$addNewItem.item.shortName}</div>
				<div>{$addNewItem.item.width}X{$addNewItem.item.height}</div>
				<img alt="item" src={`https://assets.tarkov.dev/${$addNewItem.item.id}-base-image.png`} />
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
		padding: 0;
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
