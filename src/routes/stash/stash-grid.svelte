<script lang="ts">
	import type { Item, Profile, Option, BsgItem } from '../../types';

	export let profile: Profile;
	export let onOptionClicked: (option: Option, item: Item) => void;
	let orderedItems: Array<Item | undefined>;
	let bsgItems: { [key: string]: BsgItem };
	$: orderedItems = [];
	$: bsgItems = {};

	let itemOpenId = '-1';
	let fakeId = 'XXX';

	$: {
		if (profile) {
			const addedItems = new Set();
			const tempItems: Array<Item | undefined> = [];

			const grid = Array.from({ length: profile.sizeY }, () => Array(profile.sizeX).fill(null));

			profile.items.forEach((item) => {
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
						if (addedItems.has(item.id)) {
							tempItems.push({ ...item, id: fakeId });
						} else {
							tempItems.push(item);
							addedItems.add(item.id);
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

	function calculateBackgroundStyle(item: Item) {
		return `position: absolute; background-image: url(https://assets.tarkov.dev/${
			item.tpl
		}-base-image.png); background-size: ${item.sizeX * 64}px ${item.sizeY * 64}px`;
	}

	function calculateSizeStyle(item: Item) {
		return `z-index: 2; position: relative; height: ${item.sizeY * 64}px; width: ${
			item.sizeX * 64
		}px;`;
	}
</script>

<div class="grid">
	{#each orderedItems as item}
		<div class="grid-item">
			{#if item && item.id !== fakeId}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					tabindex="-1"
					role="button"
					class="item-clickable"
					style={calculateSizeStyle(item)}
					on:click={() => handleOpenClick(item)}
				>
					<div class="item-image" style={calculateBackgroundStyle(item)} />
					<div class="short-name">{bsgItems[item.tpl].shortName}</div>
					<div class="item-data">
						{#if item.isFir}
							<div class="fir" />
						{/if}
						{#if item.isStockable}
							<div class="amount">{item.amount}</div>
						{/if}
					</div>
				</div>
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
				</div>
			{/if}
		</div>
	{/each}
</div>

<style>
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

	.item-clickable {
		cursor: pointer;
	}

	.item-clickable:hover {
		filter: brightness(120%);
	}

	.fir {
		height: 12px;
		width: 12px;
		background-image: url($lib/images/fir.png);
		background-size: 12px 12px;
	}

	.amount {
		font-size: 13px;
		z-index: 2;
	}

	.item-image {
		width: 100%;
		height: 100%;
	}

	.item-data {
		height: 100%;
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		justify-content: flex-end;
	}

	.short-name {
		font-size: 11px;
		position: absolute;
		right: 2px;
		top: 2px;
		z-index: 2;
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
