<script lang="ts">
	// import type { Item } from '../../../types';
	// import { goto } from '$app/navigation';

	import Modal from './modal.svelte';
	import type { BsgItem } from '../../../types';
	import { calculateBackgroundColor } from '../../../helper';
	// import { invokeWithLoader } from '../../../helper';

	export let onClose: () => void;
	export let allItems: Record<string, BsgItem>;

	let showModal = true;
	let parsedItems: Array<BsgItem>;
	let selectedItem: BsgItem | undefined;

	let filter = '';

	$: if (!showModal) onClose();

	$: {
		if (allItems) {
			parsedItems = Object.keys(allItems)
				.map((i) => allItems[i])
				.filter((i) => i.type === 'Item')
				.filter((i) => !i.unlootable)
				.filter(
					(i) =>
						i.name.toLowerCase().includes(filter.toLowerCase()) ||
						i.shortName.toLowerCase().includes(filter.toLowerCase()),
				);
		}
	}

	function handleConfirm() {
		showModal = false;

		// invokeWithLoader('change_amount', { item: { ...item, amount } }).catch((e) =>
		// 	goto(`/error?message=${e}`),
		// );
	}

	function selectItem(item: BsgItem) {
		console.log('selectItem', item);
		selectedItem = item;
	}
</script>

<Modal bind:showModal onConfirm={handleConfirm}>
	<h2 slot="header">Add item into stash</h2>

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
		height: 400px;
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
