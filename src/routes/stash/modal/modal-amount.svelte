<script lang="ts">
  import type { Item } from '../../../types';
  import { goto } from '$app/navigation';
  import AmountInput from './amount-input.svelte';
  import Modal from './modal.svelte';
  import { invokeWithLoader } from '../../../helper';

  export let onClose: () => void;
  export let item: Item;

  let showModal = true;
  let amount = item.amount;
  let max = item.stackMaxSize;

  $: if (!showModal) onClose();

  function handleConfirm() {
    invokeWithLoader('change_amount', { item: { ...item, amount } })
      .catch((e) => goto(`/error?message=${e}`))
      .finally(() => {
        showModal = false;
      });
  }
</script>

<Modal bind:showModal onConfirm={handleConfirm}>
  <h2 slot="header">
    Updating item {item.id}
  </h2>
  <AmountInput bind:amount {max} />
</Modal>

<style>
</style>
