<script lang="ts">
	import type { Item } from '../../../types';
	import { invoke } from '@tauri-apps/api';
	import { goto } from '$app/navigation';

	import Modal from './modal.svelte';

	export let onClose: () => void;
	export let item: Item;

	let showModal = true;
	let amount = item.amount;

	$: if (!showModal) onClose();

	function handleConfirm() {
		showModal = false;

		invoke('change_amount', { item: { ...item, amount } }).catch((e) =>
			goto(`/error?message=${e}`)
		);
	}

	function handleKeyUp() {
		if (amount < 1) {
			amount = 1;
		} else if (amount > 500000) {
			amount = 500000;
		}
	}
</script>

<Modal bind:showModal onConfirm={handleConfirm}>
	<h2 slot="header">
		Updating item {item.id}
	</h2>

	<div>
		<input type="number" on:keyup={handleKeyUp} bind:value={amount} min="1" max="500000" />
	</div>
</Modal>
