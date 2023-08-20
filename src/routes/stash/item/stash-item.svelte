<script lang="ts">
	import type { Item, BsgItem } from '../../../types';
	export let item: Item;
	export let bsgItems: Record<string, BsgItem>;
	export let handleOpenClick: (item: Item | undefined) => void;

	function calculateBackgroundStyle(item: Item) {
		return `position: absolute; background-image: url(https://assets.tarkov.dev/${
			item.tpl
		}-base-image.png); background-size: ${item.sizeX * 64}px ${item.sizeY * 64}px;`;
	}

	function calculateSizeStyle(item: Item) {
		return `z-index: 2; position: relative; height: ${item.sizeY * 64}px; width: ${
			item.sizeX * 64
		}px; background-color: ${calculateBackgroundColor(item)}`;
	}

	function calculateBackgroundColor(item: Item) {
		switch (item.backgroundColor) {
			case 'black':
				return `rgba(0, 0, 0, 0.3)`;
			case 'blue':
				return `rgba(63, 63, 147, 0.1)`;
			case 'green':
				return `rgba(55, 119, 55, 0.1)`;
			case 'grey':
				return `rgba(50, 50, 50, 0.1)`;
			case 'orange':
				return `rgba(255, 182, 115, 0.1)`;
			case 'red':
				return `rgba(255, 0, 0, 0.1)`;
			case 'violet':
				return `rgba(155, 38, 182, 0.1)`;
			case 'yellow':
				return `rgba(246, 235, 97, 0.1)`;
			default:
				return `rgba(0, 0, 0, 0.0)`;
		}
	}
</script>

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
		{#if item.maxResource && item.maxResource !== 1}
			<div class="resource">
				{`${item.resource || item.maxResource}/${item.maxResource}`}
			</div>
		{/if}
	</div>
</div>

<style>
	.item-clickable {
		cursor: pointer;
	}

	.item-clickable:hover {
		background-color: rgba(44, 42, 42, 0.7) !important;
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

	.resource {
		font-size: 13px;
		color: orangered;
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
</style>
