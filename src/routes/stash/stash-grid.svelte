<script lang="ts">
	import type { Item, Profile, Option, BsgItem } from '../../types';
	import { ITEMS_TEMPLATE_UPDATABLE } from './helper';

	export let profile: Profile;
	export let onOptionClicked: (option: Option, item: Item) => void;
	let orderedItems: Array<Item | undefined>;
	let bsgItems: { [key: string]: BsgItem };
	$: orderedItems = [];
	$: bsgItems = {};

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

<div class="grid">
	{#each orderedItems as item}
		<div class="grid-item">
			{#if item}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					tabindex="-1"
					role="button"
					class={`item-${item.tpl} item-clickable`}
					on:click={() => handleOpenClick(item)}
				>
					<div class="item-info">
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
					{#if ITEMS_TEMPLATE_UPDATABLE.includes(item.tpl)}
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

	.fir {
		position: absolute;
		top: 2px;
		left: 2px;
		height: 10px;
		width: 10px;
		background-image: url($lib/images/fir.png);
		background-size: 10px 10px;
	}

	.item-info {
		width: 100%;
		height: 100%;
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
		user-select: none;
	}

	.options {
		position: absolute;
		bottom: -30px;
		right: 0;
		background-color: var(--color-background);
		border: 1px solid var(--color-text);
		font-size: 12px;
		z-index: 2;
		min-width: 100px;
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
