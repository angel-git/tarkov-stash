<script lang="ts">
  import type { BsgItem, Item } from '../../../types';
  import { goto } from '$app/navigation';

  import Modal from './modal.svelte';
  import { invokeWithLoader } from '../../../helper';

  export let onClose: () => void;
  export let item: Item;
  export let bsgItems: Record<string, BsgItem>;

  let showModal = true;

  $: if (!showModal) onClose();

  function handleConfirm() {
    showModal = false;

    invokeWithLoader('remove_item', { item }).catch((e) => goto(`/error?message=${e}`));
  }
</script>

<Modal bind:showModal onConfirm={handleConfirm}>
  <h2 slot="header">
    Deleting item {item.id}
  </h2>

  <div>
    <h4>THIS WILL DELETE THIS ITEM AND EVERYTHING INSIDE</h4>
    {#if item.gridItems}
      {#each item.gridItems as gridItem}
        <ul>
          {#each gridItem.items as innerItem}
            <li>{bsgItems[innerItem.tpl].name}</li>
          {/each}
        </ul>
      {/each}
    {/if}
  </div>
</Modal>

<style>
  h4 {
    color: var(--color-destructive);
  }

  ul {
    padding: 0;
  }

  ul li {
    list-style-type: none;
  }
</style>
