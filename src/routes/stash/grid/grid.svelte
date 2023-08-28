<script lang="ts">
	import type { BsgItem, Item, Option } from '../../../types';
	import StashItem from '../item/stash-item.svelte';
	import Grid from './grid.svelte';

	export let items: Array<Item>;
	export let sizeY: number;
	export let sizeX: number;
	export let nestedLevel: number;
	export let bsgItems: Record<string, BsgItem>;
	export let onOptionClicked: (option: Option, item: Item) => void;

	let secondaryItemMenuId = '-1';
	let containerOpenId = '-1';
	let orderedItems: Array<Item | undefined>;
	$: orderedItems = [];

	$: {
		if (items) {
			const addedItems = new Set();
			const tempItems: Array<Item | undefined> = [];

			const grid = Array.from({ length: sizeY }, () => Array(sizeX).fill(null));

			items.forEach((item) => {
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

			for (let col = 0; col < sizeY; col++) {
				for (let row = 0; row < sizeX; row++) {
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
		}
	}

	function handleOpenClick(item: Item | undefined) {
		if (!item) return;
		if (secondaryItemMenuId === item.id) {
			secondaryItemMenuId = '-1';
		} else {
			secondaryItemMenuId = item.id;
		}
	}

	function handleOptionClicked(option: Option, item: Item | undefined) {
		if (!item) return;
		secondaryItemMenuId = '-1';
		if (option === 'open') {
			containerOpenId = item.id;
		} else {
			onOptionClicked(option, item);
		}
	}
</script>

<div class="grid" style={`grid-template-columns: repeat(${sizeX}, 64px); width: ${sizeX * 64}px;`}>
	{#each orderedItems as item}
		<div class="grid-item">
			{#if item}
				<StashItem {bsgItems} {handleOpenClick} {item} />
			{:else}
				<div class="empty" />
			{/if}
			{#if item?.id === secondaryItemMenuId}
				<div class="options">
					<div class="title">{bsgItems[item.tpl].name}</div>
					{#if item.isContainer}
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<div
							class="option"
							tabindex="-1"
							role="button"
							on:click={() => handleOptionClicked('open', item)}
						>
							Open
						</div>
					{/if}
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
			{#if item?.id === containerOpenId}
				<div class="nested-grid" style={`z-index: ${nestedLevel + 10}`}>
					<div class="nested-grid-header">
						<div>{bsgItems[item.tpl].name}</div>
						<button on:click={() => (containerOpenId = '-1')}>close</button>
					</div>
					{#each item.gridItems as gridItem}
						<Grid
							nestedLevel={nestedLevel + 1}
							{bsgItems}
							items={gridItem.items}
							sizeX={gridItem.cellsH}
							sizeY={gridItem.cellsV}
							{onOptionClicked}
						/>
					{/each}
				</div>
			{/if}
		</div>
	{/each}
</div>

<style>
	.grid {
		display: grid;
		margin: 0 auto;
	}

	.grid-item {
		height: 64px;
		width: 64px;
		position: relative;
		background-image: url($lib/images/empty.png);
	}

	.nested-grid {
		border: 2px solid var(--color-background);
		background-color: var(--color-background);
		position: absolute;
		top: 0;
	}

	.nested-grid-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		background-color: rgba(44, 42, 42, 0.7);
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
		background-color: rgba(44, 42, 42, 0.7);
		color: var(--color-highlight);
	}
</style>