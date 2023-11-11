<script lang="ts">
  import type { Item } from '../../../types';
  import { goto } from '$app/navigation';

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

  function handleKeyUp() {
    if (amount < 1) {
      amount = 1;
    } else if (amount > max) {
      amount = max;
    }
  }

  function setAmountToMax() {
    amount = max || 1;
  }
</script>

<Modal bind:showModal onConfirm={handleConfirm}>
  <h2 slot="header">
    Updating item {item.id}
  </h2>

  <div>
    <input type="number" on:keyup={handleKeyUp} bind:value={amount} min="1" {max} />
    <button on:click={setAmountToMax} disabled={amount === max}>Set to max</button>
  </div>
</Modal>

<style>
  input {
    background-color: var(--color-background);
    color: var(--color-text);
  }

  button {
    background-color: var(--color-background);
    color: var(--color-secondary);
    border: 1px solid;
    border-color: var(--color-secondary);
    border-radius: 2px;
    cursor: pointer;
    transition-duration: 0.2s;
  }

  button:hover {
    background-color: var(--color-secondary);
    color: var(--color-background);
  }

  button:disabled,
  button:disabled:hover {
    cursor: default;
    opacity: 0.2;
    background-color: var(--color-background);
    color: var(--color-secondary);
  }
</style>
