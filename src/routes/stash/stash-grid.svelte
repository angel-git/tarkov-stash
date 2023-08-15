<script lang="ts">
	import type { Item, Profile, Option } from '../../types';

	export let profile: Profile;
	export let onOptionClicked: (option: Option, item: Item) => void;
	let orderedItems: Array<Item | undefined>;
	$: orderedItems = [];

	let itemOpenId = '-1';

	$: {
		if (profile) {
			const tempItems: Array<Item | undefined> = [];
			for (let col = 0; col < profile.sizeY; col++) {
				for (let row = 0; row < profile.sizeX; row++) {
					const maybeItem = profile.items.find((item: Item) => item.x === row && item.y === col);
					tempItems.push(maybeItem);
				}
			}
			orderedItems = [...tempItems];
		}
	}


	function handleOpenClick(item: Item) {
		// TODO check if we allow to open menu for this item
		if (itemOpenId === item.id) {
			itemOpenId = '-1';
		} else {
			itemOpenId = item.id;
		}
	}

	function handleOptionClicked(option: Option, item: Item) {
		itemOpenId = '-1';
		onOptionClicked(option, item);
	}
</script>

<div class="grid">
	{#each orderedItems as item}
		<div class="grid-item">
			{#if item}
				<div
					role="button"
					class={`item-${item.tpl} item-clickable`}
					on:click={() => handleOpenClick(item)}
				>
					<div class="amount">{item.isStockable ? item.amount : ''}</div>
				</div>
			{:else}
				<div class="empty" />
			{/if}
			{#if item?.id === itemOpenId}
				<div class="options">
					<div class="option" on:click={() => handleOptionClicked('amount', item)}>
						Change amount
					</div>
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
		background-image: url($lib/images/unknown.png);
	}

	.item-5449016a4bdc2d6f028b456f {
		background-image: url($lib/images/5449016a4bdc2d6f028b456f.png);
	}

	.item-569668774bdc2da2298b4568 {
		background-image: url($lib/images/569668774bdc2da2298b4568.png);
	}

	.item-5696686a4bdc2da3298b456a {
		background-image: url($lib/images/5696686a4bdc2da3298b456a.png);
	}

	.item-clickable {
		height: 64px;
		width: 64px;
		cursor: pointer;
	}

	.empty {
		height: 64px;
		width: 64px;
		background-image: url($lib/images/empty.png);
	}

	.amount {
		position: absolute;
		bottom: 2px;
		right: 2px;
		font-size: 13px;
	}

	.options {
		position: absolute;
		bottom: -30px;
		right: 0;
		background-color: var(--color-background);
		border: 1px solid var(--color-text);
		font-size: 12px;
		z-index: 2;
		width: 100px;
	}

	.options .option {
		padding: 8px 4px;
	}

	.options .option:hover {
		cursor: pointer;
		color: var(--color-highlight);
	}
</style>
