<script lang="ts">
  import type { Item } from '../../../types';
  import { goto } from '$app/navigation';

  import Modal from './modal.svelte';
  import { getName, invokeWithLoader } from '../../../helper';

  export let onClose: () => void;
  export let item: Item;
  export let locale: Record<string, string>;

  let showModal = true;

  $: if (!showModal) onClose();

  function handleConfirm() {
    invokeWithLoader('remove_item', { item })
      .catch((e) => goto(`/error?message=${e}`))
      .finally(() => {
        showModal = false;
      });
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
            <li>{getName(innerItem.tpl, locale)}</li>
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
